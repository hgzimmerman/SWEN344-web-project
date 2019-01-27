-- Your SQL goes here
CREATE TABLE users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
--     name VARCHAR NOT NULL,
    client_id VARCHAR NOT NULL
);

CREATE TABLE events (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    title VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    start_at TIMESTAMP NOT NULL,
    stop_at TIMESTAMP NOT NULL
);


CREATE TABLE funds (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    quantity FLOAT NOT NULL
);

CREATE TABLE stocks (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    symbol VARCHAR NOT NULL,
    stock_name VARCHAR NOT NULL
);

CREATE TABLE stock_prices (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    stock_uuid UUID NOT NULL REFERENCES stocks(uuid) ON DELETE CASCADE,
    price FLOAT NOT NULL,
    record_time TIMESTAMP NOT NULL -- default now???
);


CREATE TABLE stock_transactions (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    price_uuid UUID NOT NULL REFERENCES stock_prices(uuid) ON DELETE CASCADE,
    quantity INTEGER
);
