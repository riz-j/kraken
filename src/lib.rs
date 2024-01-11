pub mod models {
    pub mod auth_model;
    pub mod city_model;
    pub mod country_model;
    pub mod user_model;
}
pub mod stores {
    pub mod auth_store;
    pub mod city_store;
    pub mod country_store;
    pub mod user_store;
}
pub mod routers {
    pub mod auth_router;
    pub mod city_router;
    pub mod country_router;
    pub mod spa_router;
}
pub mod schemas {
    pub mod city_schema;
    pub mod country_schema;
}
pub mod middlewares {
    pub mod auth_middleware;
}
pub mod services {
    pub mod auth_service;
}
pub mod macros {
    pub mod get_user_model;
}
pub mod db;
