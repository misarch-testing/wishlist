use axum::{debug_handler, extract::State, http::StatusCode, Json};
use bson::Uuid;
use log::info;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{foreign_types::ProductVariant, user::User};

#[derive(Serialize)]
pub struct Pubsub {
    pub pubsubname: String,
    pub topic: String,
    pub routes: Vec<Route>,
}

#[derive(Serialize)]
pub struct Route {
    pub rules: Vec<Rule>,
    pub default: String,
}

#[derive(Serialize)]
pub struct Rule {
    // Needs to be renamed because it is a rust keyword.
    #[serde(rename(serialize = "match"))]
    pub match_field: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct TopicEventResponse {
    pub status: i32,
}

impl Default for TopicEventResponse {
    fn default() -> Self {
        Self { status: 0 }
    }
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub id: Uuid,
    pub topic: String,
}

#[derive(Clone)]
pub struct HttpEventServiceState {
    pub product_variant_collection: Collection<ProductVariant>,
    pub user_collection: Collection<User>,
}

pub async fn list_topic_subscriptions() -> Result<Json<Vec<Pubsub>>, StatusCode> {
    let pubsub_user = Pubsub {
        pubsubname: "pubsub".to_string(),
        topic: "user/user/created".to_string(),
        routes: vec![Route {
            rules: vec![],
            default: "/on-topic-event".to_string(), // TODO: Check if this is really the right value.
        }],
    };
    let pubsub_product_variant = Pubsub {
        pubsubname: "pubsub".to_string(),
        topic: "catalog/product-variant/created".to_string(),
        routes: vec![Route {
            rules: vec![],
            default: "catalog/product-variant/created".to_string(), // TODO: Check if this is really the right value.
        }],
    };
    Ok(Json(vec![pubsub_user, pubsub_product_variant]))
}

#[debug_handler(state = HttpEventServiceState)]
pub async fn on_topic_event(
    State(state): State<HttpEventServiceState>,
    Json(event): Json<Event>,
) -> Result<Json<TopicEventResponse>, StatusCode> {
    info!("{:?}", event);

    match event.id.to_string().as_str() {
        "catalog/product-variant/created" => {
            add_product_variant_to_mongodb(state.product_variant_collection, event.id).await?
        }
        "user/user/created" => add_user_to_mongodb(state.user_collection, event.id).await?,
        _ => {
            // TODO: This message can be used for further visibility.
            let _message = format!(
                "Event of topic: `{}` is not a handable by this service.",
                event.topic.as_str()
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    Ok(Json(TopicEventResponse::default()))
}

/// Add a newly created product variant to MongoDB.
pub async fn add_product_variant_to_mongodb(
    collection: Collection<ProductVariant>,
    id: Uuid,
) -> Result<(), StatusCode> {
    let product_variant = ProductVariant { _id: id };
    match collection.insert_one(product_variant, None).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Add a newly created user to MongoDB.
pub async fn add_user_to_mongodb(collection: Collection<User>, id: Uuid) -> Result<(), StatusCode> {
    let user = User { _id: id };
    match collection.insert_one(user, None).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
