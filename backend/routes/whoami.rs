use crate::models::users::User;
use crate::Response;
use crate::SESSION_NAME;
use actix_session::Session;
use actix_web::{
    get,
    web::{block, Data},
    Responder,
};
use diesel::prelude::*;
use log::warn;

#[get("/whoami")]
pub async fn whoami_route(session: Session, pool: Data<crate::DbPool>) -> impl Responder {
    let user_id = session.get::<i32>(SESSION_NAME).unwrap();

    if user_id.is_none() {
        warn!("User not logged in");
        return Response::unauthorized().msg("User not logged in").send();
    }

    let user_id = user_id.unwrap();

    let user = block(move || {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();

        users.filter(id.eq(user_id)).first::<User>(&mut conn).ok()
    })
    .await
    .unwrap();

    if user.is_none() {
        warn!("No user present");
        return Response::unauthorized().msg("No user present").send();
    }

    Response::success()
        .data(user.unwrap())
        .msg("User is logged in")
        .send()
}
