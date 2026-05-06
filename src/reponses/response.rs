use serde::{ Serialize};

#[derive(Serialize)]
pub struct ResponseNewShort{
    pub status_code: u32,
    pub short_code: String
}

#[derive(Serialize)]
pub struct ResponseGetShorten{
    pub url: String
}
#[derive(Serialize)]
pub struct ResponseErro{
    pub status_code: u32,
    pub message: String,
}