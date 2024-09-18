use crate::domain::interfaces;
use crate::domain::models::{Delivery, Item, Order, Payment};
use log::{log, Level};
use tokio_postgres::error::SqlState;
use tokio_postgres::{Client, Error, NoTls, Row, Transaction};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new(uri: String) -> Result<Database, Error> {
        let (client, connection) = tokio_postgres::connect(uri.as_str(), NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log!(target: "psql", Level::Error, "Connection error: {}", e);
            }
            log!(target: "psql", Level::Info, "Connection established: {}", uri);
        });
        Ok(Database { client })
    }

    async fn insert_order<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn std::error::Error>> {
        let result = transaction
            .execute(
                "INSERT INTO Orders VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                &[
                    &data.order_uid,
                    &data.track_number,
                    &data.entry,
                    &data.locale,
                    &data.internal_signature,
                    &data.customer_id,
                    &data.delivery_service,
                    &data.shardkey,
                    &data.sm_id,
                    &data.date_created,
                    &data.oof_shard,
                ],
            )
            .await;
        if let Err(e) = result {
            log!(target: "psql", Level::Error, "Error adding order: {data:?} to database: {e}");
            if let Err(roll_err) = transaction.rollback().await {
                log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                return Err("Internal server error".into());
            }
            return Err(e.into());
        }
        Ok(transaction)
    }

    async fn insert_delivery<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn std::error::Error>> {
        let delivery = &data.delivery;
        let result = transaction
            .query(
                "INSERT INTO Deliveries(name, phone, zip, address, region, email) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
                &[
                    &delivery.name,
                    &delivery.phone,
                    &delivery.zip,
                    &delivery.address,
                    &delivery.region,
                    &delivery.email,
                ],
            )
            .await;
        let delivery_id: i32;
        match result {
            Ok(rows) => {
                delivery_id = rows[0].get(0);
            }
            Err(err) => {
                log!(target: "psql", Level::Error, "Error adding delivery: {delivery:?}, err: {err}");
                if let Err(roll_err) = transaction.rollback().await {
                    log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                    return Err("Internal server error".into());
                }
                return Err(err.into());
            }
        }
        let result = transaction
            .execute(
                "INSERT INTO OrderDeliveries VALUES($1, $2)",
                &[&data.order_uid, &delivery_id],
            )
            .await;
        if let Err(err) = result {
            log!(target: "psql", Level::Error, "Error adding OrderDeliveries: order_uid: {}, delivery_id : {delivery_id} {err}", data.order_uid);
            if let Err(roll_err) = transaction.rollback().await {
                log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                return Err("Internal server error".into());
            }
            return Err(err.into());
        }
        Ok(transaction)
    }

    async fn insert_payment<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn std::error::Error>> {
        let payment = &data.payment;
        let result = transaction
            .execute(
                "INSERT INTO Payments VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[
                    &payment.transaction,
                    &payment.request_id,
                    &payment.currency,
                    &payment.provider,
                    &payment.amount,
                    &payment.payment_dt,
                    &payment.bank,
                    &payment.delivery_cost,
                    &payment.goods_total,
                    &payment.custom_fee,
                ],
            )
            .await;
        if let Err(e) = result {
            log!(target: "psql", Level::Error, "Error adding payment: {payment:?} to database: {e}");
            if let Err(roll_err) = transaction.rollback().await {
                log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                return Err("Internal server error".into());
            }
            return Err(e.into());
        }
        let result = transaction
            .execute(
                "INSERT INTO OrderPayments VALUES ($1, $2)",
                &[&data.order_uid, &payment.transaction],
            )
            .await;
        if let Err(e) = result {
            log!(target: "psql", Level::Error, "Error adding OrderPayments: order_uid: {}, payment_transaction: {}, err: {e}", data.order_uid, payment.transaction);
            if let Err(roll_err) = transaction.rollback().await {
                log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                return Err("Internal server error".into());
            }
            return Err(e.into());
        }
        Ok(transaction)
    }

    async fn insert_items<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn std::error::Error>> {
        let items_insert = transaction
            .prepare("INSERT INTO Items VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) ON CONFLICT DO NOTHING;")
            .await?;
        let order_items = transaction
            .prepare("INSERT INTO OrderItems VALUES ($1, $2);")
            .await?;
        for item in &data.items {
            let result = transaction
                .execute(
                    &items_insert,
                    &[
                        &item.chrt_id,
                        &item.track_number,
                        &item.price,
                        &item.rid,
                        &item.name,
                        &item.sale,
                        &item.size,
                        &item.total_price,
                        &item.nm_id,
                        &item.brand,
                        &item.status,
                    ],
                )
                .await;
            if let Err(e) = result {
                log!(target: "psql", Level::Warn, "Error adding item: {item:?} to database: {e}");
                if let Err(roll_err) = transaction.rollback().await {
                    log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                    return Err("Internal server error".into());
                }
                return Err(e.into());
            }
            let result = transaction
                .execute(&order_items, &[&data.order_uid, &item.chrt_id])
                .await;
            if let Err(e) = result {
                log!(target: "psql", Level::Error, "Error adding OrderItems: order_uid: {}, item_chrt_id: {}, err: {e}, code: {:?}", data.order_uid, item.chrt_id, e.code());
                if let Err(roll_err) = transaction.rollback().await {
                    log!(target: "psql", Level::Error, "Rollback error: {roll_err}");
                    return Err("Internal server error".into());
                }
                return Err(e.into());
            }
        }
        Ok(transaction)
    }

    async fn get_order(&self, order_id: &str) -> Option<Order> {
        let result = self
            .client
            .query("SELECT * FROM Orders WHERE order_uid = $1", &[&order_id])
            .await;
        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    log!(target: "psql", Level::Warn, "Order not found, order_id: {}", order_id);
                    return None;
                }
                Some(Order {
                    order_uid: rows[0].get("order_uid"),
                    track_number: rows[0].get("track_number"),
                    entry: rows[0].get("entry"),
                    locale: rows[0].get("locale"),
                    internal_signature: rows[0].get("internal_signature"),
                    customer_id: rows[0].get("customer_id"),
                    delivery_service: rows[0].get("delivery_service"),
                    shardkey: rows[0].get("shardkey"),
                    sm_id: rows[0].get("sm_id"),
                    date_created: rows[0].get("date_created"),
                    oof_shard: rows[0].get("oof_shard"),
                    ..Default::default()
                })
            }
            Err(err) => {
                log!(target: "psql", Level::Error, "Get order failed, id: {}, Err: {}", order_id, err);
                None
            }
        }
    }

    async fn get_delivery(&self, order_id: &str) -> Option<Delivery> {
        let result = self
            .client
            .query(
                "SELECT * FROM Deliveries d
                JOIN OrderDeliveries od ON od.delivery_id = d.id
                WHERE od.order_uid = $1;",
                &[&order_id],
            )
            .await;
        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    log!(target: "psql", Level::Warn, "Delivery not found, order_id: {}", order_id);
                    return None;
                }
                Some(Delivery {
                    name: rows[0].get("name"),
                    phone: rows[0].get("phone"),
                    zip: rows[0].get("zip"),
                    address: rows[0].get("address"),
                    region: rows[0].get("region"),
                    email: rows[0].get("email"),
                })
            }
            Err(err) => {
                log!(target: "psql", Level::Error, "Get delivery failed, order_id: {}, Err: {}", order_id, err);
                None
            }
        }
    }

    async fn get_payment(&self, order_uid: &str) -> Option<Payment> {
        let result = self
            .client
            .query(
                "SELECT * FROM Payments p
                 JOIN OrderPayments op ON p.transaction = op.payment_id
                 WHERE op.order_uid = $1;",
                &[&order_uid],
            )
            .await;
        let rows: Vec<Row>;
        match result {
            Ok(lines) => {
                if lines.is_empty() {
                    log!(target: "psql", Level::Warn, "Payment not found, order_id: {}", order_uid);
                    return None;
                }
                rows = lines;
            }
            Err(err) => {
                log!(target: "psql", Level::Error, "Get payment failed, order_id: {}, Err: {}", order_uid, err);
                return None;
            }
        }
        Some(Payment {
            transaction: rows[0].get("transaction"),
            request_id: rows[0].get("request_id"),
            currency: rows[0].get("currency"),
            provider: rows[0].get("provider"),
            amount: rows[0].get("amount"),
            payment_dt: rows[0].get("payment_dt"),
            bank: rows[0].get("bank"),
            delivery_cost: rows[0].get("delivery_cost"),
            goods_total: rows[0].get("goods_total"),
            custom_fee: rows[0].get("custom_fee"),
        })
    }

    async fn get_items(&self, order_id: &str) -> Option<Vec<Item>> {
        let result = self
            .client
            .query(
                "SELECT i.* FROM Items i 
                 JOIN OrderItems oi ON i.chrt_id = oi.chrt_id 
                 WHERE oi.order_uid = $1",
                &[&order_id],
            )
            .await;

        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    log!(target: "psql", Level::Warn, "Items not found, order_id: {}", order_id);
                    return None;
                }
                let items: Vec<Item> = rows
                    .iter()
                    .map(|row| Item {
                        chrt_id: row.get("chrt_id"),
                        track_number: row.get("track_number"),
                        price: row.get("price"),
                        rid: row.get("rid"),
                        name: row.get("name"),
                        sale: row.get("sale"),
                        size: row.get("size"),
                        total_price: row.get("total_price"),
                        nm_id: row.get("nm_id"),
                        brand: row.get("brand"),
                        status: row.get("status"),
                    })
                    .collect();
                Some(items)
            }
            Err(err) => {
                log!(target: "psql", Level::Error, "Get items failed, order_id: {}, Err: {}", order_id, err);
                None
            }
        }
    }
}

