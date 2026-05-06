mod models;
mod handlers;
mod state;
mod reponses;

use axum::{
    Router,
    routing::{get, post},
};

use std::{
    collections::HashMap,
    sync::{
        Arc,Mutex
    }
};

use handlers::url_handler::{health, new_shorten, get_shorten};
use models::link::Link;
use state::app_state::AppState;


#[tokio::main]
async fn main() {

    let state = AppState{map: HashMap::new(), end: 0};
    let shared = Arc::new(Mutex::new(state));
    
    let app = Router::new()
    .route("/", get(health))
    .route("/shorten", post(new_shorten))
    .route("/r/{code}", get(get_shorten))
    .with_state(shared);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Start");

    axum::serve(listener, app).await.unwrap();
}
