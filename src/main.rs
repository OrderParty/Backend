use diesel::prelude::*;

use rocket::response::{Debug};
use rocket::serde::{json::Json};
use rocket::{get, launch, routes};

use order_party::*;
use order_party::models::Event;
use order_party::schema::events::dsl::events;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
#[get("/")]
fn index() -> Result<Json<Box<[Event]>>> {
    let connection = &mut establish_connection();
    let results = events
        .select(Event::as_select())
        .get_results(connection);
    Ok(Json(results?.into_boxed_slice()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
