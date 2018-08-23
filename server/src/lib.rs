#![forbid(unsafe_code)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod user;
pub mod db;
pub mod schema;
