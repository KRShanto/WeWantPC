use crate::models::users::NewUser;
use crate::utils::create_user;
use crate::utils::get_input;
use crate::DbPool;

pub async fn create_admin(pool: DbPool) {
    let name = get_input(Some("Enter the name"));
    let email = get_input(Some("Enter the email"));
    let password = get_input(Some("Enter the password"));

    let new_user = NewUser {
        name,
        email,
        password,
        role: String::from(crate::Role::Admin),
    };

    let created_user = create_user(pool, new_user).await.unwrap();

    println!("The admin user has been created!");
    println!("Admin: {:#?}", created_user);
}
