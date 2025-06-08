mod items;

use actix_web::{web, HttpResponse, get};
use serde_json::json;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(root)
            .service(
                web::scope("/api")
                    .configure(items::configure_routes)
            )
    );
}

#[get("/")]
async fn root() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "Welcome to the RESTful API"
    }))
}
