use crate::ctx::Ctx;
use async_trait::async_trait;

#[async_trait]
pub trait BaseStore<S, I, U> {
    async fn select(&self, ctx: &Ctx, id: u32) -> Result<S, sqlx::Error>;
    async fn list(&self, ctx: &Ctx) -> Result<Vec<S>, sqlx::Error>;
    async fn insert(&self, ctx: &Ctx, item: I) -> Result<(), sqlx::Error>;
    async fn update(&self, ctx: &Ctx, id: u32, item: U) -> Result<(), sqlx::Error>;
}
