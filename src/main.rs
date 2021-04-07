use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::Serialize;

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("Paul Halmos: in his own words"),
            link: String::from("https://example.com"),
            author: String::from("John Ewing")
        },
        Post {
            title: String::from("Map location and dimensional definition of subsurface caverns"),
            link: String::from("https://example.com"),
            author: String::from("James E. Wolfe")
        },
    ];

    data.insert("title", "Hacker Clone");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
