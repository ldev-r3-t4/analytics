//! This crate runs the analytics server for the Large Scale Round 3 Team 4 project.
//!
//! Git repositories:
//!
//! - [Analytics Server](https://github.com/ldev-r3-t4/analytics)
//! - [Analytics Protocol](https://github.com/ldev-r3-t4/analytics-proto)
//!
//! To see the protocol documentation, click [here](https://ldev-r3-t4.github.io/analytics/analytics_proto/).
//!
//! Choose one of the version modules which corresponds to a mount point below to see more about it.

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unmounted_route)]
extern crate rocket;
extern crate rocket_contrib;
extern crate analytics_proto as proto;
extern crate reqwest as rq;

pub mod v1;

use rocket::Rocket;
use rocket::response::content::HTML;

fn countdown() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/v1", v1::routes())
        .manage(v1::Data::default())
}

fn main() {
    countdown().launch();
}

/// `GET /`
///
/// This is the index page of the server which directs users to the documentation.
#[get("/")]
pub fn index() -> HTML<&'static str> {
    HTML(include_str!("index.html"))
}
