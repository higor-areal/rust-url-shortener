use serde_json::{json, Value};

use axum::{
    Json,
    extract::{
        Path, State
    }, response::Redirect,
    
};

use std::{
    sync::Arc
};



use crate::{
    models::link::{Link, NewLink}, reponses::response::{Response, ResponseNewShort}, state::app_state::AppState
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

pub async fn get_links(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Link>>, Json<Response>>{
    let redis = state.redis.clone();

    let res : Vec<Link> = match redis.lasted_links().await {
        Ok(t) => t,
        Err(msg) => return Err(Json(Response{
            status_code: 404,
            message: msg
        }))
    };
    
    Ok(Json(res))
}


pub async fn del_shorten(
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<Response> {
    let redis = state.redis.clone();

    let mut res = Response{
        status_code: 404,
        message: "not deleted".to_string()
    };


    match redis.del_code(&code).await{
        Ok(_) => {
            res.status_code = 200;
            res.message = "deleted".to_string();
        },
        Err(msg) => res.message = msg
    }

    Json(res)

}