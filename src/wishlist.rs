use mongodb::bson::DateTime;
use async_graphql::Object;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wishlist {
    pub id: String,
    pub user_id: String,
    pub product_variant_ids: Vec<String>,
    pub name: String,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}

#[Object]
impl Wishlist {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn user_id(&self) -> String {
        self.user_id.clone()
    }

    async fn product_variant_ids(&self) -> Vec<String> {
        self.product_variant_ids.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn created_at(&self) -> String {
        self.created_at.try_to_rfc3339_string().unwrap()
    }

    async fn last_updated_at(&self) -> String {
        self.last_updated_at.try_to_rfc3339_string().unwrap()
    }
}