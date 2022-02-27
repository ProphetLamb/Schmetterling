use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_a, is_not, tag, take_while, take_while_m_n},
    character::{
        complete::{alpha1, alphanumeric1, anychar, char, digit0, multispace1},
        streaming::anychar,
    },
    combinator::{all_consuming, map, map_opt, map_res, opt, recognize, value, verify},
    error::{FromExternalError, ParseError},
    multi::{fold_many0, many0_count},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use yew::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Text {
    lines: String,
}

impl Text {
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn parse(&self) -> Vec<Symbol> {
        Default::default()
    }
}

pub struct Tokenizer<'s> {
    text: &'s str,
}

impl<'s> Iterator for Tokenizer<'s> {
    type Item = Symbol<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        None
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line<'s> {
    pub id: usize,
    pub text: Vec<Symbol<'s>>,
}

impl<'s> Line<'s> {
    pub fn number(&self) -> usize {
        self.id + 1
    }

    pub fn gutter_id(&self) -> String {
        format!("L{}", self.number())
    }

    pub fn code_id(&self) -> String {
        format!("LC{}", self.number())
    }

    pub fn to_dom(&self) -> Html {
        html!(<></>)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol<'a> {
    Quote(Quote<'a>),
    Trivia(&'a str),
    Ident(&'a str),
}

fn parse_str<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        char('"'),
        verify(is_not("\\\""), |s: &str| !s.is_empty()),
        char('"'),
    )(i)
}

fn parse_chars<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        char('\''),
        verify(is_not("\\\'"), |s: &str| !s.is_empty()),
        char('\''),
    )(i)
}

fn parse_tick<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        char('`'),
        verify(is_not("\\`"), |s: &str| !s.is_empty()),
        char('`'),
    )(i)
}

fn parse_multi_tick<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        tag("```"),
        verify(is_not("\\```"), |s: &str| !s.is_empty()),
        tag("```"),
    )(i)
}

fn parse_gt_lt<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        tag("<<"),
        verify(is_not("\\>>"), |s: &str| !s.is_empty()),
        tag(">>"),
    )(i)
}

fn parse_guillemets<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        char('«'),
        verify(is_not("\\»"), |s: &str| !s.is_empty()),
        char('»'),
    )(i)
}

pub enum Quote<'a> {
    Chars(&'a str),
    GtLt(&'a str),
    Guillements(&'a str),
    MultiTick(&'a str),
    Str(&'a str),
    Tick(&'a str),
}

impl<'a> Quote<'a> {
    fn as_str(&self) -> &str {
        match self {
            Quote::Chars(s) => s,
            Quote::GtLt(s) => s,
            Quote::Guillements(s) => s,
            Quote::MultiTick(s) => s,
            Quote::Str(s) => s,
            Quote::Tick(s) => s,
        }
    }
}

fn parse_quote<'a, E>(i: &'a str) -> IResult<&'a str, Quote<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_chars, Quote::Chars),
        map(parse_gt_lt, Quote::GtLt),
        map(parse_guillemets, Quote::Guillements),
        map(parse_multi_tick, Quote::MultiTick),
        map(parse_str, Quote::Str),
        map(parse_tick, Quote::Tick),
    ))(i)
}

fn parse_trivia<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_string() {
        parse_str::<()>("\"string literal\"").expect("ok: string literal");
        parse_str::<()>("\"string literal with escaped \\\".\"")
            .expect("ok: string literal with escape");
        parse_str::<()>("\"\"").expect("ok: empty string literal");
        parse_str::<()>("").expect_err("err: empty");
        parse_str::<()>(" \"").expect("err: not starts with quote");
        parse_str::<()>("\"no end").expect_err("err: unterminated");
    }
}
