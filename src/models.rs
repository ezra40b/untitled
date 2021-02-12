use crate::schema::users;

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub random: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String
}

pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}