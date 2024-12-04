#[cfg(test)]
mod test {
    use actix_web::{test, web, App};

    use crate::handlers;

    #[cfg(test)]
    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().route("/", web::get().to(handlers::hello))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, Quira.sh!");
    }

    #[actix_web::test]
    async fn test_status() {
        let app =
            test::init_service(App::new().route("/status", web::get().to(handlers::status))).await;
        let req = test::TestRequest::get().uri("/status").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let expected = r#"{"name":"Hello Actix App","version":"1.0.0"}"#;
        assert_eq!(body, expected);
    }

    #[actix_web::test]
    async fn test_set_name() {
        let app =
            test::init_service(App::new().route("/a/{name}", web::get().to(handlers::set_name)))
                .await;
        let req = test::TestRequest::get().uri("/a/Dom").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let expected = r#"{"name":"Dom"}"#;
        assert_eq!(body, expected);
    }

    #[actix_web::test]
    async fn test_not_found() {
        let app = test::init_service(App::new()).await;
        let req = test::TestRequest::get().uri("/notfound").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404);
    }
}
