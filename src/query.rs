use crate::{user::User, Wishlist};
use async_graphql::{Context, Error, Object, Result};

use bson::Uuid;
use mongodb::{bson::doc, Collection, Database};

/// Describes GraphQL wishlist queries.
pub struct Query;

#[Object]
impl Query {
    /// Retrieve user with wishlists.
    async fn user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "UUID of user to retrieve.")] id: Uuid,
    ) -> Result<User> {
        let db_client = ctx.data_unchecked::<Database>();
        let collection: Collection<User> = db_client.collection::<User>("users");
        query_user(&collection, id).await
    }

    /// Retrieves wishlist of specific id.
    async fn wishlist<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "UUID of wishlist to retrieve.")] id: Uuid,
    ) -> Result<Wishlist> {
        let db_client = ctx.data_unchecked::<Database>();
        let collection: Collection<Wishlist> = db_client.collection::<Wishlist>("wishlists");
        query_wishlist(&collection, id).await
    }

    /// Entity resolver for wishlist of specific key.
    #[graphql(entity)]
    async fn wishlist_entity_resolver<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(key, desc = "UUID of wishlist to retrieve.")] id: Uuid,
    ) -> Result<Wishlist> {
        let db_client = ctx.data_unchecked::<Database>();
        let collection: Collection<Wishlist> = db_client.collection::<Wishlist>("wishlists");
        query_wishlist(&collection, id).await
    }
}

/// Shared function to query a wishlist from a MongoDB collection of wishlists
///
/// * `connection` - MongoDB database connection.
/// * `id` - UUID of wishlist.
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

/// Shared function to query a user from a MongoDB collection of users.
///
/// * `connection` - MongoDB database connection.
/// * `id` - UUID of user.
pub async fn query_user(collection: &Collection<User>, id: Uuid) -> Result<User> {
    match collection.find_one(doc! {"_id": id }, None).await {
        Ok(maybe_user) => match maybe_user {
            Some(user) => Ok(user),
            None => {
                let message = format!("User with UUID id: `{}` not found.", id);
                Err(Error::new(message))
            }
        },
        Err(_) => {
            let message = format!("User with UUID id: `{}` not found.", id);
            Err(Error::new(message))
        }
    }
}
