use actix_web::{web, HttpResponse, Responder};
use crate::database::create_link;
use crate::state::State;
use crate::models::shorten::{ShortenRequest, ShortenResponse};
use random_string::generate; 



//using package to generate random alias using random_string crate 
fn generate_alias() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; 
    generate(6, charset)
}


pub async fn shorten_url(
    state: web::Data<State>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    let alias = req.alias.clone().unwrap_or_else(generate_alias);

    let client = match state.database_client().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Database error" })),
    };

    match create_link(&client, &alias, &req.url).await {
        Ok(_) => HttpResponse::Created().json(ShortenResponse {
            alias: alias.clone(),
            url: req.url.clone(),
        }),
        Err(_) => HttpResponse::Conflict().json(serde_json::json!({ "error": "Alias already exists" })),
    }
}
