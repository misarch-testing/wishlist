use std::{collections::HashSet, fs::File, io::Write};

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use clap::{arg, command, Parser};
use mongodb::{bson::DateTime, options::ClientOptions, Client, Collection, Database};
use uuid::Uuid;

mod wishlist;
use wishlist::Wishlist;

mod query;
use query::Query;

mod mutation;
use mutation::Mutation;

mod mutation_input_structs;
mod order_datatypes;

mod base_connection;
mod wishlist_connection;

/// Builds the GraphiQL frontend.
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

/// Establishes database connection and returns the client.
async fn db_connection() -> Client {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://db:27017").await.unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    Client::with_options(client_options).unwrap()
}

/// Can be used to insert dummy wishlist data in the MongoDB database.
async fn insert_dummy_data(collection: &Collection<Wishlist>) {
    let wishlists: Vec<Wishlist> = vec![Wishlist {
        _id: Uuid::new_v4().as_hyphenated().to_string(),
        user_id: Uuid::new_v4().as_hyphenated().to_string(),
        product_variant_ids: HashSet::new(),
        name: String::from("test"),
        created_at: DateTime::now(),
        last_updated_at: DateTime::now(),
    }];
    collection.insert_many(wishlists, None).await.unwrap();
}

/// Command line argument to toggle schema generation instead of service execution.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Generates GraphQL schema in `./schemas/wishlist.graphql`.
    #[arg(long)]
    generate_schema: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.generate_schema {
        let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
        let mut file = File::create("./schemas/wishlist.graphql")?;
        let schema_sdl = schema.sdl();
        file.write_all(schema_sdl.as_bytes())?;
        println!("GraphQL schema: ./schemas/wishlist.graphql was successfully generated!");
    } else {
        start_service().await;
    }
    Ok(())
}

/// Starts wishlist service on port 8000.
async fn start_service() {
    let client = db_connection().await;
    let db: Database = client.database("wishlist-database");
    let collection: mongodb::Collection<Wishlist> = db.collection::<Wishlist>("wishlists");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(collection)
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://0.0.0.0:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
