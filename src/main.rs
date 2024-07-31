use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn render_example() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>Go to Rust</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(render_example))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
