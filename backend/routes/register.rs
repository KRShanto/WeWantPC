use crate::models::users::{NewUser, User};
use crate::utils::create_user;
use crate::ResponseType;
use crate::Role;
use crate::SESSION_NAME;
use actix_session::Session;
use actix_web::{
    post,
    web::{self, block},
    HttpResponse, Responder,
};

use diesel::prelude::*;

/// Register a new user
///
/// It accepts a json object of the [`NewUser`] struct.
/// It will first check if the user already exists.
/// Then it will hash the password using argon2.
/// Then it will insert the user into the database.
/// After that it will log the user in using the session.
// TODO: validate the email and password
#[post("/register")]
pub async fn register_route(
    session: Session,
    pool: web::Data<crate::DbPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let new_user = user.into_inner();
    let user = {
        use crate::schema::users::dsl::*;
        let user_email = new_user.email.clone();
        let pool = pool.clone();

        block(move || {
            let mut conn = pool.get().unwrap();

            users
                .filter(email.eq(&user_email))
                .first::<User>(&mut conn)
                .ok()
        })
        .await
        .unwrap()
    };

    if user.is_some() {
        return HttpResponse::Ok().json(ResponseType::AlreadyExists);
    }

    let new_user = NewUser {
        role: String::from(Role::User),
        ..new_user
    };

    match create_user((*pool.into_inner()).clone(), new_user).await {
        Ok(user) => {
            session.insert(SESSION_NAME, user.id).unwrap();
            HttpResponse::Ok().json(ResponseType::Success)
        }
        Err(err) => {
            println!("Error: {}", err);

            HttpResponse::Ok().json(ResponseType::ServerError)
        }
    }
}
