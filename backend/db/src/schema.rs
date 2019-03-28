table! {
    adaptive_health (uuid) {
        uuid -> Uuid,
        available_servers -> Int4,
        load -> Int4,
        did_serve -> Bool,
        time_recorded -> Timestamp,
    }
}

table! {
    events (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        title -> Varchar,
        text -> Varchar,
        start_at -> Timestamp,
        stop_at -> Timestamp,
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
        stock_uuid -> Uuid,
        quantity -> Int4,
        price_of_stock_at_time_of_trading -> Float8,
        record_time -> Timestamp,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        twitter_user_id -> Varchar,
        zip_code -> Nullable<Varchar>,
    }
}

joinable!(events -> users (user_uuid));
joinable!(stock_transactions -> stocks (stock_uuid));
joinable!(stock_transactions -> users (user_uuid));

allow_tables_to_appear_in_same_query!(
    adaptive_health,
    events,
    stocks,
    stock_transactions,
    users,
);
