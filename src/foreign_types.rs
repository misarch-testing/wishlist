use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, hash::Hash};
use uuid::Uuid;

use crate::uuid_serde::serialize_uuid;

/// Foreign type of a user.
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, SimpleObject)]
#[graphql(unresolvable)]
pub struct User {
    /// UUID of the user.
    #[serde(serialize_with = "serialize_uuid")]
    pub id: Uuid,
}

/// Foreign type of a product variant.
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, SimpleObject)]
#[graphql(unresolvable)]
pub struct ProductVariant {
    /// UUID of the product variant.
    #[serde(serialize_with = "serialize_uuid")]
    pub id: Uuid,
}

impl PartialOrd for ProductVariant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
