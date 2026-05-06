use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub original_url: String,
    pub clicks: u32,
}

#[derive(Deserialize, Serialize)]
pub struct NewLink {
    pub url: String,
}