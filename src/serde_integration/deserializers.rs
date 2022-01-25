use iso8601::DateTime;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

pub fn deserialize_one_zero_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    match u8::deserialize(deserializer) {
        Ok(a) => {
            match a {
                0 => Ok(true),
                1 => Ok(false),
                _ => Err(serde::de::Error::invalid_value(Unexpected::Unsigned(u64::from(a)),&"expected 0 or 1"))
            }
        },
        Err(b) => {
            Err(serde::de::Error::custom(b))
        }
    }
}

pub fn deserialize_iso8601<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
    use std::str::FromStr;
    match String::deserialize(deserializer) {
        Ok(a) => {
            match DateTime::from_str(a.as_str()) {
                Ok(a) => { Ok(a) }
                Err(b) => { Err(D::Error::custom(b)) }
            }
        }
        Err(b) => {
            Err(D::Error::custom(b))
        }
    }
}