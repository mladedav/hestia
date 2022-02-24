#[macro_use]
extern crate diesel;
extern crate dotenv;

mod appconfig;
mod authorization;

mod db;
mod handlers;

use actix_web::dev::Server;
use actix_web::error::InternalError;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer, HttpResponse, guard};

#[cfg(debug_assertions)]
const ADDRESS: &str = "127.0.0.1:8080";
#[cfg(not(debug_assertions))]
const ADDRESS: &str = "0.0.0.0:8080";

pub fn start() -> std::io::Result<Server> {
    let server = HttpServer::new(||
            App::new()
                .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                    InternalError::from_response(
                        "",
                        HttpResponse::BadRequest()
                            .content_type("application/json")
                            .body(format!(r#"{{"error":"{}"}}"#, err)),
                    )
                    .into()
                }))
                .app_data(Data::new(appconfig::handlebars()))
                .default_service(
                    // 404 or 405
                    web::resource("")
                        .route(web::get().to(handlers::default::p404))
                        .route(
                            web::route()
                                .guard(guard::Not(guard::Get()))
                                .to(HttpResponse::MethodNotAllowed),
                        )
                )
                .configure(appconfig::config_app)
        )
        .bind(ADDRESS)?
        .run();
    Ok(server)
}
