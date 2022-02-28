use std::str::FromStr;

use self::{Symbol::*, Trivia::*};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while, take_while1},
    character::complete::{anychar, char},
    combinator::{map, verify},
    error::{FromExternalError, ParseError},
    sequence::{delimited, preceded},
    IResult,
};
use paste::paste;
use unicode_categories::{self, UnicodeCategories};
#[cfg(feature = "yew-wasm")]
use yew::prelude::*;

#[cfg(feature = "yew-wasm")]
pub trait ToDom {
    /// Retrieves the com representation.
    fn to_dom(self) -> Html;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Text<'a> {
    pub symbols: Vec<Symbol<'a>>,
}

pub struct SymbolIter<'s> {
    text: &'s str,
}

impl<'s> Iterator for SymbolIter<'s> {
    type Item = Symbol<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }
        match parse_symbol::<()>(self.text) {
            Ok((text, symbol)) => {
                self.text = text;
                Some(symbol)
            }
            Err(_) => {
                debug_assert!(false, "unhandled pattern, assuming trivia.");
                None
            }
        }
    }
}

pub struct LineIter<'s> {
    symbols: &'s [Symbol<'s>],
    id: usize,
}

impl<'s> Iterator for LineIter<'s> {
    type Item = Line<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let symbols = self.symbols;
        if symbols.is_empty() {
            return None;
        }

        let mut pos = 0;
        for symbol in self.symbols {
            if matches!(symbol, Trivia(symbol) if matches!(symbol, Line(_))) {
                break;
            }
            pos += 1;
        }
        let symbols = self.symbols;
        self.symbols = &symbols[pos..];
        let id = self.id;
        self.id = id + 1;

        let text = &symbols[..pos];
        Some(Line { id, text })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line<'s> {
    pub id: usize,
    pub text: &'s [Symbol<'s>],
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
}

#[cfg(feature = "yew-wasm")]
impl<'s> ToDom for Line<'s> {
    fn to_dom(self) -> Html {
        html! {
        <>
            {for self.text.iter().map(|symbol| symbol.to_dom())}
        </>
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol<'a> {
    Quote(&'a str),
    Trivia(Trivia<'a>),
    Ident(&'a str),
}

impl<'a> Symbol<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Quote(value) => *value,
            Trivia(value) => value.as_str(),
            Ident(value) => *value,
        }
    }
}

#[cfg(feature = "yew-wasm")]
impl<'a> ToDom for Symbol<'a> {
    fn to_dom(self) -> Html {
        match self {
            Quote(value) => html! {
                <span class="source-quote">{value}</span>
            },
            Trivia(value) => html! {
                <span class="source-trivia">{value.as_str()}</span>
            },
            Ident(value) => html! {
                <span class="source-ident">{value}</span>
            },
        }
    }
}

fn escape_literal<'a, Error: ParseError<&'a str>>(
    start: &'a str,
    end: &'a str,
    escaped: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, Error> {
    delimited(
        tag(start),
        verify(is_not(escaped), |s: &str| !s.is_empty()),
        tag(end),
    )
}

fn parse_quote<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        escape_literal("\"", "\"", "\\\""),
        escape_literal("'", "'", "\\'"),
        escape_literal("<<", ">>", "\\>>"),
        escape_literal(">>", "<<", "\\<<"),
        escape_literal("›", "‹", "\\‹"),
        escape_literal("»", "«", "\\«"),
        escape_literal("‹", "›", "\\›"),
        escape_literal("«", "»", "\\»"),
    ))(i)
}

fn parse_new_line<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((tag("\r\n"), tag("\n"), tag("\u{2028}")))(i)
}

fn parse_space<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    take_while1(|c: char| c.is_separator_paragraph() || c.is_separator_space())(i)
}

fn parse_punctuation<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    take_while1(|c: char| c.is_punctuation() || c.is_symbol())(i)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trivia<'a> {
    Space(&'a str),
    Line(&'a str),
    Symbol(&'a str),
}

impl<'a> Trivia<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Space(s) => s,
            Line(s) => s,
            Symbol(s) => s,
        }
    }
}

fn parse_trivia<'a, E>(i: &'a str) -> IResult<&'a str, Trivia<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_new_line, Line),
        map(parse_space, Space),
        map(parse_punctuation, Symbol),
    ))(i)
}

fn parse_ident<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    // starts with at least one text, follows with multiple text, numbers, or dash.
    preceded(
        take_while1(|c: char| c.is_letter() || c.is_mark()),
        take_while(|c: char| {
            c.is_letter() || c.is_mark() || c.is_punctuation_dash() || c.is_number()
        }),
    )(i)
}

/// parses any recognized symbol.
fn parse_symbol<'a, E>(i: &'a str) -> IResult<&'a str, Symbol<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_trivia, Trivia),
        map(parse_quote, Quote),
        map(parse_ident, Ident),
    ))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_parse {
        ($match:tt, $tailing:tt, $($parse:tt)*) => {
            assert_eq!($($parse)*(&format!("{}{}",$match, $tailing)), Ok(($tailing, $match)));
        };
    }
}
