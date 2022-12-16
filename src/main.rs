#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use rocket::serde::Serialize;

mod cors;

use cors::CORS;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct EchoResponse {
    unique_id: String,
    server_time: DateTime<Utc>,
}

#[get("/")]
fn index() -> Json<EchoResponse> {
    let unique_id = uuid::Uuid::new_v4()
        .hyphenated()
        .encode_lower(&mut uuid::Uuid::encode_buffer())
        .to_owned();
    Json(EchoResponse {
        unique_id,
        server_time: Utc::now(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![index])
}
