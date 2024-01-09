use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    OutputType, SimpleObject,
};
use mongodb_cursor_pagination::FindResult;
use uuid::Uuid;

use crate::wishlist::NodeWrapper;

pub struct FindResultWrapper<Node>(pub FindResult<Node>);

/// Object that writes total count of items in a query, regardless of pagination.
#[derive(SimpleObject)]
pub struct AdditionalFields {
    total_count: u64,
}

/// Implementation of conversion from MongoDB pagination to GraphQL Connection.
impl<Node> From<FindResultWrapper<Node>>
    for Connection<Uuid, Node, AdditionalFields>
where
    Node: Into<Uuid> + OutputType + Clone,
{
    fn from(value: FindResultWrapper<Node>) -> Self {
        let has_previous_page = value.0.page_info.has_previous_page;
        let has_next_page = value.0.page_info.has_next_page;
        let additional_fields = AdditionalFields {
            total_count: value.0.total_count,
        };
        let mut connection =
            Connection::with_additional_fields(has_previous_page, has_next_page, additional_fields);
        let items = value.0.items;
        let edges: Vec<Edge<Uuid, Node, EmptyFields>> = items
            .iter()
            .map(|v| Into::<Edge<Uuid, Node, EmptyFields>>::into(NodeWrapper(v.clone())))
            .collect();
        connection.edges.extend(edges);
        connection
    }
}
