extern crate nom;
use nom::{bytes::complete::take_while_m_n, combinator::map_res, IResult};
use paste::paste;

use std::{fmt::Display, str::FromStr};

use serde::{de::Visitor, Deserialize, Serialize};

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn from_hex8(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn parse_hex8(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(8, 8, is_hex_digit), from_hex8)(input)
}

#[macro_export]
macro_rules! ord_by {
    ($type:tt, $field:ident) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.$field.partial_cmp(&other.$field)
            }
        }
    };
}

macro_rules! hier_id {
    ($name:ident: $ntype:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $ntype {
            pub value: u32,
        }
        ord_by!($ntype, value);

        impl $ntype {
            pub fn new(value: u32) -> Self {
                Self { value }
            }
        }

        impl From<$ntype> for String {
            fn from(val: $ntype) -> Self {
                format!("{:08X}", val.value)
            }
        }

        paste! { fn [<parse_ $name>](input: &str) -> IResult<&str, $ntype> {
            let (input, value) = parse_hex8(input)?;
            Ok((input, $ntype { value }))
        } }

        impl FromStr for $ntype {
            type Err = String;

            paste! {fn from_str(s: &str) -> Result<Self, Self::Err> {
                match [<parse_ $name>](s) {
                    Ok((_, p)) => Ok(p),
                    Err(e) => Err(e.to_string()),
                }
            } }
        }

        impl From<$ntype> for u32 {
            fn from($name: $ntype) -> Self {
                $name.value
            }
        }
    };
    ($name:ident: $ntype:ident, $parent:ident: $ptype:ty) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $ntype {
            pub value: u32,
            pub $parent: $ptype,
        }
        ord_by!($ntype, value);

        impl $ntype {
            pub fn new(value: u32, $parent: $ptype) -> Self {
                Self { value, $parent }
            }
        }

        impl From<$ntype> for String {
            fn from(val: $ntype) -> Self {
                format!("{}{:08X}", val.$parent, val.value)
            }
        }

        paste! { fn [<parse_ $name>] (input: &str) -> IResult<&str, $ntype> {
            let (input, $parent) = [<parse_ $parent>](input)?;
            let (input, value) = parse_hex8(input)?;
            Ok((input, $ntype { value, $parent }))
        } }

        impl FromStr for $ntype {
            type Err = String;

            paste! {fn from_str(s: &str) -> Result<Self, Self::Err> {
                match [<parse_ $name>](s) {
                    Ok((_, p)) => Ok(p),
                    Err(e) => Err(e.to_string()),
                }
            } }
        }
    };
}

hier_id!(proj: Proj);
hier_id!(doc: Doc, proj: Proj);
hier_id!(sec: Sec, doc: Doc);

macro_rules! serde_str {
    ($type:ty) => {
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s: String = self.clone().into();
                write!(f, "{}", s)
            }
        }

        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let s: String = self.clone().into();
                serializer.serialize_str(&s)
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
