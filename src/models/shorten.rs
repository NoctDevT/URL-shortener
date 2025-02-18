use serde::{Deserialize, Serialize};

//abstracting for modularity and for option to scale codebase

#[derive(Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    pub alias: Option<String>,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub alias: String,
    pub url: String,
}
