mod create_admin;

use create_admin::create_admin;

pub async fn handle_user_input(input: &str, pool: crate::DbPool) {
    match input {
        "h" | "help" => {
            println!("h: help   ->   show this help message");
            println!("q: quit   ->   exit the server");
            println!("exit   ->   exit the server");
            println!("create-admin   ->   create an admin user");
        }
        "q" | "quit" | "exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        "create-admin" => {
            println!("Creating admin user...");
            create_admin(pool).await;
        }
        _ => {
            println!("Invalid input! Type `h` for help.");
        }
    }
}
