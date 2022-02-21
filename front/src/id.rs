extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};

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

fn parse_proj_id(input: &str) -> IResult<&str, Proj> {
    let (input, _) = tag("proj=")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, Proj { value: id }))
}

fn parse_doc_id(input: &str) -> IResult<&str, Doc> {
    let (input, proj) = parse_proj_id(input)?;
    let (input, _) = tag("&doc=")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, Doc { value: id, proj }))
}

fn parse_card_id(input: &str) -> IResult<&str, Sec> {
    let (input, doc) = parse_doc_id(input)?;
    let (input, _) = tag("&card=")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, Sec { value: id, doc }))
}

impl From<u32> for Proj {
    fn from(value: u32) -> Self {
        Proj { value }
    }
}

impl From<Doc> for u32 {
    fn from(val: Doc) -> Self {
        val.value
    }
}

impl From<Sec> for u32 {
    fn from(val: Sec) -> Self {
        val.value
    }
}

impl FromStr for Proj {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_proj_id(s) {
            Ok((_, p)) => Ok(p),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl FromStr for Doc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_doc_id(s) {
            Ok((_, p)) => Ok(p),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl FromStr for Sec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_card_id(s) {
            Ok((_, p)) => Ok(p),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Serialize for Proj {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Serialize for Doc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Serialize for Sec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for Proj {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(ProjIdVisitor {})
    }
}

impl<'de> Deserialize<'de> for Doc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(DocIdVisitor {})
    }
}

impl<'de> Deserialize<'de> for Sec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(CardIdVisitor {})
    }
}

struct ProjIdVisitor {}

impl<'de> Visitor<'de> for ProjIdVisitor {
    type Value = Proj;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match parse_proj_id(v) {
            Ok((_, id)) => Ok(id),
            Err(_) => Err(serde::de::Error::custom("invalid format")),
        }
    }
}

struct DocIdVisitor {}

impl<'de> Visitor<'de> for DocIdVisitor {
    type Value = Doc;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match parse_doc_id(v) {
            Ok((_, id)) => Ok(id),
            Err(_) => Err(serde::de::Error::custom("invalid format")),
        }
    }
}

struct CardIdVisitor {}

impl<'de> Visitor<'de> for CardIdVisitor {
    type Value = Sec;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match parse_card_id(v) {
            Ok((_, id)) => Ok(id),
            Err(_) => Err(serde::de::Error::custom("invalid format")),
        }
    }
}
