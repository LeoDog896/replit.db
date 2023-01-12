use actix_storage::Storage;
use actix_storage_hashmap::HashMapStore;
use actix_web::{error, Error, delete, get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/{key}")]
async fn get(storage: Storage, path: web::Path<(String,)>) -> Result<impl Responder, Error> {
    let value = storage.get_bytes(&path.0).await?;
    if let Some(value) = value {
        Ok(HttpResponse::Ok().body(value))
    } else {
        Err(error::ErrorNotFound(""))
    }
}

#[delete("/{key}")]
async fn del(storage: Storage, path: web::Path<(String,)>) -> Result<impl Responder, Error> {
    if storage.contains_key(&path.0).await? {
        storage.delete(&path.0).await?;
        Ok(HttpResponse::NoContent())
    } else {
        Err(error::ErrorNotFound(""))
    }
}

#[post("/")]
async fn add(storage: Storage, req_body: String) -> Result<impl Responder, Error> {
    if req_body.is_empty() {
        return Err(error::ErrorBadRequest(""));
    }

    let mut body_set = req_body.split('=');
    let key = body_set.next().ok_or_else(|| error::ErrorBadRequest(""))?;
    let value = body_set.next().ok_or_else(|| error::ErrorBadRequest(""))?;
    storage.set_bytes(key, value).await?;
    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = HashMapStore::new();

    let storage = Storage::build().store(store).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .service(get)
            .service(del)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
