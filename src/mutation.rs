use std::collections::HashSet;

use async_graphql::{Context, Error, Object, Result};
use bson::Bson;
use bson::Uuid;
use mongodb::{
    bson::{doc, DateTime},
    Collection,
};

use crate::{
    foreign_types::{ProductVariant, User},
    mutation_input_structs::{AddWishlistInput, UpdateWishlistInput},
    query::query_wishlist,
    wishlist::Wishlist,
};

/// Describes GraphQL wishlist mutations.
pub struct Mutation;

#[Object]
impl Mutation {
    /// Adds a wishlist with a user_id, a list of product_variant_ids and a name.
    ///
    /// Formats UUIDs as hyphenated lowercase Strings.
    async fn add_wishlist<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "AddWishlistInput")] input: AddWishlistInput,
    ) -> Result<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let normalized_product_variants: HashSet<ProductVariant> = input
            .product_variant_ids
            .iter()
            .map(|id| ProductVariant { id: id.clone() })
            .collect();
        let current_timestamp = DateTime::now();
        let wishlist = Wishlist {
            _id: Uuid::new(),
            user: User { id: input.user_id },
            internal_product_variants: normalized_product_variants,
            name: input.name,
            created_at: current_timestamp,
            last_updated_at: current_timestamp,
        };
        match collection.insert_one(wishlist, None).await {
            Ok(result) => {
                let id = uuid_from_bson(result.inserted_id)?;
                query_wishlist(&collection, id).await
            }
            Err(_) => Err(Error::new("Adding wishlist failed in MongoDB.")),
        }
    }

    /// Updates name and/or product_variant_ids of a specific wishlist referenced with an id.
    ///
    /// Formats UUIDs as hyphenated lowercase Strings.
    async fn update_wishlist<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "UpdateWishlistInput")] input: UpdateWishlistInput,
    ) -> Result<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();

        let current_timestamp = DateTime::now();
        update_product_variant_ids(&collection, &input, &current_timestamp).await?;
        update_name(&collection, &input, &current_timestamp).await?;
        let wishlist = query_wishlist(&collection, input.id).await?;
        Ok(wishlist)
    }

    /// Deletes wishlist of id.
    async fn delete_wishlist<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "UUID of wishlist to delete.")] id: Uuid,
    ) -> Result<bool> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        if let Err(_) = collection.delete_one(doc! {"_id": id }, None).await {
            let message = format!("Deleting wishlist of id: `{}` failed in MongoDB.", id);
            return Err(Error::new(message));
        }
        Ok(true)
    }
}

/// Extracts UUID from Bson.
///
/// Adding a wishlist returns a UUID in a Bson document. This function helps to extract the UUID.
fn uuid_from_bson(bson: Bson) -> Result<Uuid> {
    match bson {
        Bson::Binary(id) => Ok(id.to_uuid()?),
        _ => {
            let message = format!(
                "Returned id: `{}` needs to be a Binary in order to be parsed as a Uuid",
                bson
            );
            Err(Error::new(message))
        }
    }
}

/// Updates product variant ids of a wishlist.
///
/// * `collection` - MongoDB collection to update.
/// * `input` - `UpdateWishlistInput`.
async fn update_product_variant_ids(
    collection: &Collection<Wishlist>,
    input: &UpdateWishlistInput,
    current_timestamp: &DateTime,
) -> Result<()> {
    if let Some(definitely_product_variant_ids) = &input.product_variant_ids {
        let normalized_product_variants: Vec<ProductVariant> = definitely_product_variant_ids
            .iter()
            .map(|id| ProductVariant { id: id.clone() })
            .collect();
        if let Err(_) = collection.update_one(doc!{"_id": input.id }, doc!{"$set": {"internal_product_variants": normalized_product_variants, "last_updated_at": current_timestamp}}, None).await {
            let message = format!("Updating product_variant_ids of wishlist of id: `{}` failed in MongoDB.", input.id);
            return Err(Error::new(message))
        }
    }
    Ok(())
}

/// Updates name of a wishlist.
///
/// * `collection` - MongoDB collection to update.
/// * `input` - `UpdateWishlistInput`.
async fn update_name(
    collection: &Collection<Wishlist>,
    input: &UpdateWishlistInput,
    current_timestamp: &DateTime,
) -> Result<()> {
    if let Some(definitely_name) = &input.name {
        let result = collection
            .update_one(
                doc! {"_id": input.id },
                doc! {"$set": {"name": definitely_name, "last_updated_at": current_timestamp}},
                None,
            )
            .await;
        if let Err(_) = result {
            let message = format!(
                "Updating name of wishlist of id: `{}` failed in MongoDB.",
                input.id
            );
            return Err(Error::new(message));
        }
    }
    Ok(())
}
