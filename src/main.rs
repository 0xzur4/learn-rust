#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status;
use mysql::*;
use mysql::prelude::*;
use std::env;
use rocket::http::Status;
use dotenv::dotenv;
use rocket_cors::CorsOptions;
use core::result::Result::Ok;


#[get("/")]
fn index() -> &'static str {
    "Welcome to API"
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<usize>, // Menambahkan id pada User
    name: String,
    email: String,
}

fn establish_connection() -> PooledConn {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opts = Opts::from_url(&database_url).expect("Failed to parse database URL");
    let pool = Pool::new(opts).expect("Failed to create pool.");
    pool.get_conn().expect("Failed to get connection.")
}

// Membuat data baru
#[post("/user", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Result<status::Created<&'static str>, status::Custom<String>> {
    let mut conn = establish_connection();

    match conn.exec_drop(
        "INSERT INTO users (name, email) VALUES (:name, :email)",
        params! {
            "name" => &user.name,
            "email" => &user.email,
        },
    ) {
        Ok(_) => Ok(status::Created::new("/user")),
        Err(e) => Err(status::Custom(Status::InternalServerError, format!("Failed to insert user: {}", e))),
    }
}

// Membaca data yang sudah ada
#[get("/users")]
fn read_users() -> Json<Vec<User>> {
    let mut conn = establish_connection();
    
    let users = conn.query_map(
        "SELECT id, name, email FROM users",
        |(id, name, email)| User { id: Some(id), name, email },
    ).expect("Failed to read users");

    Json(users)
}

// update/edit data
#[put("/user/<id>", format = "json", data = "<user>")]
fn update_user(id: u32, user: Json<User>) -> Result<status::Custom<String>, status::Custom<String>> {
    let mut conn = establish_connection();

    match conn.exec_drop(
        "UPDATE users SET name = :name, email = :email WHERE id = :id",
        params! {
            "id" => id,
            "name" => &user.name,
            "email" => &user.email,
        },
    ) {
        Ok(_) => Ok(status::Custom(Status::Ok, "User updated successfully".to_string())),
        Err(e) => Err(status::Custom(Status::InternalServerError, format!("Failed to update user: {}", e))),   
    }
}


// Menghapus data berdasarkan ID
#[delete("/user/<id>")]
fn delete_user(id: usize) -> Result<status::NoContent, status::Custom<String>> {
    let mut conn = establish_connection();

    match conn.exec_drop(
        "DELETE FROM users WHERE id = :id",
        params! {
            "id" => id,
        },
    ) {
        Ok(_) => Ok(status::NoContent),
        Err(e) => Err(status::Custom(Status::InternalServerError, format!("Failed to delete user: {}", e))),
    }
}


#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .to_cors()
        .expect("Error creating CORS middleware");

    rocket::build()
    .configure(rocket::Config{
        port: 8080,
        ..Default::default()
    })
        .attach(cors)
        .mount("/", routes![index, create_user, read_users, delete_user, update_user])
}
