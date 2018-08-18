extern crate storm;

#[macro_use]
extern crate diesel;

use storm::db::establish_connection;
use storm::models::users::User;
use diesel::prelude::*;
use std::io::stdin;

fn main() {
    use storm::schema::users::dsl::*;

    let connection = establish_connection();

    println!("Username: ");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)];

    let user = storm::create_user(&connection, name);

    let results = users.limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("--------");
        println!("{}", user.username);
    }
}
