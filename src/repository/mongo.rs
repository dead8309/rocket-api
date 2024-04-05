use std::{ env, fmt::format };
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{ doc, extjson::de::Error, oid::ObjectId },
    results::InsertOneResult,
    sync::{ Client, Collection },
};
use crate::models::user::User;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v,
            Err(_) => format!("Could not find MONGOURI in .env file"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust-mongo");
        let collection: Collection<User> = db.collection("users");
        UserRepository { collection }
    }

    pub fn create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        let new_user = User {
            id: None,
            name: user.name,
            email: user.email,
            age: user.age,
        };

        let result = self.collection.insert_one(new_user, None).expect("Failed to create a user");

        Ok(result)
    }

    pub fn get_user(&self, id: String) -> Result<User, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let user = self.collection.find_one(filter, None).ok().expect("User not found");
        Ok(user.unwrap())
    }
}
