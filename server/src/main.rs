#[macro_use]
extern crate rocket;

use diesel::PgConnection;
use rocket_sync_db_pools::database;

mod account;
mod balance;
mod error;
mod expenditure;
mod repayment;
mod user;

#[database("main")]
struct MainDbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build().attach(MainDbConn::fairing()).mount(
        "/",
        routes![
            account::post_account,
            account::get_account,
            expenditure::post_expenditure,
            expenditure::get_expenditures,
            expenditure::del_expenditure,
            expenditure::put_expenditure,
            repayment::post_repayment,
            repayment::get_repayments,
            repayment::del_repayment,
            repayment::put_repayment,
            user::post_user,
            user::get_users,
            balance::get_balance,
        ],
    )
}
