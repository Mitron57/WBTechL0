use crate::domain::interfaces;
use crate::domain::models::{Delivery, Item, Order, Payment};
use crate::infrastructure::MultiError;
use axum::async_trait;
use deadpool_postgres::{Manager, Pool, Runtime, Timeouts, Transaction};
use std::error::Error;
use tokio_postgres::NoTls;

macro_rules! fill_fields {
    (Order, $data:expr, $($field:ident),+) => {
        Order {
            $(
                $field: $data.get(stringify!($field))
            ),+,
            ..Default::default()
        }
    };
    ($struct_name:ident, $data:expr, $($field:ident),+) => {
        $struct_name {
            $(
                $field: $data.get(stringify!($field))
            ),+
        }
    };
 }

pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: String) -> Result<Database, Box<dyn Error>> {
        let config = config.parse::<tokio_postgres::Config>()?;
        let pool = Pool::builder(Manager::new(config, NoTls))
            .runtime(Runtime::Tokio1)
            .timeouts(Timeouts::wait_millis(30000))
            .build()?;
        Ok(Database { pool })
    }

    async fn insert_order<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn Error>> {
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
        if let Err(err) = result {
            if let Err(roll_err) = transaction.rollback().await {
                return Err(MultiError::new(vec![err.into(), roll_err.into()]).into());
            }
            return Err(err.into());
        }
        Ok(transaction)
    }

    async fn insert_delivery<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn Error>> {
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
        let delivery_id = match result {
            Ok(rows) => rows[0].get::<_, i32>(0),
            Err(err) => {
                if let Err(roll_err) = transaction.rollback().await {
                    return Err(MultiError::new(vec![err.into(), roll_err.into()]).into());
                }
                return Err(err.into());
            }
        };
        let result = transaction
            .execute(
                "INSERT INTO OrderDeliveries VALUES($1, $2)",
                &[&data.order_uid, &delivery_id],
            )
            .await;
        if let Err(err) = result {
            if let Err(roll_err) = transaction.rollback().await {
                return Err(MultiError::new(vec![err.into(), roll_err.into()]).into());
            }
            return Err(err.into());
        }
        Ok(transaction)
    }

    async fn insert_payment<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn Error>> {
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
        if let Err(err) = result {
            if let Err(roll_err) = transaction.rollback().await {
                return Err(MultiError::new(vec![err.into(), roll_err.into()]).into());
            }
            return Err(err.into());
        }
        let result = transaction
            .execute(
                "INSERT INTO OrderPayments VALUES ($1, $2)",
                &[&data.order_uid, &payment.transaction],
            )
            .await;
        if let Err(err) = result {
            if let Err(roll_err) = transaction.rollback().await {
                return Err(MultiError::new(vec![err.into(), roll_err.into()]).into());
            }
            return Err(err.into());
        }
        Ok(transaction)
    }

    async fn insert_items<'a>(
        transaction: Transaction<'a>,
        data: &Order,
    ) -> Result<Transaction<'a>, Box<dyn Error>> {
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
            if let Err(err) = result {
                if let Err(roll_err) = transaction.rollback().await {
                    return Err(roll_err.into());
                }
                return Err(err.into());
            }
            let result = transaction
                .execute(&order_items, &[&data.order_uid, &item.chrt_id])
                .await;
            if let Err(err) = result {
                if let Err(roll_err) = transaction.rollback().await {
                    return Err(MultiError::new(vec![err.into(), roll_err.into()]).into());
                }
                return Err(err.into());
            }
        }
        Ok(transaction)
    }
    
    async fn get_order(&self, order_id: &str) -> Result<Option<Order>, Box<dyn Error>> {
        let result = self
            .pool
            .get()
            .await?
            .query("SELECT * FROM Orders WHERE order_uid = $1", &[&order_id])
            .await;
        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    return Ok(None);
                }
                Ok(Some(fill_fields!(
                    Order,
                    rows[0],
                    order_uid,
                    track_number,
                    entry,
                    locale,
                    internal_signature,
                    customer_id,
                    delivery_service,
                    shardkey,
                    sm_id,
                    date_created,
                    oof_shard
                )))
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn get_delivery(&self, order_id: &str) -> Result<Option<Delivery>, Box<dyn Error>> {
        let result = self
            .pool
            .get()
            .await?
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
                    return Ok(None);
                }
                Ok(Some(fill_fields!(
                    Delivery, rows[0], name, phone, zip, address, region, email
                )))
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn get_payment(&self, order_uid: &str) -> Result<Option<Payment>, Box<dyn Error>> {
        let result = self
            .pool
            .get()
            .await?
            .query(
                "SELECT * FROM Payments p
                 JOIN OrderPayments op ON p.transaction = op.payment_id
                 WHERE op.order_uid = $1;",
                &[&order_uid],
            )
            .await;
        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    return Ok(None);
                }
                Ok(Some(fill_fields!(
                    Payment,
                    rows[0],
                    transaction,
                    request_id,
                    currency,
                    provider,
                    amount,
                    payment_dt,
                    bank,
                    delivery_cost,
                    goods_total,
                    custom_fee
                )))
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn get_items(&self, order_id: &str) -> Result<Option<Vec<Item>>, Box<dyn Error>> {
        let result = self
            .pool
            .get()
            .await?
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
                    return Ok(None);
                }
                let items: Vec<Item> = rows
                    .iter()
                    .map(|row| {
                        fill_fields!(
                            Item,
                            row,
                            chrt_id,
                            track_number,
                            price,
                            rid,
                            name,
                            sale,
                            size,
                            total_price,
                            nm_id,
                            brand,
                            status
                        )
                    })
                    .collect();
                Ok(Some(items))
            }
            Err(err) => Err(err.into()),
        }
    }
}

#[async_trait]
impl interfaces::Database for Database {
    async fn insert(&mut self, data: Order) -> Result<(), Box<dyn Error>> {
        let mut instance = self.pool.get().await?;
        let transaction = instance.transaction().await?;
        let transaction = Self::insert_order(transaction, &data).await?;
        let transaction = Self::insert_delivery(transaction, &data).await?;
        let transaction = Self::insert_payment(transaction, &data).await?;
        let transaction = Self::insert_items(transaction, &data).await?;
        Ok(transaction.commit().await?)
    }

    async fn remove(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        let mut instance = self.pool.get().await?;
        let transaction = instance.transaction().await?;
        let result = transaction
            .execute("DELETE FROM Orders CASCADE WHERE order_id = $1", &[&id])
            .await;
        match result {
            Ok(_) => Ok(transaction.commit().await?),
            Err(err) => {
                transaction.rollback().await?;
                Err(err.into())
            }
        }
    }

    async fn get(&self, id: &str) -> Result<Option<Order>, Box<dyn Error>> {
        let order = self.get_order(id).await?;
        if order.is_none() {
            return Ok(None);
        }
        let mut order = order.unwrap();
        let payment = self.get_payment(id).await?;
        if payment.is_none() {
            return Ok(None);
        }
        order.payment = payment.unwrap();
        let delivery = self.get_delivery(id).await?;
        if delivery.is_none() {
            return Ok(None);
        }
        order.delivery = delivery.unwrap();
        let items = self.get_items(id).await?;
        if items.is_none() {
            return Ok(None);
        }
        order.items = items.unwrap();
        Ok(Some(order))
    }
}
