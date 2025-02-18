// the server

use std::net::TcpListener;
use std::time::Duration;

use actix_web::dev::Server;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};

use crate::state::State;


//abstracting routes into their own folder

use crate::routes::shorten_url::shorten_url;
use crate::routes::redirect_url::redirect;
use crate::routes::delete_url::delete_url;

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/shorten_url", web::post().to(shorten_url))
       .route("/{alias}", web::get().to(redirect))
       .route("/{alias}", web::delete().to(delete_url));
}

async fn not_found_handler(_request: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" }))
}

pub fn listen(listener: TcpListener, state: State) -> std::io::Result<Server> {
    let state = web::Data::new(state);
    let create_app = move || {
        let app = App::new().app_data(state.clone());
            app
                .wrap(tracing_actix_web::TracingLogger::default())
                .wrap(Logger::new(r#"%a "%r" %s %b (%{Content-Length}i %{Content-Type}i) "%{Referer}i" "%{User-Agent}i" %T"#))
                .wrap(Compress::default())
                .wrap(NormalizePath::trim())
                .service(web::scope("/api").configure(api_config))
                .default_service(web::route().to(not_found_handler))
    };
    let server = HttpServer::new(create_app)
        .keep_alive(Duration::from_secs(60))
        .listen(listener)?
        .run();
    Ok(server)
}
