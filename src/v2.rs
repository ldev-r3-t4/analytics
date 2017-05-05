//! Mount: `/v1/`
//!
//! This module contains the handlers for the mount point `/v1/`.
use rocket::{Route, State};
use rocket_contrib::JSON;
use proto::proto2::*;
use std::sync::Mutex;
use rq::{self, Client};

const URL: &str = "http://ec2-54-69-164-246.us-west-2.compute.amazonaws.com:8000/v1/analytic/";

#[doc(hidden)]
pub type Data = Mutex<Client>;

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub struct Storage {
    body: Analytics,
    version: u64,
}

#[doc(hidden)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Analytics {
    events: Vec<Event>,
}

impl Analytics {
    fn add(&mut self, event: Event) {
        self.events.push(event);
    }
}

#[doc(hidden)]
pub fn routes() -> Vec<Route> {
    routes![record_event, get_events]
}

#[doc(hidden)]
pub fn new_data() -> Data {
    Mutex::new(Client::new().expect("Error: Couldn't make client connection pool."))
}

/// `PUT /`
///
/// This put request records an event for analytics processing.
///
/// It requires a body with format `application/json` using the
/// `Event` datastructure from analytics-proto.
///
/// - `200 OK` when successful.
/// - `500 Internal Server Error` when unable to fetch or add events to storage.
#[put("/", format = "application/json", data = "<event>")]
pub fn record_event(state: State<Data>, event: JSON<Event>) {
    let client = state.lock().unwrap();
    const RETRY_RECORDS: usize = 16;
    (0..RETRY_RECORDS)
        .map(|_| {
            client
                .get(URL)
                .send()
                .ok()
                .map(|mut response| response.json::<Storage>().unwrap())
                .map(|mut storage| {
                         storage.body.add(event.clone());
                         storage
                     })
                .and_then(|storage| {
                    client
                        .post(&(URL.to_owned() + "ver=" + &(storage.version + 1).to_string() + "/"))
                        .json(&storage.body)
                        .send()
                        .ok()
                })
        })
        .find(|response| {
                  response
                      .as_ref()
                      .map(|response| *response.status() == rq::StatusCode::Ok)
                      .unwrap_or(false)
              })
        .unwrap();
}

/// `GET /`
///
/// Provides a JSON list of all events.
///
/// - `200 OK` when successful.
/// - `500 Internal Server Error` when unable to fetch events from storage.
#[get("/")]
pub fn get_events(state: State<Data>) -> JSON<Vec<Event>> {
    JSON(state
             .lock()
             .unwrap()
             .get(URL)
             .send()
             .ok()
             .map(|mut response| response.json::<Storage>().unwrap())
             .map(|storage| storage.body.events)
             .unwrap())
}

