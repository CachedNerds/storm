use schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: Option<String>,
}
