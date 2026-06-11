use serde::{ Serialize};

#[derive(Serialize)]
pub struct ResponseNewShort{
    pub status_code: u32,
    pub short_code: String
}

#[derive(Serialize)]
pub struct Response{
    pub status_code: u32,
    pub message: String,
}
