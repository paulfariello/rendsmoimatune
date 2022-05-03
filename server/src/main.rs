#[macro_use]
extern crate rocket;

use diesel::{prelude::*, PgConnection};
use rmmt::{prelude::*, Balance, Debt, Expenditure, Repayment, User};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use uuid;

mod account;
mod user;
mod repayment;
mod error;

use error::Error;

#[database("main")]
struct MainDbConn(PgConnection);

#[get("/api/account/<uniq_id>/expenditures")]
async fn get_expenditures(
    conn: MainDbConn,
    uniq_id: UniqId,
) -> Result<Json<Vec<Expenditure>>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account_expenditures: Vec<Expenditure> = conn
        .run(move |c| {
            expenditures
                .filter(expenditures_account_id.eq(uuid))
                .load(c)
        })
        .await?;

    Ok(Json(account_expenditures))
}

#[get("/api/account/<uniq_id>/balance")]
async fn get_balance(conn: MainDbConn, uniq_id: UniqId) -> Result<Json<Vec<Balance>>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account_debts: Vec<(Expenditure, Vec<Debt>)> = conn
        .run::<_, Result<Vec<(Expenditure, Vec<Debt>)>, diesel::result::Error>>(move |c| {
            let account_expenditures = expenditures
                .filter(expenditures_account_id.eq(uuid))
                .load(c)?;
            let account_debts = Debt::belonging_to(&account_expenditures)
                .load(c)?
                .grouped_by(&account_expenditures);
            let map: Vec<(Expenditure, Vec<Debt>)> = account_expenditures
                .into_iter()
                .zip(account_debts)
                .collect();
            Ok(map)
        })
        .await?;

    let account_repayments: Vec<Repayment> = conn
        .run(move |c| repayments.filter(repayments_account_id.eq(uuid)).load(c))
        .await?;
    let account_users: Vec<User> = conn
        .run(move |c| users.filter(users_account_id.eq(uuid)).load(c))
        .await?;

    Ok(Json(Balance::from_account(
        account_users,
        account_debts,
        account_repayments,
    )))
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(MainDbConn::fairing()).mount(
        "/",
        routes![
            account::post_account,
            account::get_account,
            get_expenditures,
            repayment::post_repayment,
            repayment::get_repayments,
            user::post_user,
            user::get_users,
            get_balance
        ],
    )
}
