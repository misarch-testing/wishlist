use async_graphql::{Object, Context, Error, FieldResult};
use mongodb::{Collection, bson::doc};
use uuid::Uuid;
use crate::Wishlist;
use futures::stream::TryStreamExt;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Retrieves all wishlists.
    async fn wishlists<'a>(&self, ctx: &Context<'a>) -> FieldResult<Vec<Wishlist>> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let mut cursor = collection.find(None, None).await.unwrap();
        let mut wishlists = vec![];
        loop {
            match cursor.try_next().await {
                Ok(maybe_wishlist) => match maybe_wishlist {
                    Some(wishlist) => wishlists.push(wishlist),
                    None => break
                },
                Err(_) => return Err(Error::new("Retrieving wishlists failed in MongoDB."))
            }
        }
        Ok(wishlists)
    }

    /// Retrieves wishlist of specific id.
    async fn wishlist<'a>(&self, ctx: &Context<'a>, id: String) -> FieldResult<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        match Uuid::parse_str(&id) {
            Ok(parsed_uuid) => {
                match collection.find_one(doc!{"id": parsed_uuid.as_hyphenated().to_string() }, None).await {
                    Ok(maybe_wishlist) => match maybe_wishlist {
                        Some(wishlist) => Ok(wishlist),
                        None => {
                            let message = format!("Wishlist with UUID id: `{id}` not found.");
                            Err(Error::new(message))
                        }
                    },
                    Err(_) => {
                        let message = format!("Wishlist with UUID id: `{id}` not found.");
                        Err(Error::new(message))
                    }
                }
            },
            Err(_) => {
                let message = format!("UUID id: `{id}` is not a valid UUID.");
                Err(Error::new(message))
            }
        }
    }
}

