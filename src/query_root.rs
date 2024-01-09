use crate::{order_datatypes::WishlistOrder, Wishlist};
use async_graphql::{Context, Error, FieldResult, Object};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Collection};
use uuid::Uuid;

/// Describes GraphQL wishlist queries.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Retrieves all wishlists.
    ///
    /// * `ctx` - GraphQL context containing DB connection.
    /// * `first` - Describes that the `first` N wishlists should be retrieved.
    /// * `skip` - Describes how many wishlists should be skipped at the beginning.
    /// * `order_by` - Specifies the order in which wishlists are retrieved.
    async fn wishlists<'a>(
        &self,
        ctx: &Context<'a>,
        first: Option<u32>,
        skip: Option<u64>,
        order_by: Option<WishlistOrder>,
    ) -> FieldResult<Vec<Wishlist>> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let wishlist_order = order_by.unwrap_or_default();
        let sorting_doc = doc! {wishlist_order.order_field.unwrap_or_default().as_str(): i32::from(wishlist_order.order_direction.unwrap_or_default())};
        let find_options = FindOptions::builder()
            .skip(skip)
            .limit(first.map(|v| i64::from(v)))
            .sort(sorting_doc)
            .build();
        let mut cursor = collection.find(None, find_options).await.unwrap();
        let mut wishlists = vec![];
        loop {
            match cursor.try_next().await {
                Ok(maybe_wishlist) => match maybe_wishlist {
                    Some(wishlist) => wishlists.push(wishlist),
                    None => break,
                },
                Err(_) => return Err(Error::new("Retrieving wishlists failed in MongoDB.")),
            }
        }
        Ok(wishlists)
    }

    /// Retrieves wishlist of specific id.
    ///
    /// * `ctx` - GraphQL context containing DB connection.
    /// * `id` - UUID of wishlist to retrieve.
    async fn wishlist<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "UUID of wishlist.")] id: Uuid,
    ) -> FieldResult<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let stringified_uuid = id.as_hyphenated().to_string();
        query_wishlist(&collection, &stringified_uuid).await
    }
}

/// Shared function to query a wishlist from a MongoDB collection of wishlists
///
/// * `connection` - MongoDB database connection.
/// * `stringified_uuid` - UUID of wishlist as String.
pub async fn query_wishlist(
    collection: &Collection<Wishlist>,
    stringified_uuid: &String,
) -> FieldResult<Wishlist> {
    match collection
        .find_one(doc! {"_id": &stringified_uuid }, None)
        .await
    {
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
