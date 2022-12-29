use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key},
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};

use log::info;
use wewantpc::{
    handle_user_input::handle_user_input,
    routes::{
        create_product_route, create_staff_route, login_route, logout_route, register_route,
        whoami_route,
    },
    utils::{get_input, get_pool},
};
// TODO add chatting feature for the admin and staff
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok().unwrap();

    let pool = get_pool();

    // spawn a new tokio task to listen user input
    info!("Listening user input... (type `h` for help)");
    {
        let pool = pool.clone();
        tokio::spawn(async move {
            loop {
                let input = get_input(None);
                handle_user_input(input.trim(), pool.clone()).await;
            }
        });
    }

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
            .service(create_staff_route)
            .service(create_product_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
