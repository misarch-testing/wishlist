use time::{OffsetDateTime, format_description::well_known::Iso8601};
use async_graphql::Object;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wishlist<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub product_variant_ids: Vec<&'a str>,
    pub name: &'a str,
    pub created_at: OffsetDateTime,
    pub last_updated_at: OffsetDateTime,
}

#[Object]
impl<'a> Wishlist<'a> {
    async fn id(&self) -> &str {
        self.id
    }

    async fn user_id(&self) -> &str {
        self.user_id
    }

    async fn product_variant_ids(&self) -> Vec<&str> {
        self.product_variant_ids.clone()
    }

    async fn name(&self) -> &str {
        self.name
    }

    async fn created_at(&self) -> String {
        self.created_at.format(&Iso8601::DEFAULT).unwrap()
    }

    async fn last_updated_at(&self) -> String {
        self.last_updated_at.format(&Iso8601::DEFAULT).unwrap()
    }
}