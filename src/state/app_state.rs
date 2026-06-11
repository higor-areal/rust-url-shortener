use crate::{
    repositories::redis_repository::RedisStore
};

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisStore,
}
