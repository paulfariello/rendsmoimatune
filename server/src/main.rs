#[macro_use]
extern crate rocket;

use diesel::{prelude::*, PgConnection};
use rmmt::{prelude::*, Account, Expenditure, Repayment};
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
async fn get_account(conn: MainDbConn, uniq_id: UniqId) -> Result<Json<Account>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account: Account = conn.run(move |c| accounts.find(uuid).first(c)).await?;
    Ok(Json(account))
}

#[get("/api/account/<uniq_id>/expenditures")]
async fn get_expenditures(conn: MainDbConn, uniq_id: UniqId) -> Result<Json<Vec<Expenditure>>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account_expenditures: Vec<Expenditure> = conn.run(move |c| expenditures.filter(expenditures_account_id.eq(uuid)).load(c)).await?;

    Ok(Json(account_expenditures))
}

#[get("/api/account/<uniq_id>/repayments")]
async fn get_repayments(conn: MainDbConn, uniq_id: UniqId) -> Result<Json<Vec<Repayment>>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account_repayments: Vec<Repayment> = conn.run(move |c| repayments.filter(repayments_account_id.eq(uuid)).load(c)).await?;

    Ok(Json(account_repayments))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDbConn::fairing())
        .mount("/", routes![get_account, get_expenditures, get_repayments])
}
