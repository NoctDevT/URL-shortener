use actix_web::{web, HttpResponse, Responder};
use crate::database::get_link;
use crate::state::State;
use tracing::info;

pub async fn redirect(
    state: web::Data<State>,
    alias: web::Path<String>, 
) -> impl Responder {
    
    let alias = alias.into_inner(); 

    let client = state.database_client().await.unwrap();
    match get_link(&client, &alias).await {
        Ok(url) => {
            info!("Redirecting your url {} to {}", alias, url);
            HttpResponse::TemporaryRedirect()
                .insert_header(("Location", url))
                .finish()
        }
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Alias not found" })),
    }
}
