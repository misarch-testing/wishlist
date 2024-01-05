use std::collections::HashSet;

use bson::datetime::DateTime;
use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Wishlist {
    pub id: String,
    pub user_id: String,
    pub product_variant_ids: HashSet<String>,
    pub name: String,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}
