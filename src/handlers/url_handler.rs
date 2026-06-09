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

use rand::Rng;

use crate::{
    models::link::{Link, NewLink}, 
    reponses::response::{Response, ResponseGetShorten, ResponseNewShort, ResponseGetLink}, 
    state::{ app_state::AppState}
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
    State(state): State<Arc<AppState>>,
    Json(x): Json<NewLink>
) -> Result<Json<ResponseNewShort>, Json<Response>> {
    print!("{}", x.url);
    if x.url.is_empty() {
        return Err(Json(Response{
            status_code: 404,
            message: "Url inválida0".to_string()
        }))
    }

    let short = short_code(8).await;

    let mut conn = state.get_pool().await?;


    let _: () = conn
        .hset_multiple(
            &short,
            &[("url", x.url.as_str()), ("clicks", "0")]
        )
        .await
        .map_err(|_| Json(Response {
            status_code: 404,
            message: "Url inválida".to_string(),
        }))?;

    Ok(Json(ResponseNewShort {
        status_code: 201,
        short_code: short,
    }))
}



pub async fn get_shorten(
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, Json<Response>> {

    let mut conn = state.get_pool().await?;

    let res: HashMap<String, String> = match conn.hgetall(&code).await {
        Ok(data) => data,
        Err(_) => {
            return Err(Json(Response {
                status_code: 500,
                message: "Erro ao buscar link".to_string(),
            }))
        }
    };

    if res.is_empty() {
        return Err(Json(Response {
            status_code: 404,
            message: "Link não encontrado".to_string(),
        }));
    }

    let url = match res.get("url") {
        Some(url) => url,
        None => {
            return Err(Json(Response {
                status_code: 500,
                message: "URL não encontrada".to_string(),
            }))
        }
    };

    let clicks: i64 = match conn
        .hincr(&code, "clicks", 1)
        .await
    {
        Ok(value) => value,
        Err(_) => {
            return Err(Json(Response {
                status_code: 500,
                message: "Erro ao incrementar clicks".to_string(),
            }))
        }
    };

    Ok(Redirect::temporary(url))
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