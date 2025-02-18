use actix_web::{web, HttpResponse, Responder};
use crate::database::{delete_link, get_link};
use crate::state::State;

//My workaround to fixing the delete issue 
// deleted the extra paramater from the database to fix the query

pub async fn delete_url(
    state: web::Data<State>,
    alias: web::Path<String>,
) -> impl Responder {
    let alias = alias.into_inner();
    println!("Attempting to delete alias: {}", alias);

    let client = match state.database_client().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Database connection error" }));
        }
    };
    //I'm going to first use the get_link to see if the link exists 
    match get_link(&client, &alias).await {
        Ok(_url) => {
            // Now if it exists, i'll go ahead and delete it 
            match delete_link(&client, &alias).await {
                Ok(_) => {
                    let message = format!("URL deleted: {}", alias);
                    println!("Successfully deleted alias: {}", alias);
                    HttpResponse::Ok().json(serde_json::json!({ "message": message }))
                },
                Err(_) => {
                    // If this fails, this is most likely a database error 
                    println!("Alias not found after all?: {}", alias);
                    HttpResponse::NotFound().json(serde_json::json!({ "error": "Alias was not found" }))
                }
            }
        },
        Err(_) => {
            //If it exists here, then it is not in the DB at all
            println!("Alias not found: {}", alias);
            HttpResponse::NotFound().json(serde_json::json!({ "error": "Alias has not been found" }))
        }
    }
}
