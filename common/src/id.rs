extern crate nom;
use nom::{bytes::complete::take_while_m_n, combinator::map_res, IResult};
use paste::paste;

use std::{fmt::Display, str::FromStr};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Proj {
    pub value: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Doc {
    pub proj: Proj,
    pub value: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sec {
    pub value: u32,
    pub doc: Doc,
}

impl Display for Proj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "proj={}", self.value)
    }
}

impl Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&doc={}", self.proj, self.value)
    }
}

impl Display for Sec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&card={}", self.doc, self.value)
    }
}

impl From<Proj> for String {
    fn from(val: Proj) -> Self {
        format!("{:08X}", val.value)
    }
}

impl From<Doc> for String {
    fn from(val: Doc) -> Self {
        format!("{}{:08X}", val.proj, val.value)
    }
}

impl From<Sec> for String {
    fn from(val: Sec) -> Self {
        format!("{}{:08X}", val.doc, val.value)
    }
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn from_hex8(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn parse_hex8(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(8, 8, is_hex_digit), from_hex8)(input)
}

fn parse_proj(input: &str) -> IResult<&str, Proj> {
    let (input, value) = parse_hex8(input)?;
    Ok((input, Proj { value }))
}

fn parse_doc(input: &str) -> IResult<&str, Doc> {
    let (input, proj) = parse_proj(input)?;
    let (input, value) = parse_hex8(input)?;
    Ok((input, Doc { value, proj }))
}

fn parse_sec(input: &str) -> IResult<&str, Sec> {
    let (input, doc) = parse_doc(input)?;
    let (input, value) = parse_hex8(input)?;
    Ok((input, Sec { value, doc }))
}

impl FromStr for Proj {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_proj(s) {
            Ok((_, p)) => Ok(p),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl FromStr for Doc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_doc(s) {
            Ok((_, p)) => Ok(p),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl FromStr for Sec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_sec(s) {
            Ok((_, p)) => Ok(p),
            Err(e) => Err(e.to_string()),
        }
    }
}

macro_rules! serde_str {
    ($type:ty) => {
        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                paste! {deserializer.deserialize_string( [<$type Visitor>] {})}
            }
        }

        paste! {struct [<$type Visitor>] {}}

        impl<'de> Visitor<'de> for paste! {[<$type Visitor>]} {
            type Value = $type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match str::parse(v) {
                    Ok(id) => Ok(id),
                    Err(_) => Err(serde::de::Error::custom("invalid format")),
                }
            }
        }
    };
}

serde_str!(Sec);
serde_str!(Doc);
serde_str!(Proj);
