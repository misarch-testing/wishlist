use std::{collections::HashSet, fs::File, io::Write};

use async_graphql::{http::GraphiQLSource, EmptySubscription, SDLExportOptions, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use clap::{arg, command, Parser};

use foreign_types::User;
use mongodb::{bson::DateTime, options::ClientOptions, Client, Collection, Database};

use tonic::transport::Server as TonicServer;
use dapr::dapr::dapr::proto::runtime::v1::app_callback_server::AppCallbackServer;

use bson::Uuid;
use wishlist::Wishlist;

mod wishlist;

mod query;
use query::Query;

mod mutation;
use mutation::Mutation;

mod app_callback_service;
use app_callback_service::AppCallbackService;

mod base_connection;
mod foreign_types;
mod mutation_input_structs;
mod order_datatypes;
mod product_variant_connection;
mod wishlist_connection;

/// Builds the GraphiQL frontend.
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

/// Establishes database connection and returns the client.
async fn db_connection() -> Client {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://wishlist-db:27017")
        .await
        .unwrap();

    // Manually set an option.
    client_options.app_name = Some("Wishlist".to_string());

    // Get a handle to the deployment.
    Client::with_options(client_options).unwrap()
}

/// Establishes connection to Dapr.
/// 
/// Adds AppCallbackService which defines pub/sub interaction with Dapr.
async fn dapr_connection() {
    let addr = "[::]:50051".parse().unwrap();

    let callback_service = AppCallbackService::default();

    println!("AppCallback server listening on: {}", addr);

    // Create a gRPC server with the callback_service.
    TonicServer::builder()
        .add_service(AppCallbackServer::new(callback_service))
        .serve(addr)
        .await.unwrap();
}

/// Can be used to insert dummy wishlist data in the MongoDB database.
#[allow(dead_code)]
async fn insert_dummy_data(collection: &Collection<Wishlist>) {
    let wishlists: Vec<Wishlist> = vec![Wishlist {
        _id: Uuid::new(),
        user: User { id: Uuid::new() },
        internal_product_variants: HashSet::new(),
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
        let sdl_export_options = SDLExportOptions::new().federation();
        let schema_sdl = schema.sdl_with_options(sdl_export_options);
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
    let db_client: Database = client.database("wishlist-database");
    let collection: mongodb::Collection<Wishlist> = db_client.collection::<Wishlist>("wishlists");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(collection)
        .enable_federation()
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://0.0.0.0:8080");

    let t1 = tokio::spawn(async {
        Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    });

    let t2 = tokio::spawn(async {
        dapr_connection().await;
    });
    
    t1.await.unwrap();
    t2.await.unwrap();
}
