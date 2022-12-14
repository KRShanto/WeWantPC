pub mod create_product;
pub mod create_staff;
pub mod login;
pub mod logout;
pub mod register;
pub mod whoami;

// pub use create_product::create_product_route;
pub use create_staff::create_staff_route;
pub use login::login_route;
pub use logout::logout_route;
pub use register::register_route;
pub use whoami::whoami_route;
