#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate analytics_proto as proto;

pub mod v1;

use rocket::Rocket;
use rocket::response::content::HTML;

fn countdown() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/v1", v1::routes())
}

fn main() {
    countdown().launch();
}

#[get("/")]
pub fn index() -> HTML<&'static str> {
    HTML(r#"
    <!DOCTYPE html>
    <html>
    <head>
    <title>Analytics Tracking</title>
    </head>
    <body>

    <p>See the documentation <a href="https://ldev-r3-t4.github.io/analytics/analytics/">here</a>.</p>

    </body>
    </html>
    "#)
}
