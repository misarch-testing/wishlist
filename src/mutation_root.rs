use async_graphql::{Object, Context, Error};
use mongodb::{Collection, bson::doc};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::wishlist::Wishlist;
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_wishlist<'a>(&self, ctx: &Context<'a>, user_id: String, product_variant_ids: Vec<String>, name: String) -> bool {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let normalized_product_variant_ids = product_variant_ids.iter().map(|id| Uuid::parse_str(&id).unwrap().as_hyphenated().to_string()).collect();
        let current_timestamp = OffsetDateTime::now_utc();
        let wishlist = Wishlist {
            id: Uuid::new_v4().as_hyphenated().to_string(),
            user_id: Uuid::parse_str(&user_id).unwrap().as_hyphenated().to_string(),
            product_variant_ids: normalized_product_variant_ids,
            name: name,
            created_at: current_timestamp,
            last_updated_at: current_timestamp,
        };
        collection.insert_one(wishlist, None).await.unwrap();
        true
    }

    async fn update_wishlist<'a>(&self, ctx: &Context<'a>, id: String, product_variant_ids: Vec<String>) -> bool {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let normalized_product_variant_ids: Vec<String> = product_variant_ids.iter().map(|id| Uuid::parse_str(&id).unwrap().as_hyphenated().to_string()).collect();
        let parsed_uuid = Uuid::parse_str(&id).unwrap();
        collection.update_one(doc!{"id": parsed_uuid.as_hyphenated().to_string() }, doc!{"$set": {"productVariantIds": normalized_product_variant_ids}}, None).await.unwrap();
        true
    }

    async fn delete_wishlist<'a>(&self, ctx: &Context<'a>, id: String) -> bool {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let parsed_uuid = Uuid::parse_str(&id).unwrap();
        collection.delete_one(doc!{"id": parsed_uuid.as_hyphenated().to_string() }, None).await.unwrap();
        true
    }
}