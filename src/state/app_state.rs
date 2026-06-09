use deadpool_redis::{Connection, Pool};

use crate::reponses::response::Response;

use axum::Json;

#[derive(Clone)]
pub struct AppState {
    pub redis: Pool,
}

impl AppState{
    pub async  fn get_pool(&self) -> Result<Connection, Json<Response>>{
        match self
        .redis
        .get()
        .await {

        Ok(t) => Ok(t),
        Err(e) => {
            Err(Json(Response {
                status_code: 500,
                message: format!("erro redis: {:?}", e),
            }))
        }
    }
    }
}
