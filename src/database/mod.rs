use crate::variables::{MONGO_URI, DB_NAME, USERS};

use mongodb::{Client, Collection, Database, bson::Document};
use once_cell::sync::OnceCell;

static DBCONN: OnceCell<Client> = OnceCell::new();

pub async fn connect() {
    let client = Client::with_uri_str(MONGO_URI.as_str())
        .await
        .expect("Failed to init db connection.");
    DBCONN.set(client).unwrap();
}

pub fn get_connection() -> &'static Client {
    DBCONN.get().unwrap()
}

pub fn get_db() -> Database {
    get_connection().database(DB_NAME.as_str())
}

pub fn get_collection(collection: &str) -> Collection<Document> {
    get_db().collection(collection)
}

pub fn get_users() -> Collection<Document> {
    get_collection(USERS.as_str())
}

pub mod entities;

pub use entities::*;