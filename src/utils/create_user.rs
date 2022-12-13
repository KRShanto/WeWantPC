use crate::models::{NewUser, User};
use crate::DbPool;
use actix_web::web::block;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::prelude::*;

pub async fn create_user(pool: DbPool, new_user: NewUser) -> Result<User, diesel::result::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    // hashed password
    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = NewUser {
        name: new_user.name,
        email: new_user.email,
        password: password_hash,
        role: new_user.role,
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
        Ok(user) => Ok(user),
        Err(err) => Err(err),
    }
}
