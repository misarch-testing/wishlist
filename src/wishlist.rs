use std::collections::HashSet;

use async_graphql::SimpleObject;
use bson::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Wishlist {
    pub _id: String,
    pub user_id: String,
    pub product_variant_ids: HashSet<String>,
    pub name: String,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}
