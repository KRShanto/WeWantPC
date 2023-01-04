use crate::SESSION_NAME;
use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};

#[post("/logout")]
pub async fn logout_route(session: Session) -> impl Responder {
    session.remove(SESSION_NAME);
    HttpResponse::Ok()
}
