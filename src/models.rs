use super::schema::users;

#[derive(Queryable)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str
}
