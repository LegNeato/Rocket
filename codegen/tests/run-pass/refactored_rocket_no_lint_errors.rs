#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(dead_code, unused_variables)]
#![deny(unmounted_routes, unmanaged_state)]

extern crate rocket;

use rocket::{Rocket, State};

#[get("/")]
fn index(state: State<u32>) {  }

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .manage(100u32)
}

fn main() {
    if false {
        rocket().launch();
    }

    let instance = rocket();
    if false {
        instance.launch();
    }
}
