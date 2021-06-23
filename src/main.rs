#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Response{
    status: usize,
    message: String,
}

#[get("/")]
fn index() -> Json<Response> {
    Json(Response { status: 200, message: "Welcome".to_string() })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}