use diesel::prelude::*;
#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub access_token: String,
    pub admin_access_token: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Event))]
#[diesel(table_name = crate::schema::waiters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Waiter {
    pub id: i32,
    pub event_id: i32,
    pub name: String,
    pub access_pin: String,
    pub is_active: bool,
    pub can_accept_payment: bool,
    pub scope: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Event))]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub event_id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    // TODO: check whether float is right here
    pub price: f32,
    pub discounted_price: f32,
    pub stock: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Event))]
#[diesel(table_name = crate::schema::tables)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Table {
    pub id: i32,
    pub event_id: i32,
    pub name: String
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Table))]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Order {
    pub id: i32,
    pub table_id: i32,
    pub timestamp: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Event))]
#[diesel(table_name = crate::schema::discounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Discount {
    pub id: i32,
    pub event_id: i32,
    pub name: String,
    pub value: i32,
}
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Table))]
#[diesel(belongs_to(Waiter))]
#[diesel(belongs_to(Discount))]
#[diesel(table_name = crate::schema::payments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Payment {
    pub id: i32,
    pub table_id: i32,
    pub waiter_id: i32,
    pub discount_id: i32,
    pub price: f32,
    // TODO: check whether float is right here
    pub timestamp: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(Payment))]
#[diesel(table_name = crate::schema::order_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub item_id: i32,
    pub payment_id: Option<i32>,
    pub info: String,
    // TODO: check whether float is right here
    pub price: f32,
    pub discount: f32,
    pub completed: bool,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Item))]
#[diesel(table_name = crate::schema::extras)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Extra {
    pub id: i32,
    pub item_id: i32,
    pub name: String,
    // TODO: check whether float is right here
    pub price: f32,
    pub max_amount: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(OrderItem))]
#[diesel(belongs_to(Extra))]
#[diesel(table_name = crate::schema::order_item_extras)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct OrderItemExtra {
    pub id: i32,
    pub order_item_id: i32,
    pub extra_id: i32,
    pub amount: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Event))]
#[diesel(table_name = crate::schema::settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Setting {
    pub id: i32,
    pub event_id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Waiter))]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Notification {
    pub id: i32,
    pub event_id: i32,
    pub waiter_id: Option<i32>,
    pub name: String,
    pub description: String,
    pub timestamp: String,
}