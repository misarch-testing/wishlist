use std::{cmp::Ordering, collections::HashSet};

use async_graphql::{
    connection::{Edge, EmptyFields},
    ComplexObject, OutputType, Result, SimpleObject,
};
use bson::datetime::DateTime;
use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{
    foreign_types::{ProductVariant, User},
    order_datatypes::{BaseOrder, OrderDirection},
    product_variant_connection::ProductVariantConnection,
    uuid_serde::serialize_uuid,
};

/// The Wishlist of a user.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Wishlist {
    /// Wishlist UUID.
    #[serde(serialize_with = "serialize_uuid")]
    pub _id: Uuid,
    /// User.
    pub user: User,
    /// Name of Wishlist.
    pub name: String,
    /// Timestamp when Wishlist was created.
    pub created_at: DateTime,
    /// Timestamp when Wishlist was last updated.
    pub last_updated_at: DateTime,
    #[graphql(visible = false)]
    pub internal_product_variants: HashSet<ProductVariant>,
}

#[ComplexObject]
impl Wishlist {
    /// Retrieves product variants.
    async fn product_variants(
        &self,
        #[graphql(desc = "Describes that the `first` N wishlists should be retrieved.")]
        first: Option<usize>,
        #[graphql(desc = "Describes how many wishlists should be skipped at the beginning.")]
        skip: Option<usize>,
        #[graphql(desc = "Specifies the order in which wishlists are retrieved.")] order_by: Option<
            BaseOrder,
        >,
    ) -> Result<ProductVariantConnection> {
        let mut product_variants: Vec<ProductVariant> =
            self.internal_product_variants.clone().into_iter().collect();
        let comparator: fn(&ProductVariant, &ProductVariant) -> bool =
            match order_by.unwrap_or_default().direction.unwrap_or_default() {
                OrderDirection::Asc => |x, y| x < y,
                OrderDirection::Desc => |x, y| x > y,
            };
        product_variants.sort_by(|x, y| match comparator(x, y) {
            true => Ordering::Less,
            false => Ordering::Greater,
        });
        let total_count = product_variants.len();
        let product_variants_part: Vec<ProductVariant> = product_variants
            .into_iter()
            .skip(skip.unwrap_or(0))
            .take(first.unwrap_or(usize::MAX))
            .collect();
        let has_next_page = total_count > product_variants_part.len();
        Ok(ProductVariantConnection {
            nodes: product_variants_part,
            has_next_page,
            total_count: total_count as u64,
        })
    }
}

impl From<Wishlist> for Uuid {
    fn from(value: Wishlist) -> Self {
        value._id
    }
}

pub struct NodeWrapper<Node>(pub Node);

impl<Node> From<NodeWrapper<Node>> for Edge<uuid::Uuid, Node, EmptyFields>
where
    Node: Into<uuid::Uuid> + OutputType + Clone,
{
    fn from(value: NodeWrapper<Node>) -> Self {
        let uuid = Into::<uuid::Uuid>::into(value.0.clone());
        Edge::new(uuid, value.0)
    }
}
