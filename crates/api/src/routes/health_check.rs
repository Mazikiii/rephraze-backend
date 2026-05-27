use actix_web::{App, HttpRequest, HttpResponse, Responder, web};
pub async fn health_check() -> impl Responder {
    HttpRespond::Ok().finish()
}
