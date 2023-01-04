pub mod handle_user_input;
pub mod models;
pub mod routes;
pub mod schema;
pub mod types;
pub mod utils;

pub const SESSION_NAME: &str = "user_id";

pub use serde::{Deserialize, Serialize};
pub use types::*;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
