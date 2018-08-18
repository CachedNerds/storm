extern crate storm;

#[macro_use]
extern crate diesel;

use storm::db::establish_connection;
use storm::models::users::*;
use diesel::prelude::*;
use std::io::stdin;

fn main() {
    use storm::schema::users::dsl::*;

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

    storm::update_user(&connection, user_id, &updated_user);

    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("--------");
        println!("{}", user.username);
    }
}
