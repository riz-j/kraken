pub mod models {
    pub mod city_model;
    pub mod country_model;
}
pub mod clients {
    pub mod city_client;
    pub mod country_client;
}
pub mod routers {
    pub mod city_router;
    pub mod country_router;
}
pub mod schemas {
    pub mod city_schema;
    pub mod country_schema;
}
pub mod middlewares {
    pub mod require_auth;
}
pub mod db;
