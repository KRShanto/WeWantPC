use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key},
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use wewantpc::routes::{login_route, logout_route, register_route, whoami_route};
use wewantpc::DbPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //info, debug, warn, error
    env_logger::init();
    dotenv::dotenv().ok().unwrap();

    // set up database connection pool
    let dbconnection = std::env::var("DATABASE_URL").expect("DATABASE_URL not found!");

    // PostgreSQL connnection manager
    let manager = ConnectionManager::<PgConnection>::new(dbconnection);

    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    // customize session and cookie expiration
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .service(web::resource("/").to(|| async { "Hello rust devs!" }))
            .service(register_route)
            .service(login_route)
            .service(logout_route)
            .service(whoami_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
