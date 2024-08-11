use rocket::response::Debug;

pub mod events;
pub mod index;
mod helper;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;