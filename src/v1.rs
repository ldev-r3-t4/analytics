//! Mount: `/v1/`
//!
//! This module contains the handlers for the mount point `/v1/`.
use rocket::{Route, State};
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket_contrib::JSON;
use proto::*;
use std::sync::Mutex;

pub type Data = Mutex<Analytics>;

#[derive(Default, Debug, Clone)]
pub struct Analytics {
    events: Vec<Event>,
}

impl Analytics {
    fn add(&mut self, event: Event) {
        self.events.push(event);
    }
}

pub fn routes() -> Vec<Route> {
    routes![record_event]
}

/// `PUT /`
///
/// This put request records an event for analytics processing.
///
/// It requires a body with format `application/json` using the
/// `Event` datastructure from analytics-proto.
#[put("/", format = "application/json", data = "<event>")]
pub fn record_event(state: State<Data>, event: JSON<Event>) {
    state.lock().unwrap().add(event.clone());
}

/// `GET /`
///
/// Provides a JSON list of all events.
#[get("/")]
pub fn get_events(state: State<Data>) -> JSON<Vec<Event>> {
    JSON(state.lock().unwrap().events.clone())
}

