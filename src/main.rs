use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod default;
use default::get_component;

#[get("/")]
async fn render_example() -> impl Responder {
    let mut button_data = HashMap::new();
    button_data.insert("color".to_string(), "bg-stone-100".to_string());
    button_data.insert("text".to_string(), "Increase the counter".to_string());
    let particle_button = get_component("particles/increase_button", None, Some(&button_data));

    let mut components = HashMap::new();
    components.insert("button".to_string(), particle_button.unwrap());

    let main_component = get_component("main", Some(&components), None);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(main_component.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(render_example)
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
