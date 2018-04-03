use super::schema::users;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub token: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
}
