use models::users::*;
use diesel::{ self, prelude::*, pg::PgConnection };
use schema;

pub fn create(conn: &PgConnection, username: &str) -> User {
    let new_user = NewUser { username };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn fetch(conn: &PgConnection, user_id: i32) -> User {
    use schema::users::dsl::*;

    users.find(user_id)
         .get_result(conn)
         .expect("Error loading users")
}

pub fn update(conn: &PgConnection, user_id: i32, user: &UpdateUser) -> User {
    use schema::users::dsl::*;

    diesel::update(users.find(user_id))
        .set(user)
        .get_result(conn)
        .expect(&format!("Unable to find user {}", user_id))
}

pub fn remove(conn: &PgConnection, user_id: i32) {
    use schema::users::dsl::*;

    diesel::delete(users.find(user_id))
        .execute(conn)
        .expect(&format!("Unable to find user {}", user_id));
}
