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
pub struct Response{
    pub status_code: u32,
    pub message: String,
}

#[derive(Serialize)]
pub struct ResponseGetLink {
    pub code: String,
    pub original_url: String,
    pub clicks: u32,
}