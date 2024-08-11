#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::prelude::*;

use rocket::launch;

use crate::db::db::DbConnection;
use dotenvy::dotenv;
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

mod db;
mod errors;
mod routes;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(DbConnection::fairing())
        .mount(
            "/",
            openapi_get_routes![routes::index::index, routes::events::get_events,],
        )
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
