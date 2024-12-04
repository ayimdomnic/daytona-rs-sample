use actix_web::web;

use crate::handlers;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(handlers::hello))
            .route("/post", web::post().to(handlers::echo))
            .route("/status", web::get().to(handlers::status))
            .route("/a/{name}", web::get().to(handlers::set_name)),
    );
}
