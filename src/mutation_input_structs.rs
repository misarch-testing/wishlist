use async_graphql::{InputObject, SimpleObject};
use std::collections::HashSet;

use crate::custom_uuid::CustomUuid;

#[derive(SimpleObject, InputObject)]
pub struct AddWishlistInput {
    /// UUID of user owning the wishlist.
    pub user_id: CustomUuid,
    /// UUIDs of product variants in wishlist.
    pub product_variant_ids: HashSet<CustomUuid>,
    /// Wishlist name.
    pub name: String,
}

#[derive(SimpleObject, InputObject)]
pub struct UpdateWishlistInput {
    /// UUID of wishlist to update.
    pub id: CustomUuid,
    /// product variant UUIDs of wishlist to update
    pub product_variant_ids: Option<HashSet<CustomUuid>>,
    /// Wishlist name to update
    pub name: Option<String>,
}
