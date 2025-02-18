use actix_web::{web, Responder};
use crate::database::get_link;
use crate::state::State;

pub async fn redirect(
    state: web::Data<State>,
    alias: web::Path<String>, 
) -> impl Responder {
    

}
