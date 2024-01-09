use std::collections::HashSet;

use async_graphql::{
    connection::{Edge, EmptyFields},
    OutputType, SimpleObject,
};
use bson::datetime::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SimpleObject)]
pub struct Wishlist {
    pub _id: String,
    pub user_id: String,
    pub product_variant_ids: HashSet<String>,
    pub name: String,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}

impl From<Wishlist> for Uuid {
    fn from(value: Wishlist) -> Self {
        Uuid::parse_str(&value._id).unwrap()
    }
}

pub struct NodeWrapper<Node>(pub Node);

impl<Node> From<NodeWrapper<Node>> for Edge<Uuid, Node, EmptyFields>
where
    Node: Into<Uuid> + OutputType + Clone,
{
    fn from(value: NodeWrapper<Node>) -> Self {
        let uuid = Into::<Uuid>::into(value.0.clone());
        Edge::new(uuid, value.0)
    }
}