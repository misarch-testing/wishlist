use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    SimpleObject,
};
use mongodb_cursor_pagination::FindResult;
use uuid::Uuid;

use crate::wishlist::Wishlist;

pub struct FindResultWishlist(pub FindResult<Wishlist>);

/// Object that writes total count of items in a query, regardless of pagination.
#[derive(SimpleObject)]
pub struct TotalCount {
    total_count: u64,
}

/// Implementation of conversion from MongoDB pagination to GraphQL Connection.
impl From<FindResultWishlist> for Connection<Uuid, Wishlist, TotalCount> {
    fn from(value: FindResultWishlist) -> Self {
        let has_previous_page = value.0.page_info.has_previous_page;
        let has_next_page = value.0.page_info.has_next_page;
        let additional_fields = TotalCount {
            total_count: value.0.total_count,
        };
        let mut connection =
            Connection::with_additional_fields(has_previous_page, has_next_page, additional_fields);
        let items = value.0.items;
        let edges: Vec<Edge<Uuid, Wishlist, EmptyFields>> = items
            .iter()
            .map(|v| Into::<Edge<Uuid, Wishlist, EmptyFields>>::into(v.clone()))
            .collect();
        connection.edges.extend(edges);
        connection
    }
}
