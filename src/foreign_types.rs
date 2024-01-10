use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::custom_uuid::Uuid;

/// Foreign type of a user.
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, SimpleObject)]
#[graphql(unresolvable)]
pub struct User {
    /// UUID of the user.
    pub id: Uuid,
}

/// Foreign type of a product variant.
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, SimpleObject)]
#[graphql(unresolvable)]
pub struct ProductVariant {
    /// UUID of the product variant.
    pub id: Uuid,
}
