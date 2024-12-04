use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/post")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/a/{name}")]
async fn set_name(name: web::Path<String>) -> impl Responder {
    let obj = User {
        name: name.to_string(),
    };
    web::Json(obj)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(set_name)
            .route("/hello", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use super::*;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world!");
    }

    #[actix_web::test]
    async fn test_echo() {
        let app = test::init_service(App::new().service(echo)).await;
        let req = test::TestRequest::post()
            .uri("/post")
            .set_payload("This is a test")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "This is a test");
    }

    #[actix_web::test]
    async fn test_manual_hello() {
        let app = test::init_service(App::new().route("/hey", web::get().to(manual_hello))).await;
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hey there!");
    }

    #[actix_web::test]
    async fn test_not_found() {
        let app = test::init_service(App::new()).await;
        let req = test::TestRequest::get().uri("/non-existent").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404);
    }
}
