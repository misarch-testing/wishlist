use async_graphql::{Object, Context};
use mongodb::Collection;
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

    // async fn wishlist<'a>(&self, ctx: &Context<'a>, id: usize) -> &'a Wishlist {
    //     let database = ctx.data_unchecked::<Database>();
    //     let collection = database.collection::<Wishlist>("wishlists");
    //     let mut cursor = collection.find().await;
    // }
}