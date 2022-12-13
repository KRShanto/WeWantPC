pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

pub const SESSION_NAME: &str = "user_id";

pub use serde::{Deserialize, Serialize};

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Roles for maintaining an ecommerce site
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
    Staff,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" => Role::Admin,
            "user" => Role::User,
            "staff" => Role::Staff,
            _ => Role::User,
        }
    }
}

impl From<Role> for String {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => "admin".to_string(),
            Role::User => "user".to_string(),
            Role::Staff => "staff".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    AlreadyExists,
    Success,
    NotFound,
    ServerError,
}
