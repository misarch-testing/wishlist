use crate::{
    base_connection::{BaseConnection, FindResultWrapper},
    order_datatypes::WishlistOrder,
    wishlist_connection::WishlistConnection,
    Wishlist,
};
use async_graphql::{Context, Error, Object, Result};
use bson::Document;
use bson::Uuid;
use mongodb::{bson::doc, options::FindOptions, Collection};
use mongodb_cursor_pagination::{error::CursorError, FindResult, PaginatedCursor};

/// Describes GraphQL wishlist queries.
pub struct Query;

#[Object]
impl Query {
    /// Retrieves all wishlists.
    async fn wishlists<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Describes that the `first` N wishlists should be retrieved.")]
        first: Option<u32>,
        #[graphql(desc = "Describes how many wishlists should be skipped at the beginning.")]
        skip: Option<u64>,
        #[graphql(desc = "Specifies the order in which wishlists are retrieved.")] order_by: Option<
            WishlistOrder,
        >,
    ) -> Result<WishlistConnection> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let wishlist_order = order_by.unwrap_or_default();
        let sorting_doc = doc! {wishlist_order.field.unwrap_or_default().as_str(): i32::from(wishlist_order.direction.unwrap_or_default())};
        let find_options = FindOptions::builder()
            .skip(skip)
            .limit(first.map(|v| i64::from(v)))
            .sort(sorting_doc)
            .build();
        let document_collection = collection.clone_with_type::<Document>();
        let maybe_find_results: Result<FindResult<Wishlist>, CursorError> =
            PaginatedCursor::new(Some(find_options.clone()), None, None)
                .find(&document_collection, None)
                .await;
        match maybe_find_results {
            Ok(find_results) => {
                let find_result_wrapper = FindResultWrapper(find_results);
                let connection = Into::<BaseConnection<Wishlist>>::into(find_result_wrapper);
                Ok(Into::<WishlistConnection>::into(connection))
            }
            Err(_) => return Err(Error::new("Retrieving wishlists failed in MongoDB.")),
        }
    }

    /// Retrieves wishlist of specific id.
    async fn wishlist<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(key, desc = "UUID of wishlist to retrieve.")] id: Uuid,
    ) -> Result<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        query_wishlist(&collection, id).await
    }
}

/// Shared function to query a wishlist from a MongoDB collection of wishlists
///
/// * `connection` - MongoDB database connection.
/// * `stringified_uuid` - UUID of wishlist as String.
pub async fn query_wishlist(collection: &Collection<Wishlist>, id: Uuid) -> Result<Wishlist> {
    match collection.find_one(doc! {"_id": id }, None).await {
        Ok(maybe_wishlist) => match maybe_wishlist {
            Some(wishlist) => Ok(wishlist),
            None => {
                let message = format!("Wishlist with UUID id: `{}` not found.", id);
                Err(Error::new(message))
            }
        },
        Err(_) => {
            let message = format!("Wishlist with UUID id: `{}` not found.", id);
            Err(Error::new(message))
        }
    }
}
