use crate::{ models::user::User, repository::mongo::UserRepository };
use mongodb::results::InsertOneResult;
use rocket::{ http::Status, serde::json::{self, Json}, State };

#[post("/users", data = "<new_user>")]
pub fn create_user(
    db: &State<UserRepository>,
    new_user: Json<User>
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        age: new_user.age,
    };
    let result = db.create_user(data);
    match result {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<path>")]
pub fn get_user(
    db: &State<UserRepository>,
    path: String
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let user = db.get_user(id);
    match user {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError)
    }
}
