use actix_web::{web, HttpResponse, Responder};
use crate::database::delete_link;
use crate::state::State;

//returns 404 if the alias isn't found otherwise gives Ok status  
pub async fn delete_url(
    state: web::Data<State>,
    alias: web::Path<String>, 
) -> impl Responder {
    let alias = alias.into_inner(); 

    //gets db connection
    let client = state.database_client().await.unwrap();
    match delete_link(&client, &alias).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "URL deleted" })),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Alias not found" })),
    }
}
