use rocket::response::Debug;

pub(crate) type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;