use crate::DbPool;
use crate::Permission;
use crate::ResponseType;
use crate::Role;
use crate::{
    models::users::{NewUser, User},
    utils::create_user,
};
use actix_web::{
    post,
    web::{block, Data, Json},
    HttpResponse, Responder,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Body {
    user: NewUser,
    permissions: Vec<Permission>,
}

#[post("/create-staff")]
pub async fn create_staff_route(body: Json<Body>, pool: Data<DbPool>) -> impl Responder {
    let user_to_staff = body.user.clone();
    let permissions = body.permissions.clone();
    let pool = (*pool.into_inner()).clone();

    // Check if user already exists
    let user_result = {
        // let mut conn = conn.clone();
        let mut conn = pool.get().unwrap();
        let user_email = user_to_staff.email.clone();

        block(move || {
            use crate::schema::users::dsl::*;
            users.filter(email.eq(&user_email)).first::<User>(&mut conn)
        })
        .await
        .unwrap()
    };

    // Check if the user's role is user or not
    if user_result.is_ok() {
        let user = user_result.unwrap();

        if user.role == String::from(Role::User) {
            // update the user's role to staff
            let staff_user = block(move || {
                use crate::schema::users::dsl::*;
                let mut conn = pool.get().unwrap();

                diesel::update(users.filter(email.eq(&user.email)))
                    .set(role.eq(String::from(Role::Staff(permissions))))
                    .get_result::<User>(&mut conn)
            })
            .await
            .unwrap();

            match staff_user {
                Ok(_) => HttpResponse::Ok().json(ResponseType::Success),
                Err(_) => HttpResponse::Ok().json(ResponseType::ServerError),
            }
        } else {
            // user already exists
            // user is either staff or admin
            HttpResponse::Ok().json(ResponseType::AlreadyExists)
        }
    } else {
        // create a new user
        match create_user(
            pool,
            NewUser {
                name: user_to_staff.name,
                email: user_to_staff.email,
                password: user_to_staff.password,
                role: String::from(Role::Staff(permissions)),
            },
        )
        .await
        {
            Ok(_) => HttpResponse::Ok().json(ResponseType::Success),
            Err(_) => HttpResponse::InternalServerError().json(ResponseType::ServerError),
        }
    }
}
