use crate::db::db::DbConnection;
use crate::routes::helper::*;
use order_party::models::Event;
use order_party::schema::events::dsl::events;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Events")]
#[get("/")]
pub async fn index(// connection: DbConnection
) -> Result<String> {
    Ok(String::from("Hello world"))
}
