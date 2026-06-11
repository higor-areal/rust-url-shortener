use deadpool_redis::{Connection, Pool};

use crate::{
    repositories::redis_repository::RedisStore
};

use axum::Json;

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisStore,
}
