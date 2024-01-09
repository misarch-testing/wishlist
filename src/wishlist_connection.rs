use async_graphql::connection::{Connection, Edge};
use uuid::Uuid;
use crate::wishlist::Wishlist;

pub type WishlistConnection = Connection<Wishlist, Uuid>;
pub type WishlistEdge = Edge<Wishlist, Uuid, ()>;

fn create_wishlist_connection() -> WishlistConnection {
    WishlistConnection {
        edges: Vec::<WishlistEdge>::new(),
        additional_fields: (),
        has_next_page: true,
        has_previous_page: true,
        _mark1: std::marker::PhantomData,
        _mark2: std::marker::PhantomData,
        _mark3: std::marker::PhantomData,
    }
}