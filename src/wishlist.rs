use std::collections::HashSet;

use async_graphql::{
    connection::{Edge, EmptyFields},
    SimpleObject,
};
use bson::datetime::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Wishlist {
    pub _id: String,
    pub user_id: String,
    pub product_variant_ids: HashSet<String>,
    pub name: String,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}

impl From<Wishlist> for Edge<Uuid, Wishlist, EmptyFields> {
    fn from(value: Wishlist) -> Self {
        let uuid = Uuid::parse_str(&value._id).unwrap();
        Edge::new(uuid, value)
    }
}
