use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use mongodb::{Client, options::ClientOptions};
use uuid::Uuid;
use time::macros::datetime;

mod wishlist;
use wishlist::Wishlist;

mod query_root;
use query_root::QueryRoot;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn db_connection() -> Client {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://db:27017").await.unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment
    Client::with_options(client_options).unwrap()
}

#[tokio::main]
async fn main() {
    let client = db_connection().await;

    let db: mongodb::Database = client.database("wishlist-database");

    let collection: mongodb::Collection<Wishlist> = db.collection::<Wishlist>("wishlists");

    // Dummy data to insert
    let wishlists: Vec<Wishlist> = vec![Wishlist {
        id: &Uuid::new_v4().as_hyphenated().to_string(),
        user_id: &Uuid::new_v4().as_hyphenated().to_string(),
        product_variant_ids: vec![],
        name: "test",
        created_at: datetime!(2019-01-01 0:00 UTC),
        last_updated_at: datetime!(2019-01-01 0:00 UTC),
    }];
    collection.insert_many(wishlists, None).await.unwrap();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(collection)
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}