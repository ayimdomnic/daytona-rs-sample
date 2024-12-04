//! Handlers for the Actix Web application.
//!
//! This module provides handlers for various routes in the application,
//! including a root endpoint, an echo endpoint, an application status endpoint,
//! and a dynamic user greeting endpoint. These handlers demonstrate how to
//! structure a basic Actix Web application and process incoming requests.

use actix_web::{web, HttpResponse, Responder};
use crate::models::{AppInfo, User};

/// Handler for the root endpoint `/`.
///
/// # Description
/// Responds with a simple "Hello, Quira.sh!" message to demonstrate
/// a basic HTTP GET request handler.
///
/// # Returns
/// An HTTP 200 response with a plain text body.
///
/// # Example
/// ```bash
/// curl http://localhost:8080/
/// # Response:
/// # Hello, Quira.sh!
/// ```
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Quira.sh!")
}

/// Handler for the `/post` endpoint.
///
/// # Description
/// Echoes back the raw request body received in a POST request.
/// This demonstrates how to handle POST requests and process the request body.
///
/// # Arguments
/// - `req_body` - The raw request body as a `String`.
///
/// # Returns
/// An HTTP 200 response containing the echoed request body.
///
/// # Example
/// ```bash
/// curl -X POST -d "Sample Data" http://localhost:8080/post
/// # Response:
/// # Sample Data
/// ```
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// Handler for the `/status` endpoint.
///
/// # Description
/// Returns a JSON response containing application metadata, such as
/// the application name and version. This demonstrates JSON serialization
/// and response handling.
///
/// # Returns
/// An HTTP 200 response with a JSON body containing app metadata.
///
/// # Example
/// ```bash
/// curl http://localhost:8080/status
/// # Response:
/// # {"name":"Hello Actix App","version":"1.0.0"}
/// ```
pub async fn status() -> impl Responder {
    let info = AppInfo {
        name: "Hello Actix App".to_string(),
        version: "1.0.0".to_string(),
    };
    HttpResponse::Ok().json(info)
}

/// Handler for the dynamic user greeting endpoint `/a/{name}`.
///
/// # Description
/// Extracts the dynamic `name` parameter from the URL path and
/// responds with a JSON object containing the user's name. This demonstrates
/// URL path parameter extraction and JSON serialization.
///
/// # Arguments
/// - `name` - A `web::Path<String>` representing the name extracted from the URL.
///
/// # Returns
/// An HTTP 200 response with a JSON object containing the name.
///
/// # Example
/// ```bash
/// curl http://localhost:8080/a/Dom
/// # Response:
/// # {"name":"Dom"}
/// ```
pub async fn set_name(name: web::Path<String>) -> impl Responder {
    let user = User {
        name: name.to_string(),
    };
    HttpResponse::Ok().json(user)
}
