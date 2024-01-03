use async_graphql::{Object, Context, FieldResult, Error};
use mongodb::{Collection, bson::{doc, DateTime}};
use uuid::Uuid;

use crate::wishlist::Wishlist;
pub struct MutationRoot;

fn uuid_parse(id: &String) -> Result<String, Error> {
    match Uuid::parse_str(&id) {
        Ok(parsed_uuid) => Ok(parsed_uuid.as_hyphenated().to_string()),
        Err(_) => {
            let message = format!("Could not parse UUID: `{id}` is not a valid UUID.");
            Err(Error::new(message))
        }
    }
}

#[Object]
impl MutationRoot {
    /// Adds a wishlist with a user_id, a list of product_variant_ids and a name.
    async fn add_wishlist<'a>(&self, ctx: &Context<'a>, user_id: String, product_variant_ids: Vec<String>, name: String) -> FieldResult<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let maybe_normalized_product_variant_ids: Result<Vec<String>, Error> = product_variant_ids.iter().map(uuid_parse).collect();
        let normalized_product_variant_ids = maybe_normalized_product_variant_ids?;
        let current_timestamp = DateTime::now();
        let parsed_user_id = uuid_parse(&user_id)?;
        let wishlist = Wishlist {
            id: Uuid::new_v4().as_hyphenated().to_string(),
            user_id: parsed_user_id,
            product_variant_ids: normalized_product_variant_ids,
            name: name,
            created_at: current_timestamp,
            last_updated_at: current_timestamp,
        };
        match collection.insert_one(wishlist, None).await {
            Ok(_) => Ok(true),
            Err(_) => {
                let message = format!("Adding wishlist failed in MongoDB.");
                Err(Error::new(message))
            }
        }
    }
    
    /// Updates name and/or product_variant_ids of a specific wishlist referenced with an id.
    async fn update_wishlist<'a>(&self, ctx: &Context<'a>, id: String, product_variant_ids: Option<Vec<String>>, name: Option<String>) -> FieldResult<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let parsed_uuid = uuid_parse(&id)?;
        let current_timestamp = DateTime::now();
        if let Some(definitely_product_variant_ids) = product_variant_ids {
            let maybe_normalized_product_variant_ids: Result<Vec<String>, Error> = definitely_product_variant_ids.iter().map(uuid_parse).collect();
            let normalized_product_variant_ids = maybe_normalized_product_variant_ids?;
            if let Err(_) = collection.update_one(doc!{"id": &parsed_uuid }, doc!{"$set": {"product_variant_ids": normalized_product_variant_ids, "last_updated_at": current_timestamp}}, None).await {
                let message = format!("Updating product_variant_ids of wishlist of id: `{id}` failed in MongoDB.");
                return Err(Error::new(message))
            }
        }
        if let Some(definitely_name) = name {
            if let Err(_) = collection.update_one(doc!{"id": parsed_uuid }, doc!{"$set": {"name": definitely_name, "last_updated_at": current_timestamp}}, None).await {
                let message = format!("Updating name of wishlist of id: `{id}` failed in MongoDB.");
                return Err(Error::new(message))
            }
        }
        Ok(true)
    }
    
    /// Deletes wishlist of id.
    async fn delete_wishlist<'a>(&self, ctx: &Context<'a>, id: String) -> FieldResult<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let parsed_uuid = uuid_parse(&id)?;
        if let Err(_) = collection.delete_one(doc!{"id": parsed_uuid }, None).await {
            let message = format!("Deleting wishlist of id: `{id}` failed in MongoDB.");
            return Err(Error::new(message))
        }
        Ok(true)
    }
}