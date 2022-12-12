use crate::models::{NewUser, User};
use crate::ResponseType;
use crate::Role;
use crate::SESSION_NAME;
use actix_session::Session;
use actix_web::{
    post,
    web::{self, block},
    HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::prelude::*;

/// Register a new user
///
/// It accepts a json object of the [`NewUser`] struct.
/// It will first check if the user already exists.
/// Then it will hash the password using argon2.
/// Then it will insert the user into the database.
/// After that it will log the user in using the session.
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

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = NewUser {
        name: new_user.name,
        email: new_user.email,
        password: password_hash,
        role: String::from(Role::User),
    };

    let created_user = block(move || {
        use crate::schema::users;
        let mut conn = pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&mut conn)
    })
    .await
    .unwrap();

    match created_user {
        Ok(user) => {
            session.insert(SESSION_NAME, user.id).unwrap();
            HttpResponse::Ok().json(ResponseType::Success)
        }
        Err(err) => {
            println!("Error: {}", err);

            HttpResponse::Ok().json(ResponseType::ServerError)
        }
    }

    // session.insert(SESSION_NAME, user.id).unwrap();

    // HttpResponse::Ok().json(ResponseType::Success)
}
