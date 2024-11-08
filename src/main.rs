use actix_web::{web, App, HttpServer, HttpResponse};
use rusqlite::{params, Connection};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// Function to establish a SQLite connection
fn get_db_connection() -> Connection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::open(db_url).expect("Failed to open database")
}

// Handler to get all users
async fn get_users() -> HttpResponse {
    let conn = get_db_connection();
    let mut stmt = conn.prepare("SELECT id, name, email FROM users").unwrap();
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }).unwrap();

    let users: Vec<User> = user_iter.filter_map(Result::ok).collect();
    HttpResponse::Ok().json(users)
}

// Handler to create a new user
async fn create_user(user: web::Json<User>) -> HttpResponse {
    let conn = get_db_connection();
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        params![user.name, user.email],
    ).expect("Failed to insert user");

    HttpResponse::Created().json(user.into_inner())
}

// Main function to run the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
