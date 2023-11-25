#[derive(Debug, sqlx::FromRow)]
pub struct SelectCountry {
    pub id: i64,
    pub name: String,
}

#[allow(dead_code)]
pub struct InsertCountry {
    pub name: String,
}

#[allow(dead_code)]
pub struct SelectCity {
    pub id: i64,
    pub name: String,
}

#[allow(dead_code)]
pub struct InsertCity {
    pub name: String,
}
