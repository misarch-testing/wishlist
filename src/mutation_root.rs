use std::collections::HashSet;

use async_graphql::{Object, Context, FieldResult, Error};
use mongodb::{Collection, bson::{doc, DateTime}};
use uuid::Uuid;

use crate::{wishlist::Wishlist, input_structs::{AddWishlistInput, UpdateWishlistInput}};
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Adds a wishlist with a user_id, a list of product_variant_ids and a name.
    async fn add_wishlist<'a>(&self,
        ctx: &Context<'a>, 
        input: AddWishlistInput,
    ) -> FieldResult<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let normalized_product_variant_ids: HashSet<String> = input.product_variant_ids.iter().map(|id| id.as_hyphenated().to_string()).collect();
        let current_timestamp = DateTime::now();
        let stringified_user_id = input.user_id.as_hyphenated().to_string();
        let wishlist = Wishlist {
            id: Uuid::new_v4().as_hyphenated().to_string(),
            user_id: stringified_user_id,
            product_variant_ids: normalized_product_variant_ids,
            name: input.name,
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
    async fn update_wishlist<'a>(&self,
        ctx: &Context<'a>,
        input: UpdateWishlistInput,
    ) -> FieldResult<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let stringified_uuid = input.id.as_hyphenated().to_string();
        let current_timestamp = DateTime::now();
        if let Some(definitely_product_variant_ids) = input.product_variant_ids {
            let normalized_product_variant_ids: Vec<String> = definitely_product_variant_ids.iter().map(|id| id.as_hyphenated().to_string()).collect();
            if let Err(_) = collection.update_one(doc!{"id": &stringified_uuid }, doc!{"$set": {"product_variant_ids": normalized_product_variant_ids, "last_updated_at": current_timestamp}}, None).await {
                let message = format!("Updating product_variant_ids of wishlist of id: `{}` failed in MongoDB.", &stringified_uuid);
                return Err(Error::new(message))
            }
        }
        if let Some(definitely_name) = input.name {
            if let Err(_) = collection.update_one(doc!{"id": &stringified_uuid }, doc!{"$set": {"name": definitely_name, "last_updated_at": current_timestamp}}, None).await {
                let message = format!("Updating name of wishlist of id: `{}` failed in MongoDB.", stringified_uuid);
                return Err(Error::new(message))
            }
        }
        Ok(true)
    }
    
    /// Deletes wishlist of id.
    async fn delete_wishlist<'a>(&self,
        ctx: &Context<'a>,
        #[graphql(desc = "UUID of wishlist to delete.")] id: Uuid
    ) -> FieldResult<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let stringified_uuid = id.as_hyphenated().to_string();
        if let Err(_) = collection.delete_one(doc!{"id": &stringified_uuid }, None).await {
            let message = format!("Deleting wishlist of id: `{}` failed in MongoDB.", stringified_uuid);
            return Err(Error::new(message))
        }
        Ok(true)
    }
}