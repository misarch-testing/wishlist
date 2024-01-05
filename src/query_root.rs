use async_graphql::{Object, Context, Error, FieldResult, ID};
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
    async fn wishlist<'a>(&self, ctx: &Context<'a>,
        #[graphql(desc = "UUID of wishlist.")] id: Uuid
    ) -> FieldResult<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let stringified_uuid = id.as_hyphenated().to_string();
        match collection.find_one(doc!{"id": id.as_hyphenated().to_string() }, None).await {
            Ok(maybe_wishlist) => match maybe_wishlist {
                Some(wishlist) => Ok(wishlist),
                None => {
                    let message = format!("Wishlist with UUID id: `{}` not found.", stringified_uuid);
                    Err(Error::new(message))
                }
            },
            Err(_) => {
                let message = format!("Wishlist with UUID id: `{}` not found.", stringified_uuid);
                Err(Error::new(message))
            }
        }
    }
}

