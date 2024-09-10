// @generated automatically by Diesel CLI.

diesel::table! {
    discounts (id) {
        id -> Integer,
        event_id -> Integer,
        name -> Text,
        value -> Integer,
    }
}

diesel::table! {
    events (id) {
        id -> Integer,
        name -> Text,
        access_token -> Text,
        admin_access_token -> Text,
    }
}

diesel::table! {
    extras (id) {
        id -> Integer,
        item_id -> Integer,
        name -> Text,
        price -> Float,
        max_amount -> Integer,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        event_id -> Integer,
        name -> Text,
        description -> Text,
        image -> Text,
        price -> Float,
        discounted_price -> Float,
        stock -> Integer,
    }
}

diesel::table! {
    notifications (id) {
        id -> Integer,
        event_id -> Integer,
        waiter_id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    order_item_extras (id) {
        id -> Integer,
        order_item_id -> Integer,
        extra_id -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    order_items (id) {
        id -> Integer,
        order_id -> Integer,
        item_id -> Integer,
        payment_id -> Nullable<Integer>,
        info -> Text,
        price -> Float,
        discount -> Float,
        completed -> Bool,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        table_id -> Integer,
        waiter_id -> Integer,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    payments (id) {
        id -> Integer,
        table_id -> Integer,
        waiter_id -> Integer,
        discount_id -> Integer,
        price -> Float,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    settings (id) {
        id -> Integer,
        event_id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    tables (id) {
        id -> Integer,
        event_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    waiters (id) {
        id -> Integer,
        event_id -> Integer,
        name -> Text,
        access_pin -> Text,
        is_active -> Bool,
        can_accept_payment -> Bool,
        scope -> Text,
    }
}

diesel::joinable!(discounts -> events (event_id));
diesel::joinable!(extras -> items (item_id));
diesel::joinable!(items -> events (event_id));
diesel::joinable!(notifications -> events (event_id));
diesel::joinable!(notifications -> waiters (waiter_id));
diesel::joinable!(order_item_extras -> extras (extra_id));
diesel::joinable!(order_item_extras -> order_items (order_item_id));
diesel::joinable!(order_items -> items (item_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> payments (payment_id));
diesel::joinable!(orders -> tables (table_id));
diesel::joinable!(payments -> discounts (discount_id));
diesel::joinable!(payments -> tables (table_id));
diesel::joinable!(payments -> waiters (waiter_id));
diesel::joinable!(settings -> events (event_id));
diesel::joinable!(tables -> events (event_id));
diesel::joinable!(waiters -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    discounts,
    events,
    extras,
    items,
    notifications,
    order_item_extras,
    order_items,
    orders,
    payments,
    settings,
    tables,
    waiters,
);
