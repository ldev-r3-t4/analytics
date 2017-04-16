#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate analytics_proto as proto;

pub mod v1;

use rocket::Rocket;

fn countdown() -> Rocket {
    rocket::ignite()
        .mount("/v1", v1::routes())
}

fn main() {
    countdown().launch();
}
