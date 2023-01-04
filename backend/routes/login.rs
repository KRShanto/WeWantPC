use crate::models::users::User;
use crate::Response;
use crate::SESSION_NAME;
use actix_session::Session;
use actix_web::{post, web, web::block, Responder};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use diesel::prelude::*;
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    email: String,
    password: String,
}

/// Login a user
///
/// It accepts a json object of the [`LoginUser`] struct.
/// It will first check if the user exists.
/// Then it will verify the password using argon2.
/// After that it will log the user in using the session.
#[post("/login")]
pub async fn login_route(
    session: Session,
    pool: web::Data<crate::DbPool>,
    user_to_login: web::Json<LoginUser>,
) -> impl Responder {
    let user_to_login = user_to_login.into_inner();
    let user = block(move || {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        users
            .filter(email.eq(&user_to_login.email))
            .first::<User>(&mut conn)
            .ok()
    })
    .await
    .unwrap();

    if user.is_none() {
        warn!("No user present");
        // return HttpResponse::Ok().json(ResponseType::NotFound);
        return Response::not_found().msg("No user present").send();
    }

    let user = user.unwrap();

    let password = block({
        let user_password = user.password.clone();

        move || {
            let hash = PasswordHash::new(&user_password).unwrap();
            Argon2::default()
                .verify_password(user_to_login.password.as_bytes(), &hash)
                .is_ok()
        }
    })
    .await
    .unwrap();

    if !password {
        warn!("Password is incorrect");
        // return HttpResponse::Ok().json(ResponseType::NotFound);
        return Response::incorrect_password().send();
    }

    session.insert(SESSION_NAME, user.id).unwrap();

    // HttpResponse::Ok().json(ResponseType::Success)
    Response::success()
        .msg("Logged in successfully")
        .data(user)
        .send()
}
