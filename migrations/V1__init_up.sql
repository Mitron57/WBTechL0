CREATE TABLE Orders
(
    order_uid          TEXT PRIMARY KEY,
    track_number       TEXT,
    entry              TEXT,
    locale             TEXT,
    internal_signature TEXT,
    customer_id        TEXT,
    delivery_service   TEXT,
    shardkey           TEXT,
    sm_id              INTEGER,
    date_created       TEXT,
    oof_shard          TEXT
);

CREATE TABLE Deliveries
(
    id      SERIAL PRIMARY KEY,
    name    TEXT,
    phone   TEXT,
    zip     TEXT,
    address TEXT,
    region  TEXT,
    email   TEXT
);

CREATE TABLE Payments
(
    transaction   TEXT PRIMARY KEY,
    request_id    TEXT,
    currency      TEXT,
    provider      TEXT,
    amount        INTEGER,
    payment_dt    INTEGER,
    bank          TEXT,
    delivery_cost INTEGER,
    goods_total   INTEGER,
    custom_fee    INTEGER
);

CREATE TABLE Items
(
    chrt_id      INTEGER PRIMARY KEY,
    track_number TEXT,
    price        INTEGER,
    rid          TEXT,
    name         TEXT,
    sale         INTEGER,
    size         TEXT,
    total_price  INTEGER,
    nm_id        INTEGER,
    brand        TEXT,
    status       INTEGER
);

CREATE TABLE OrderItems
(
    order_uid TEXT,
    chrt_id   INTEGER,
    PRIMARY KEY (order_uid, chrt_id),
    FOREIGN KEY (order_uid) REFERENCES Orders (order_uid) ON DELETE CASCADE,
    FOREIGN KEY (chrt_id) REFERENCES Items (chrt_id) ON DELETE CASCADE
);

CREATE TABLE OrderDeliveries
(
    order_uid   TEXT,
    delivery_id INTEGER,
    PRIMARY KEY (order_uid, delivery_id),
    FOREIGN KEY (order_uid) REFERENCES Orders (order_uid) ON DELETE CASCADE,
    FOREIGN KEY (delivery_id) REFERENCES Deliveries (id) ON DELETE CASCADE
);

CREATE TABLE OrderPayments
(
    order_uid  TEXT,
    payment_id TEXT,
    PRIMARY KEY (order_uid, payment_id),
    FOREIGN KEY (order_uid) REFERENCES Orders (order_uid) ON DELETE CASCADE,
    FOREIGN KEY (payment_id) REFERENCES Payments (transaction) ON DELETE CASCADE
);
