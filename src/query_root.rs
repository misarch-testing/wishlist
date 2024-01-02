use async_graphql::{Object, Context};
use mongodb::{Collection, bson::doc};
use uuid::Uuid;
use crate::Wishlist;
use futures::stream::TryStreamExt;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn wishlists<'a>(&self, ctx: &Context<'a>) -> Vec<Wishlist> {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let mut cursor = collection.find(None, None).await.unwrap();
        let mut wishlists = vec![];
        while let Some(wishlist) = cursor.try_next().await.unwrap() {
            wishlists.push(wishlist);
        }
        wishlists
    }

    async fn wishlist<'a>(&self, ctx: &Context<'a>, id: String) -> Wishlist {
        let collection: &Collection<Wishlist> = ctx.data_unchecked::<Collection<Wishlist>>();
        let parsed_uuid = Uuid::parse_str(&id).unwrap();
        let wishlist = collection.find_one(doc!{"id": parsed_uuid.as_hyphenated().to_string() }, None).await.unwrap().unwrap();
        wishlist
    }
}

