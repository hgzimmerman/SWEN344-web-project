table! {
    events (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        text -> Varchar,
        time_due -> Timestamp,
    }
}

table! {
    funds (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        quantity -> Float8,
    }
}

table! {
    stock_prices (uuid) {
        uuid -> Uuid,
        stock_uuid -> Uuid,
        price -> Float8,
        record_time -> Timestamp,
    }
}

table! {
    stocks (uuid) {
        uuid -> Uuid,
        symbol -> Varchar,
        stock_name -> Varchar,
    }
}

table! {
    stock_transactions (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        price_uuid -> Uuid,
        quantity -> Nullable<Int4>,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        client_id -> Varchar,
    }
}

joinable!(events -> users (user_uuid));
joinable!(funds -> users (user_uuid));
joinable!(stock_prices -> stocks (stock_uuid));
joinable!(stock_transactions -> stock_prices (price_uuid));
joinable!(stock_transactions -> users (user_uuid));

allow_tables_to_appear_in_same_query!(
    events,
    funds,
    stock_prices,
    stocks,
    stock_transactions,
    users,
);
