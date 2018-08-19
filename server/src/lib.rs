#![forbid(unsafe_code)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod db;
pub mod models;
pub mod controllers;
