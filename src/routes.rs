//! Route configuration for the Actix Web application.
//!
//! This module defines and configures the routes for the application,
//! linking endpoints to their respective handlers. The routes are organized
//! using `actix_web::web::scope` for better modularity and maintainability.

use actix_web::web;
use crate::handlers;

/// Configures the application routes.
///
/// # Description
/// This function sets up the routing for the Actix Web application by associating
/// HTTP methods and paths with their corresponding handler functions. The routes
/// are grouped under a `web::scope` for a modular structure.
///
/// # Arguments
/// - `cfg`: A mutable reference to `web::ServiceConfig` used to register the routes.
///
/// # Routes
/// - `/` (GET): Calls the [`handlers::hello`] function to return a simple greeting.
/// - `/post` (POST): Calls the [`handlers::echo`] function to echo back the request body.
/// - `/status` (GET): Calls the [`handlers::status`] function to return application metadata as JSON.
/// - `/a/{name}` (GET): Calls the [`handlers::set_name`] function to return a dynamic user greeting in JSON.
///
/// # Example
/// ```rust
/// use actix_web::{web, App, HttpServer};
/// use my_app::routes::config_routes;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new().configure(config_routes)
///     })
///     .bind(("127.0.0.1", 8080))?
///     .run()
///     .await
/// }
/// ```
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("") // Base scope for routes.
            .route("/", web::get().to(handlers::hello)) // Root route.
            .route("/post", web::post().to(handlers::echo)) // Echo route.
            .route("/status", web::get().to(handlers::status)) // Application status route.
            .route("/a/{name}", web::get().to(handlers::set_name)), // Dynamic user greeting route.
    );
}
