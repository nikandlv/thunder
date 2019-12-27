use actix_web::{web, get, Responder};
#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

pub fn get(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/").service(index)
    );
}