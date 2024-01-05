use std::collections::HashSet;
use async_graphql::{SimpleObject, InputObject};
use uuid::Uuid;

#[derive(SimpleObject, InputObject)]
pub struct AddWishlistInput {
    /// UUID of user owning the wishlist.
    pub user_id: Uuid,
    /// UUID of user owning the wishlist.
    pub product_variant_ids: HashSet<Uuid>,
    /// Wishlist name.
    pub name: String
}


#[derive(SimpleObject, InputObject)]
pub struct UpdateWishlistInput {
    /// UUID of wishlist to update.
    pub id: Uuid,
    /// product_variant_version UUIDs of wishlist to update
    pub product_variant_ids: Option<HashSet<Uuid>>,
    /// Wishlist name to update
    pub name: Option<String>
}