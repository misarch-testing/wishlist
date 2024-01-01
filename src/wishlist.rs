use uuid::Uuid;
use time::{PrimitiveDateTime, macros::format_description};
use async_graphql::Object;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wishlist {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_variant_ids: Vec<Uuid>,
    pub name: String,
    pub created_at: PrimitiveDateTime,
    pub last_updated_at: PrimitiveDateTime,
}

#[Object]
impl Wishlist {
    async fn id(&self) -> String {
        self.id.as_hyphenated().to_string()
    }

    async fn user_id(&self) -> String {
        self.user_id.as_hyphenated().to_string()
    }

    async fn product_variant_ids(&self) -> Vec<String> {
        self.product_variant_ids.iter().map(|uuid| uuid.as_hyphenated().to_string()).collect()
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn created_at(&self) -> String {
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        self.created_at.format(&format).unwrap()
    }

    async fn last_updated_at(&self) -> String {
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        self.last_updated_at.format(&format).unwrap()
    }
}