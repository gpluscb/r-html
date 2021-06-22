use crate::parser::{parse_template, ParsePart};
use proc_macro::TokenStream;
use std::fs;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::ExprLit;
use syn::Lit;
use syn::__private::ToTokens;
use syn::{parse_macro_input, Expr};

mod parser;

struct ArgsInfo {
    template_location: String,
    template_args: Vec<Expr>,
}

impl Parse for ArgsInfo {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let mut list = Punctuated::<Expr, Comma>::parse_separated_nonempty(input)?.into_iter();

        let first: Expr = list.next().unwrap();
        let template_location = match first {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }) => lit.value(),
            _ => {
                panic!("First thing needs to be string lit");
            }
        };

        let template_args = list.collect();

        Ok(ArgsInfo {
            template_location,
            template_args,
        })
    }
}

fn code_from_part(part: ParsePart) -> String {
    /* Comment not super accurate
    out.push_str("some_html_literal");
    some literal rust control {
    out.push_str("another html literal");
    out.push_str(some rust value);
    some literal rust control }
    out.push_str("another html literal");
     */

    match part {
        ParsePart::Args(_) => panic!("Args variant, shouldn't happen really"),
        ParsePart::Html(html) => format!("out.push_str(r####\"{}\"####);\n", html),
        ParsePart::RsValue(value) => format!("out.push_str(AsRef::<str>::as_ref(&{}));\n", value),
        ParsePart::RsControl(mut control) => {
            control.push('\n');
            control
        }
    }
}

#[proc_macro]
pub fn template(tokens: TokenStream) -> TokenStream {
    let info = parse_macro_input!(tokens as ArgsInfo);

    let input_string = fs::read_to_string(info.template_location)
        .expect("File doesn't exist or something like that");

    let parts = parse_template(&input_string);

    codegen(parts, info.template_args)
}

fn codegen(parts: Vec<ParsePart>, args: Vec<Expr>) -> TokenStream {
    /* Comment not super accurate
    {
        fn _template(arg1: arg1type, arg2: arg2type) -> String {
            let mut out = String::new();
            out.push_str("some_html_literal");
            some literal rust control {
            out.push_str("another html literal");
            out.push_str(some rust value);
            some literal rust control }
            out.push_str("another html literal");
        }

        _template(expr1, expr2);
    }
    */
    let mut iter = parts.into_iter();

    let function_args = match iter.next().unwrap() {
        ParsePart::Args(args) => args,
        _ => panic!("No args in template??"),
    };
    let mut string = String::new();
    string.push_str("{\n");
    string.push_str("fn _template(");
    string.push_str(&function_args.join(","));
    string.push_str(") -> String {\n");

    string.push_str("let mut out = String::new();\n");
    string.push_str(&iter.map(code_from_part).collect::<String>());

    string.push_str("out\n");

    string.push_str("}\n");

    string.push_str("_template(");
    string.push_str(
        &args
            .into_iter()
            .map(|expr| format!("{}, ", expr.into_token_stream()))
            .collect::<String>(),
    );
    string.push_str(")\n");
    string.push('}');

    string.parse().expect("Codegen failed")
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_template;

    #[test]
    pub fn test() {
        println!(
            "{:?}",
            parse_template(include_str!("../../templates/template_example.rs.html"))
        );
    }
}
