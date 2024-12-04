//! Main entry point for the Actix Web application.
//!
//! This module contains the main function which serves as the entry point for the Actix Web application. 
//! It initializes the server and configures the routes through modular components.

mod handlers;  // Module for handling HTTP requests.
mod models;    // Module for defining data structures.
mod routes;    // Module for configuring the application's routes.
mod tests;     // Module containing unit and integration tests for the application.

use actix_web::{App, HttpServer};

/// The main function that starts the Actix Web server.
///
/// # Description
/// This is the entry point for the application. It sets up the Actix Web server, 
/// configures routes, and starts listening on the specified address. The `HttpServer` 
/// is configured using `App::new()` and the modular route configuration from `routes::config_routes`.
///
/// # Execution Flow
/// 1. The server is initialized with `HttpServer::new`, which accepts a closure that
///    configures the Actix application.
/// 2. The `App::new()` is used to create the application instance, and the route configuration is applied 
///    by calling `.configure(routes::config_routes)`.
/// 3. The server listens on `127.0.0.1:8080` and begins accepting incoming HTTP requests.
///
/// # Example
/// ```rust
/// use actix_web::HttpServer;
/// use my_app::main;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     main().await
/// }
/// ```
///
/// # Panics
/// - If the server fails to bind to the specified address (`127.0.0.1:8080`), the program will panic.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config_routes) // Use modular route configuration
    })
    .bind(("127.0.0.1", 8080))? // Bind the server to address 127.0.0.1:8080
    .run() // Start the server to accept requests
    .await // Await the server's execution, blocking the main thread
}
