mod models;
mod handlers;
mod state;
mod reponses;
mod repositories;

use axum::{
    Router,
    routing::{get, post, delete},
};

use deadpool_redis::{
    Config,
};

use std::sync::Arc;

use handlers::url_handler::{health, new_shorten, get_shorten, get_links, del_shorten};
use state::app_state::AppState;
use repositories::redis_repository::RedisStore;


#[tokio::main]
async fn main() {

    let cfg = Config::from_url("redis://localhost:6379");

    let state = Arc::new(AppState {
        redis: RedisStore::new(cfg)
    });

    let app = Router::new()
    .route("/", get(health))
    .route("/shorten", post(new_shorten))
    .route("/r/{code}", get(get_shorten))
    .route("/links", get(get_links))
    .route("/links/{code}", delete(del_shorten))
    .with_state(state);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Start");

    axum::serve(listener, app).await.unwrap();
}
