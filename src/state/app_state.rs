use std::collections::HashMap;
use crate::models::link::Link;

#[derive(Clone)]
pub struct AppState{
    pub map: HashMap<String, Link>,
    pub end: u32,
}

