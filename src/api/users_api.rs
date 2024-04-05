use crate::{ models::user::User, repository::mongo::UserRepository };
use mongodb::{ bson::oid::ObjectId, results::InsertOneResult };
use rocket::{ http::Status, serde::json::Json, State };

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
pub fn get_user(db: &State<UserRepository>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let user = db.get_user(&id);
    match user {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/users/<path>", data = "<user>")]
pub fn patch_user(
    db: &State<UserRepository>,
    path: String,
    user: Json<User>
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        age: user.age,
    };
    let result = db.update_user(&id, data);
    match result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/users/<path>")]
pub fn delete_user(db: &State<UserRepository>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }

    let result = db.delete_user(&id);

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User deleted successfully"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub fn get_all_users(db: &State<UserRepository>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}
