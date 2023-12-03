use crate::{
    db,
    models::user_model::{InsertUser, UserId},
};

pub async fn insert_user(user: &InsertUser) -> Result<UserId, sqlx::Error> {
    let pool = db::POOL.get().unwrap();
    let mut tx = pool.begin().await?;

    sqlx::query(
        "
        INSERT INTO users (email, password, first_name, last_name)
        VALUES ($1, $2, $3, $4);
    ",
    )
    .bind(user.email.to_string())
    .bind(user.password.to_string())
    .bind(user.first_name.to_string())
    .bind(user.last_name.to_string())
    .execute(&mut tx)
    .await?;

    let user_id = sqlx::query_as::<_, UserId>(
        "
        SELECT last_insert_rowid() as id;
    ",
    )
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;

    println!("User with ID of: {}, has been inserted", user_id.id);

    Ok(user_id)
}
