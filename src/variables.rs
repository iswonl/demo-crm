use std::env;

lazy_static! {
    pub static ref HOST_NAME: String = env::var("HOST_NAME").unwrap_or("0.0.0.0:8000".to_string());

    pub static ref MONGO_URI: String = env::var("MONGO_URI").expect("Missing MONGO_URI environment variable.");
    pub static ref DB_NAME: String = env::var("DB_NAME").expect("Missing DB_NAME environment variable.");
    pub static ref USERS: String = env::var("USERS").expect("Missing USERS environment variable.");
}