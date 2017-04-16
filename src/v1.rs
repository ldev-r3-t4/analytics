//! Mount: `/v1/`
//!
//! This module contains the handlers for the mount point `/v1/`.
use rocket::{Route, State};
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket_contrib::JSON;
use proto::*;

pub fn routes() -> Vec<Route> {
    routes![
        record_event
    ]
}

/// Route: `/`
///
/// This put request records an event for analytics processing.
///
/// It requires a body with format `application/json` using the
/// `Event` datastructure from analytics-proto.
#[put("/", format = "application/json", data = "<event>")]
pub fn record_event(event: JSON<Event>) -> Custom<()> {
    Custom(Status::Ok, ())
}
