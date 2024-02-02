use crate::{
    ctx::Ctx,
    db,
    models::user_model::{UserInsert, UserSelect, UserUpdate},
};

#[derive(Clone)]
pub struct UserStore;

impl UserStore {
    pub fn new() -> Self {
        Self
    }
}

impl UserStore {
    pub async fn list(&self, _ctx: &Ctx) -> Result<Vec<UserSelect>, sqlx::Error> {
        let users = sqlx::query_as::<_, UserSelect>(
            "
            SELECT * FROM users
            ",
        )
        .fetch_all(db::POOL.get().unwrap())
        .await?;

        Ok(users)
    }

    pub async fn select(&self, _ctx: &Ctx, id: u32) -> Result<UserSelect, sqlx::Error> {
        let user = sqlx::query_as::<_, UserSelect>(
            "
            SELECT * 
            FROM users
            WHERE 
                users.id = $1;
            ",
        )
        .bind(id)
        .fetch_one(db::POOL.get().unwrap())
        .await?;

        Ok(user)
    }

    pub async fn insert(&self, item: UserInsert) -> Result<u32, sqlx::Error> {
        let result = sqlx::query(
            "
            INSERT INTO users (email, password, first_name, last_name)
            VALUES ($1, $2, $3, $4);
        ",
        )
        .bind(item.email.to_string())
        .bind(item.password.to_string())
        .bind(item.first_name.to_string())
        .bind(item.last_name.to_string())
        .execute(db::POOL.get().unwrap())
        .await?;

        Ok(result.last_insert_rowid() as u32)
    }

    pub async fn update(&self, _ctx: &Ctx, id: u32, item: UserUpdate) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            UPDATE users
            SET 
                email = COALESCE($1, email),
                password = COALESCE($2, password),
                first_name = COALESCE($3, first_name),
                last_name = COALESCE($4, last_name)
            WHERE id = $5;
            ",
        )
        .bind(item.email)
        .bind(item.password)
        .bind(item.first_name)
        .bind(item.last_name)
        .bind(id)
        .execute(db::POOL.get().unwrap())
        .await?;

        Ok(())
    }
}
