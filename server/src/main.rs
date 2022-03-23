#[macro_use]
extern crate rocket;

use rmmt::{Account, Expenditure};
use rocket::serde::json::Json;
use uuid::Uuid;
use chrono::prelude::*;

#[get("/api/account/<uuid>")]
fn account(uuid: Uuid) -> Json<Account> {
    Json(Account {
        uuid,
        name: "Test account".to_string(),
    })
}

#[get("/api/account/<uuid>/expenditures")]
fn expenditures(uuid: Uuid) -> Json<Vec<Expenditure>> {
    Json(vec![Expenditure {
        name: "Beers".to_string(),
        date: Utc::now(),
        amount: 100,
        payer: "John".to_string(),
    }])
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![account, expenditures])
}
