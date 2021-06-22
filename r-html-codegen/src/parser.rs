use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{char, multispace0};
use nom::combinator::{all_consuming, map, map_res};
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, tuple};
use nom::{FindSubstring, IResult, InputTake};
use std::str::from_utf8;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParsePart {
    Args(Vec<String>),
    Html(String),
    RsValue(String),
    RsControl(String),
}

pub fn parse_template(template: &str) -> Vec<ParsePart> {
    parse_nom(template.as_bytes())
        .expect("Parsing went wrong")
        .1
}

fn parse_nom(input: &[u8]) -> IResult<&[u8], Vec<ParsePart>> {
    all_consuming(map(
        tuple((parse_args, parse_body)),
        |(args, mut body_parts)| {
            body_parts.insert(0, ParsePart::Args(args));
            body_parts
        },
    ))(input)
}

fn parse_args(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    delimited(
        tag("<!--args "),
        separated_list0(
            delimited(multispace0, char(','), multispace0),
            map(
                map_res(take_while(|c| c != b',' && c != b'-'), from_utf8),
                String::from,
            ),
        ),
        tag("-->"),
    )(input)
}

fn parse_body(input: &[u8]) -> IResult<&[u8], Vec<ParsePart>> {
    many0(parse_body_part)(input)
}

fn parse_body_part(input: &[u8]) -> IResult<&[u8], ParsePart> {
    alt((
        map_res(
            delimited(tag("<!--rs"), take_until("-->"), tag("-->")),
            |i: &[u8]| Ok::<_, FromUtf8Error>(ParsePart::RsControl(String::from_utf8(i.to_vec())?)),
        ),
        map_res(
            delimited(tag("[rs"), take_until("]"), tag("]")),
            |i: &[u8]| Ok::<_, FromUtf8Error>(ParsePart::RsValue(String::from_utf8(i.to_vec())?)),
        ),
        map_res(
            take_until1_either_dont_fail("[rs", "<!--rs"),
            |i: &[u8]| Ok::<_, FromUtf8Error>(ParsePart::Html(String::from_utf8(i.to_vec())?)),
        ),
    ))(input)
}

pub fn take_until1_either_dont_fail(
    tag: &str,
    other: &str,
) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    let t = tag.to_owned();
    let other = other.to_owned();
    move |i: &[u8]| {
        let t_found = i.find_substring(t.as_bytes());
        let other_found = i.find_substring(other.as_bytes());

        let shorter = match t_found {
            None => other_found,
            Some(found) => Some(usize::min(found, other_found.unwrap_or(usize::MAX))),
        };

        let res = Ok(match shorter {
            None => i.take_split(i.len()),
            Some(index) => i.take_split(index),
        })?;

        if res.1.is_empty() {
            Err(nom::Err::Error(Error {
                input: i,
                code: ErrorKind::NonEmpty,
            }))
        } else {
            Ok(res)
        }
    }
}
