use crate::ctx::Ctx;
use async_trait::async_trait;

#[async_trait]
pub trait BaseStore<T> {
    async fn get(&self, ctx: &Ctx, id: i64) -> Result<T, sqlx::Error>;
    async fn list(&self, ctx: &Ctx) -> Result<Vec<T>, sqlx::Error>;
}
