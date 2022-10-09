use mongodb::{bson::{from_document, Document, to_document, RawDocument}, Collection};
use serde::{Serialize, Deserialize};
use mongodb::bson::{doc, oid::ObjectId};

use crate::database::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub friends: Vec<String>
}

impl User {
    pub fn new(username: String) -> Self {
        Self {
            id: ObjectId::new().to_hex(),
            username: username,
            friends: vec![],
        }
    }

    pub fn from_raw_doc(raw: &RawDocument) -> User {
        User {
            id: raw.get_str("_id").unwrap().to_string(),
            username: raw.get_str("username").unwrap().to_string(),
            friends: raw.get_array("friends").unwrap().into_iter().map(|e|e.unwrap().as_str().unwrap().to_string()).collect()
        }
    }

    pub async fn from_db(id: String) -> Option<User>{
        let user = get_users()
        .find_one(doc! {"_id": id}, None)
        .await;
        if let Ok(res) = user {
            if let Some(user) = res {
                return from_document(user).ok();
            }
        }
        None
    }

    pub async fn create_db(&self) {
        get_users()
        .insert_one(
            to_document(self).unwrap(),
            None,
        )
        .await.unwrap();
    }

    pub async fn update_db(&self) {
        let filter = doc!{"_id": &self.id};
        let update = doc!{"$set": doc!{
            "username": &self.username,
            "friends": &self.friends
        }};
        get_users()
        .update_one(filter, update , None)
        .await.unwrap();
    }

    pub fn add_friend(&mut self, uid: String) {
        self.friends.push(uid);
    }
}