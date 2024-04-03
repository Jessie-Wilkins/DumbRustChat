use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use llm_inference_service;

#[post("/infer")]
async fn infer(req_body: String) -> impl Responder {
    
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(infer)
    })
    .bind(("127.0.0.1", 7001))?
    .run()
    .await
}