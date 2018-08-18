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

pub fn create_user<'a>(conn: &PgConnection, username: &'a str) -> User {
    use schema::users;

    let new_user = NewUser { username };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}