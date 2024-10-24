#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use std::sync::Mutex;
use rocket::State;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: usize,
    name: String,
}

#[derive(Default)]
struct UserStore {
    users: Mutex<Vec<User>>,
}

#[get("/users")]
fn get_users(store: &State<UserStore>) -> Json<Vec<User>> {
    let users = store.users.lock().unwrap();
    Json(users.clone())
}

#[post("/users", format = "json", data = "<user>")]
fn create_user(user: Json<User>, store: &State<UserStore>) -> Json<&'static str> {
    let mut users = store.users.lock().unwrap();
    let new_user = user.into_inner();
    println!("Adding user: {:?}", new_user); // Stampa log nella console
    users.push(new_user);
    Json("User successfully added") // Risposta di conferma
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(UserStore::default())
        .mount("/", routes![get_users, create_user])
}
