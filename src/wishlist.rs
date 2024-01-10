use std::collections::HashSet;

use async_graphql::{
    connection::{Edge, EmptyFields},
    OutputType, SimpleObject,
};
use bson::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::{
    custom_uuid::CustomUuid,
    foreign_types::{ProductVariant, User},
};

/// The Wishlist of a user.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SimpleObject)]
pub struct Wishlist {
    /// Wishlist UUID.
    pub _id: CustomUuid,
    /// User.
    pub user: User,
    /// Product variants in Wishlist.
    pub product_variants: HashSet<ProductVariant>,
    /// Name of Wishlist.
    pub name: String,
    /// Timestamp when Wishlist was created.
    pub created_at: DateTime,
    /// Timestamp when Wishlist was last updated.
    pub last_updated_at: DateTime,
}

impl From<Wishlist> for CustomUuid {
    fn from(value: Wishlist) -> Self {
        value._id
    }
}

pub struct NodeWrapper<Node>(pub Node);

impl<Node> From<NodeWrapper<Node>> for Edge<CustomUuid, Node, EmptyFields>
where
    Node: Into<CustomUuid> + OutputType + Clone,
{
    fn from(value: NodeWrapper<Node>) -> Self {
        let uuid = Into::<CustomUuid>::into(value.0.clone());
        Edge::new(uuid, value.0)
    }
}
