use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{ doc, extjson::de::Error, oid::ObjectId },
    results::{ DeleteResult, InsertOneResult, UpdateResult },
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

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let user = self.collection.find_one(filter, None).ok().expect("User not found");
        Ok(user.unwrap())
    }

    pub fn update_user(&self, id: &String, user: User) -> Result<UpdateResult, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let update =
            doc! {
            "$set": {
                "id": user.id, 
                "name": user.name,
                "email": user.email,
                "age": user.age,
            }
        };
        let result = self.collection
            .update_one(filter, update, None)
            .ok()
            .expect("Failed to update user");
        Ok(result)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let result = self.collection
            .delete_one(filter, None)
            .ok()
            .expect("Failed to delete a user");
        Ok(result)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let users = self.collection.find(None, None).ok().expect("Failed to get all users");
        let result = users.map(|user| user.unwrap()).collect();
        Ok(result)
    }
}
