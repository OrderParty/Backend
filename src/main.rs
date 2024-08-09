use diesel::prelude::*;
use order_party::*;
use order_party::models::Event;
use order_party::schema::events::dsl::events;

fn main() {
    let connection = &mut establish_connection();
    let results = events
        .select(Event::as_select())
        .get_results(connection);

    print!("{results:?}")
}
