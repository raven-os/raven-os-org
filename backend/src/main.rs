#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate newsletter;

use self::newsletter::*;

#[get("/email/<email>")]
fn add_user(email: String) -> String {
    let connection = establish_connection();

    let user = create_user(&connection, &email);
    format!("Added in database: [id: {}, email: {}]", user.id, user.email)
}

#[get("/users")]
fn get_users() -> String {
    let connection = establish_connection();

    let users = get_all_users(&connection);
    let mut response = "".to_string();
    for user in users {
        response.push_str(&user.email);
        response.push_str("\n")
    }
    response
}

fn main() {
    rocket::ignite().mount("/", routes![add_user, get_users]).launch();
}
