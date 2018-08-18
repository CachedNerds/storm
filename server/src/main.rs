extern crate storm_server;
extern crate diesel;

use storm_server::db::establish_connection;
use storm_server::models::users::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("User ID: ");
    let mut user_id = String::new();
    stdin().read_line(&mut user_id).unwrap();
    let user_id = &user_id[..(user_id.len() - 1)];
    let user_id = user_id.parse::<i32>().unwrap();

    println!("New username: ");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)];

    let updated_user = UpdateUser {
        username: Some(String::from(name))
    };

    storm_server::update_user(&connection, user_id, &updated_user);

    let user = storm_server::fetch_user(&connection, user_id);

    println!("{}", user.id);
    println!("--------");
    println!("{}", user.username);
}
