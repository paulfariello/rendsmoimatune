use diesel::prelude::*;
use rmmt::{self, prelude::*, Balance, Balancing, Debt, Expenditure, Repayment, User};
use rocket::serde::json::Json;

use crate::error::Error;
use crate::MainDbConn;

#[get("/api/account/<uniq_id>/balance")]
pub(crate) async fn get_balance(
    conn: MainDbConn,
    uniq_id: UniqId,
) -> Result<Json<(Vec<Balance>, i64, Vec<Balancing>)>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let account_debts: Vec<(Expenditure, Vec<Debt>)> = conn
        .run::<_, Result<Vec<(Expenditure, Vec<Debt>)>, diesel::result::Error>>(move |c| {
            let account_expenditures = rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::account_id.eq(uuid))
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
        .run(move |c| {
            rmmt::repayments::dsl::repayments
                .filter(rmmt::repayments::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;
    let account_users: Vec<User> = conn
        .run(move |c| {
            rmmt::users::dsl::users
                .filter(rmmt::users::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;

    let (balances, remaining) =
        Balance::from_account(account_users, account_debts, account_repayments);

    let balancing = Balancing::from_balances(balances.clone());

    Ok(Json((balances, remaining, balancing)))
}
