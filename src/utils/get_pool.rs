use crate::DbPool;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub fn get_pool() -> DbPool {
    // set up database connection pool
    let dbconnection = std::env::var("DATABASE_URL").expect("DATABASE_URL not found!");

    // PostgreSQL connnection manager
    let manager = ConnectionManager::<PgConnection>::new(dbconnection);

    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
