#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate hello_rocket;

use std::collections::HashMap;

use rocket::response::{Redirect, Flash};
use rocket::outcome::IntoOutcome;
use rocket::http::RawStr;
use serde::Serialize;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;
use self::hello_rocket::models::Post;
use diesel::prelude::*;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::http::{ Cookie, Cookies};

#[database("my_db")]
struct MyDbConn(diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[derive(Serialize)]
struct Task {
    id: u16,
    title: String,
    content: String,
}

#[get("/task")]
fn get_task() -> Json<Task> {
    Json(Task { id: 1, title: format!("テスト"), content: format!("内容です") } )
}

#[get("/posts/<_id>")]
fn get_post(conn: MyDbConn, _id: i32) -> String {
    use hello_rocket::schema::posts::dsl::*;

    let result = posts.find(_id)
        .first::<Post>(&*conn);
    match result {
        Ok(v) => format!("Post {}", v.title),
        Err(e) => format!("{}", e)
    }

    // let results = posts.filter(id.eq(_id))
    //     .limit(5)
    //     .load::<Post>(&*conn)
    //     .expect("Error loading posts");
    // format!("Post, {}", results[0].title)
}

#[derive(Debug)]
struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

#[get("/login_data")]
fn login(mut cookies: Cookies,) -> Result<Redirect, Flash<Redirect>> {
    cookies.add_private(Cookie::new("user_id", 1.to_string()));
    Ok(Redirect::to(uri!(index)))
}

#[get("/user")]
fn user_logined(user: User) -> String {
    format!("Hello, {}!", user.0)
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(index)), "Successfully logged out.")
}

#[get("/user", rank = 2)]
fn user_not_login() -> String {
    format!("User not login!")
}


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite().register(catchers![not_found])
                    .mount("/", routes![index, hello, get_task, get_post, login, logout, user_logined, user_not_login])
                    .attach(MyDbConn::fairing())
                    .launch();
}
