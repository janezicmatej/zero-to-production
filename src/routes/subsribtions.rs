use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
