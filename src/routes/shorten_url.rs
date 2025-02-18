use actix_web::{web, Responder};
use crate::database::create_link;
use crate::state::State;
use crate::models::shorten::{ShortenRequest};

pub async fn shorten_url(
    state: web::Data<State>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
   
}