impl interfaces::Database for Database {
    async fn insert(&mut self, data: Order) -> Result<(), Box<dyn std::error::Error>> {
        let transaction = self.client.transaction().await?;
        let transaction = Self::insert_order(transaction, &data).await?;
        let transaction = Self::insert_delivery(transaction, &data).await?;
        let transaction = Self::insert_payment(transaction, &data).await?;
        let transaction = Self::insert_items(transaction, &data).await?;
        log!(target: "psql", Level::Info, "Order inserted successfully");
        Ok(transaction.commit().await?)
    }

    async fn remove(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let transaction = self.client.transaction().await?;
        let result = transaction
            .execute("DELETE FROM Orders CASCADE WHERE order_id = $1", &[&id])
            .await;
        match result {
            Ok(_) => {
                log!(target: "psql", Level::Info, "Removed order with order_id: {}", id);
                Ok(transaction.commit().await?)
            }
            Err(err) => {
                log!(target: "psql", Level::Error, "Failed removing order with order_id: {}, err: {}", id, err);
                transaction.rollback().await?;
                Err(err.into())
            }
        }
    }

    async fn get(&self, id: &str) -> Option<Order> {
        let (order, payment, delivery, items) = tokio::join!(
            self.get_order(id),
            self.get_payment(id),
            self.get_delivery(id),
            self.get_items(id)
        );
        let mut order = order?;
        order.payment = payment?;
        order.delivery = delivery?;
        order.items = items?;
        Some(order)
    }
}
