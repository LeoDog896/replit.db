use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    database: HashMap<String, String>
}

#[get("/{key}")]
async fn get(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/{key}")]
async fn del(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/")]
async fn add(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(del)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}