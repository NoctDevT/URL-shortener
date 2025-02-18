use actix_web::{web, HttpResponse, Responder};
use crate::database::delete_link;
use crate::state::State;

pub async fn delete_url(
    state: web::Data<State>,
    alias: web::Path<String>, 
) -> impl Responder {
    let alias = alias.into_inner(); 

    println!("Attempting to delete alias: {}", alias);

    let client = match state.database_client().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Failed to connect to the database" })),
    };

    match delete_link(&client, &alias).await {
        Ok(_) => {
            let message = format!("URL deleted: {}", alias);
            println!("Successfully deleted alias: {}", alias);
            HttpResponse::Ok().json(serde_json::json!({ "message": message })) 
        },
        Err(_) => {
            println!("Alias not found: {}", alias);
            HttpResponse::NotFound().json(serde_json::json!({ "error": "Alias not found" }))
        },
    }
}
