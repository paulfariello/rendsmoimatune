#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use diesel::{prelude::*, PgConnection};
use rmmt::{prelude::*, Account, Expenditure};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use uuid;

mod error;
mod uniqid;

use error::Error;
use uniqid::UniqId;

#[database("main")]
struct MainDbConn(PgConnection);

#[get("/api/account/<uniq_id>")]
async fn account(conn: MainDbConn, uniq_id: UniqId) -> Result<Json<Account>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account: Account = conn.run(move |c| accounts.find(uuid).first(c)).await?;
    Ok(Json(account))
}

#[get("/api/account/<uniq_id>/expenditures")]
fn expenditures(uniq_id: UniqId) -> Json<Vec<Expenditure>> {
    Json(vec![Expenditure {
        name: "Beers".to_string(),
        date: Utc::now(),
        amount: 100,
        payer: "John".to_string(),
    }])
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDbConn::fairing())
        .mount("/", routes![account, expenditures])
}
