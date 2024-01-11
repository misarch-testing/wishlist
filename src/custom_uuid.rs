use std::{ops::Deref, str::FromStr};

use async_graphql::{connection::CursorType, SimpleObject, InputObject};
use serde::{de, Deserialize, Serialize};

#[derive(Debug, Hash, Eq, PartialEq, Clone, SimpleObject, InputObject)]
pub struct Uuid {
    value: uuid::Uuid,
}

impl Uuid {
    pub fn new_v4() -> Self {
        Self {
            value: uuid::Uuid::new_v4(),
        }
    }
}

impl Deref for Uuid {
    type Target = bson::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromStr for Uuid {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = uuid::Uuid::parse_str(s)?;
        Ok(Self { value: value })
    }
}

struct UuidVisitor;

impl<'de> de::Visitor<'de> for UuidVisitor {
    type Value = Uuid;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A valid human-readable UUID.")
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        s.parse().map_err(de::Error::custom)
    }
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.as_hyphenated().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UuidVisitor)
    }
}

impl CursorType for Uuid {
    type Error = uuid::Error;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        let value = uuid::Uuid::decode_cursor(s)?;
        Ok(Uuid { value: value })
    }

    fn encode_cursor(&self) -> String {
        uuid::Uuid::encode_cursor(&self.value)
    }
}
