use serde_json::{json, Value};
use axum::{
    Json,
    extract::{
        State,
        Path
    },
    
};
use std::{collections::HashMap, sync::{
    Arc, Mutex
}};
use rand::Rng;

use crate::{
    models::link::{Link, NewLink}, 
    reponses::response::{ResponseErro, ResponseGetShorten, ResponseNewShort}, 
    state::{self, app_state::AppState}
};


pub async fn health() -> Json<Value> {
    Json(json!(
    {
        "message": "Rust URL Shortener API"
    }
))
}


pub async fn short_code(size: usize) -> String {
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect()
}

pub async fn new_shorten(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(x): Json<NewLink>
) -> Json<ResponseNewShort> {
    let short = short_code(8).await;

    let mut data = state.lock().unwrap();

    let link = Link{original_url: x.url, clicks: 0};

    let new_short = format!("{short}{}", data.end);

    data.map.insert(
        new_short.clone(),
        link
    );

    data.end += 1;

    Json(ResponseNewShort {
        status_code: 201,
        short_code: new_short,
    })
}

pub async fn get_shorten(
    Path(code): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<ResponseGetShorten>, Json<ResponseErro>> {
    let mut data = state.lock().unwrap();

    match data.map.get_mut(&code) {
        Some(link) => {
            link.clicks += 1;

            let res = ResponseGetShorten {
                url: link.original_url.clone(),
            };

            Ok(Json(res))
        }
        None => {
            let res = ResponseErro {
                status_code: 404,
                message: format!("Erro ao procurar {}", code),
            };

            Err(Json(res))
        }
    }
}

pub async fn get_links(State(state): State<Arc<Mutex<AppState>>>) -> Json<HashMap<String, Link>>{
    let data = state.lock().unwrap();
    Json(data.map.clone())
}