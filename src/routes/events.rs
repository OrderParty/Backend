use diesel::*;
use rocket::{State};
use rocket_okapi::openapi;
use rocket::*;
use rocket::serde::json::Json;
use diesel::sqlite::*;
use rocket::response::Debug;

use order_party::*;
use crate::errors::response::*;
use order_party::models::Event;

use order_party::schema::events::dsl::events;
use crate::db::db::DbConnection;
use crate::routes::helper::*;

#[openapi(tag = "Events")]
#[get("/events")]
pub async fn get_events(
    connection: DbConnection
) -> Result<Json<Box<[Event]>>> {
    let results = connection.run(|c| events
        .select(Event::as_select())
        .get_results(c));
    Ok(Json(results.await.unwrap().into_boxed_slice()))
}
