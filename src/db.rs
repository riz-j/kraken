use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::sync::OnceCell;

pub static POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn init_pool() -> &'static Pool<Sqlite> {
    POOL.get_or_init(|| async {
        SqlitePoolOptions::new()
            .connect("sqlite:./db_main.db")
            .await
            .expect("Failed to create SqlLite Pool")
    })
    .await
}
