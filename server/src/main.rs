#[macro_use]
extern crate rocket;

use rmmt::Account;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/account/<uuid>")]
fn account(uuid: Uuid) -> Json<Account> {
    Json(Account {
        uuid,
        name: "Test account".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![account])
}
