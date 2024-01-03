use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use mongodb::{Client, options::ClientOptions, bson::DateTime, Collection, Database};
use uuid::Uuid;

mod wishlist;
use wishlist::Wishlist;

mod query_root;
use query_root::QueryRoot;

mod mutation_root;
use mutation_root::MutationRoot;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

/// Establishes database connection and returns the client.
async fn db_connection() -> Client {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://db:27017").await.unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment
    Client::with_options(client_options).unwrap()
}

/// Can be used to insert dummy wishlist data in the MongoDB database.
async fn insert_dummy_data(collection: &Collection<Wishlist>) {
    let wishlists: Vec<Wishlist> = vec![Wishlist {
        id: Uuid::new_v4().as_hyphenated().to_string(),
        user_id: Uuid::new_v4().as_hyphenated().to_string(),
        product_variant_ids: vec![],
        name: String::from("test"),
        created_at: DateTime::now(),
        last_updated_at: DateTime::now(),
    }];
    collection.insert_many(wishlists, None).await.unwrap();
}

#[tokio::main]
async fn main() {
    let client = db_connection().await;

    let db: Database = client.database("wishlist-database");
    
    let collection: mongodb::Collection<Wishlist> = db.collection::<Wishlist>("wishlists");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(collection)
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
