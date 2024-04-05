mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::users_api::{ create_user, delete_user, get_all_users, get_user, patch_user };
use repository::mongo::UserRepository;

#[launch]
fn rocket() -> _ {
    let db = UserRepository::init();
    rocket
        ::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![patch_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}
