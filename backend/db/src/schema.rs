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
    stock_acquisitions (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        price_uuid -> Uuid,
        quantity -> Nullable<Int4>,
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
    stock_sells (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        price_uuid -> Uuid,
        quantity -> Nullable<Int4>,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        oauth -> Varchar,
    }
}

joinable!(events -> users (user_uuid));
joinable!(funds -> users (user_uuid));
joinable!(stock_acquisitions -> stock_prices (price_uuid));
joinable!(stock_acquisitions -> users (user_uuid));
joinable!(stock_prices -> stocks (stock_uuid));
joinable!(stock_sells -> stock_prices (price_uuid));
joinable!(stock_sells -> users (user_uuid));

allow_tables_to_appear_in_same_query!(
    events,
    funds,
    stock_acquisitions,
    stock_prices,
    stocks,
    stock_sells,
    users,
);
