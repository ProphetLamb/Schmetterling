extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};

use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

use crate::components::{card, doc, proj};

impl Display for proj::Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "proj={}", self.value)
    }
}

impl Display for doc::Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&doc={}", self.proj, self.value)
    }
}

impl Display for card::Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&card={}", self.doc, self.value)
    }
}

fn parse_proj_id(input: &str) -> IResult<&str, proj::Id> {
    let (input, _) = tag("proj=")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, proj::Id { value: id }))
}

fn parse_doc_id(input: &str) -> IResult<&str, doc::Id> {
    let (input, proj) = parse_proj_id(input)?;
    let (input, _) = tag("&doc=")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, doc::Id { value: id, proj }))
}

fn parse_card_id(input: &str) -> IResult<&str, card::Id> {
    let (input, doc) = parse_doc_id(input)?;
    let (input, _) = tag("&card=")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, card::Id { value: id, doc }))
}

impl From<u64> for proj::Id {
    fn from(value: u64) -> Self {
        proj::Id { value }
    }
}

impl From<doc::Id> for u64 {
    fn from(val: doc::Id) -> Self {
        val.value
    }
}

impl From<card::Id> for u64 {
    fn from(val: card::Id) -> Self {
        val.value
    }
}

impl Serialize for proj::Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Serialize for doc::Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Serialize for card::Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for proj::Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(ProjIdVisitor {})
    }
}

impl<'de> Deserialize<'de> for doc::Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(DocIdVisitor {})
    }
}

impl<'de> Deserialize<'de> for card::Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(CardIdVisitor {})
    }
}

struct ProjIdVisitor {}

impl<'de> Visitor<'de> for ProjIdVisitor {
    type Value = proj::Id;

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
    type Value = doc::Id;

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
    type Value = card::Id;

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
