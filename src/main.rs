
#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use mongodb::bson::Bson;
use mongodb::bson::from_document;
use mongodb::bson::doc;
use mongodb::bson::to_document;
use serde::Deserialize;
use serde::Serialize;

pub mod variables;
pub mod database;

use variables::HOST_NAME;

use database::*;

#[derive(Serialize, Deserialize)]
struct Users{
    users: Vec<User>,
}

#[derive(Serialize, Deserialize)]
struct NewUser{
    username: String,
}

#[derive(Serialize, Deserialize)]
struct Friends{
    friends: Vec<String>,
}

impl Users{
    fn new() -> Users{
        Users {
            users: vec![],
        }
    }

    fn add_user(&mut self, user: User){
         self.users.push(user);
    }
}



#[get("/ping")]
async fn ping() -> impl Responder {
    println!("hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn get_users() -> impl Responder {
    println!("get_users");
    let mut users = Users::new();
    let db = database::get_users();
    let mut cursor = db.find(None, None).await.expect("database find users error!");
    while cursor.advance().await.unwrap() {
        users.add_user(User::from_raw_doc(cursor.current()));
    }
    HttpResponse::Ok().json(users)
}

#[post("/user")]
async fn add_user(user: web::Json<NewUser>) -> impl Responder{
    println!("add_user");
    let user = User::new(user.0.username);
    user.create_db().await;
    HttpResponse::Ok().json(user)
}

#[get("/user/{id}")]
async fn get_user(id: web::Path<String>) -> impl Responder {
    println!("get_user: {}", id);
    HttpResponse::Ok().json(User::from_db(id.to_string()).await.unwrap())
}

#[post("/user/{id}/friend")]
async fn add_friends(id: web::Path<String>, friend: web::Json<Friends>) -> impl Responder{
    println!("add_friends: {}", id);
    let mut user = User::from_db(id.to_string()).await.unwrap();
    friend.friends.iter().for_each(|id| {
        user.add_friend(id.clone());
    });
    user.update_db().await;
    HttpResponse::Ok().json(user)
}

#[get("/friends/{id}")]
async fn friends(id: web::Path<String>) -> impl Responder {
    println!("friends: {}", id);
    HttpResponse::Ok().json(Friends{
        friends: User::from_db(id.clone()).await.unwrap().friends,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    database::connect().await;
    HttpServer::new(move|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(ping)
            .service(get_users)
            .service(add_user)
            .service(get_user)
            .service(add_friends)
            .service(friends)
    })
    .bind(HOST_NAME.as_str())?
    .run()
    .await
}