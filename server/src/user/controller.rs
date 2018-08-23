use diesel::{self, pg::PgConnection, prelude::*};
use user::model::{User, NewUser, UpdateUser};
use schema::users::table as users_table;

pub fn create(connection: &PgConnection, username: &str) -> User {
    let new_user = NewUser { username };

    diesel::insert_into(users_table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn find_one(connection: &PgConnection, user_id: i32) -> User {
    users_table
        .find(user_id)
        .get_result(connection)
        .expect("Error loading users")
}

pub fn update(connection: &PgConnection, user_id: i32, user: &UpdateUser) -> User {
    diesel::update(users_table.find(user_id))
        .set(user)
        .get_result(connection)
        .expect(&format!("Unable to find user {}", user_id))
}

pub fn remove(connection: &PgConnection, user_id: i32) {
    diesel::delete(users_table.find(user_id))
        .execute(connection)
        .expect(&format!("Unable to find user {}", user_id));
}
