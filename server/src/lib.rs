#![forbid(unsafe_code)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod db;
pub mod models;

use models::users::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn fetch_user(conn: &PgConnection, user_id: i32) -> User {
    use schema::users::dsl::*;

    users.filter(id.eq(user_id))
        .get_result(conn)
        .expect("Error loading users")
}

pub fn create_user(conn: &PgConnection, username: &str) -> User {
    let new_user = NewUser { username };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn update_user(conn: &PgConnection, id: i32, user: &UpdateUser) -> User {
    diesel::update(schema::users::table.find(id))
        .set(user)
        .get_result(conn)
        .expect(&format!("Unable to find user {}", id))
}
