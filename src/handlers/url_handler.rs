use serde_json::{json, Value};

use axum::{
    Json,
    extract::{
        Path, State
    }, response::Redirect,
    
};

use redis::{AsyncCommands};

use std::{
    collections::HashMap,
    sync::Arc
};



use crate::{
    models::link::{Link, NewLink}, 
    reponses::response::{Response, ResponseGetShorten, ResponseNewShort, ResponseGetLink}, 
    state::{ app_state::AppState},
    repositories::redis_repository::RedisStore
};


pub async fn health() -> Json<Value> {
    Json(json!(
    {
        "message": "Rust URL Shortener API"
    }
))
}




pub async fn new_shorten(
    State(state): State<Arc<AppState>>,
    Json(x): Json<NewLink>
) -> Result<Json<ResponseNewShort>, Json<Response>> {

    if x.url.is_empty() {
        return Err(Json(Response{
            status_code: 404,
            message: "Url inválida0".to_string()
        }))
    };

    let redis = state.redis.clone();

    let short = match redis.create_link(&x.url).await {
        Ok(t) => t,
        Err(msg) => return Err(Json(Response{
            status_code: 404,
            message: msg
        }))
    };

    Ok(Json(ResponseNewShort {
        status_code: 201,
        short_code: short,
    }))
}



pub async fn get_shorten(
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, Json<Response>> {

    let redis = state.redis.clone();

    let url = match redis.get_url(&code).await {
        Ok(t) => t,
        Err(msg) => return Err(Json(Response{
            status_code: 404,
            message: msg
        }))
    };

    Ok(Redirect::temporary(&url))
}
/* 
pub async fn get_links(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<ResponseGetLink>>{
    let data = state.lock().unwrap();

    let res : Vec<ResponseGetLink> = data.map
    .iter()
    .map(|(code, link)|
        ResponseGetLink{
            code: code.clone(),
            original_url: link.original_url.clone(),
            clicks: link.clicks
        }
    )
    .collect();
    

    Json(res)
}

pub async fn del_shorten(
    Path(code): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<Response> {
    let mut data = state.lock().unwrap();

    let mut res = Response{
        status_code: 404,
        message: "not deleted".to_string()
    };


    if let Some(_) = data.map.remove(&code){
        res.status_code = 200;
        res.message = "deleted".to_string();
    }

    Json(res)

}*/