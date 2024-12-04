use actix_web::{web, HttpResponse, Responder};

use crate::models::{AppInfo, User};

// Handler for the root endpoint
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Quira.sh!")
}

// Handler for the `/post` endpoint
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Handler for `/status` endpoint
pub async fn status() -> impl Responder {
    let info = AppInfo {
        name: "Hello Actix App".to_string(),
        version: "1.0.0".to_string(),
    };
    HttpResponse::Ok().json(info)
}

// Handler for dynamic user greeting `/a/{name}`
pub async fn set_name(name: web::Path<String>) -> impl Responder {
    let user = User {
        name: name.to_string(),
    };
    HttpResponse::Ok().json(user)
}
