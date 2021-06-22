pub use html_escape;
pub use r_html_codegen::template;

pub trait ToHtml<'a> {
    type Out: AsRef<str> + 'a;
    fn to_html(&'a self) -> Self::Out;
}

impl<'a> ToHtml<'a> for String {
    type Out = &'a str;

    fn to_html(&'a self) -> Self::Out {
        self
    }
}

impl<'a> ToHtml<'a> for &str {
    type Out = &'a str;

    fn to_html(&'a self) -> &'a str {
        self
    }
}

macro_rules! to_html_from_to_strings {
    ($($t: ty),+) => {
        $(
            to_html_from_to_string!($t);
        )*
    }
}

macro_rules! to_html_from_to_string {
    ($t: ty) => {
        impl<'a> ToHtml<'a> for $t {
            type Out = String;

            fn to_html(&'a self) -> Self::Out {
                self.to_string()
            }
        }
    };
}

to_html_from_to_strings!(u8, u16, u32, u64, u128, usize);
to_html_from_to_strings!(i8, i16, i32, i64, i128, isize);
