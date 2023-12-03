use crate::{
    db,
    models::{auth_model::LoginRequest, user_model::SelectUser},
};

pub async fn login(login_request: &LoginRequest) -> Result<SelectUser, sqlx::Error> {
    let user = sqlx::query_as::<_, SelectUser>(
        "
        SELECT * 
        FROM users
        WHERE 
            users.email = $1
            AND users.password = $2
        ;
    ",
    )
    .bind(login_request.email.to_string())
    .bind(login_request.password.to_string())
    .fetch_one(db::POOL.get().unwrap())
    .await?;

    Ok(user)
}
