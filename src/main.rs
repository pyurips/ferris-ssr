use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use ssr_rust_template::Component;


const COMPONENTS_DIR: &str = "./components";

#[get("/")]
async fn render_example() -> impl Responder {
    let component = Component::new(COMPONENTS_DIR);

    let mut increase_button_data = HashMap::new();

    increase_button_data.insert("color".to_string(), "bg-stone-100".to_string());
    increase_button_data.insert("text".to_string(), "Increase the counter".to_string());

    let increase_particle_button = component.spawn("particles/increase_button", Some(&increase_button_data));
    let htmx_particle_button = component.spawn("particles/htmx_button", None);

    let mut components = HashMap::new();
    components.insert("button".to_string(), increase_particle_button.unwrap());
    components.insert("htmx_button".to_string(), htmx_particle_button.unwrap());

    let main_component = component.spawn("main", Some(&components));
    HttpResponse::Ok()
        .content_type("text/html")
        .body(main_component.unwrap())
}

#[get("/htmx-request")]
async fn htmx_example() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<p class=\"text-stone-100\">Request made! ;)</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(render_example)
            .service(htmx_example)
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
