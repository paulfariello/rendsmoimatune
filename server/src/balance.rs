use diesel::prelude::*;
use rmmt::{self, prelude::*, Balance, Debt, Expenditure, Repayment, User, UserBalance};
use rocket::serde::json::Json;

use crate::error::Error;
use crate::MainDbConn;

#[get("/api/account/<uniq_id>/balance")]
pub(crate) async fn get_balance(conn: MainDbConn, uniq_id: UniqId) -> Result<Json<Balance>, Error> {
    let uuid: uuid::Uuid = uniq_id.into();
    let debts: Vec<(Expenditure, Vec<Debt>)> = conn
        .run::<_, Result<Vec<(Expenditure, Vec<Debt>)>, diesel::result::Error>>(move |c| {
            let expenditures = rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::account_id.eq(uuid))
                .load(c)?;
            let debts = Debt::belonging_to(&expenditures)
                .load(c)?
                .grouped_by(&expenditures);
            let map: Vec<(Expenditure, Vec<Debt>)> = expenditures.into_iter().zip(debts).collect();
            Ok(map)
        })
        .await?;

    let repayments: Vec<Repayment> = conn
        .run(move |c| {
            rmmt::repayments::dsl::repayments
                .filter(rmmt::repayments::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;
    let users: Vec<User> = conn
        .run(move |c| {
            rmmt::users::dsl::users
                .filter(rmmt::users::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;

    Ok(Json(Balance::from_account(&users, &debts, &repayments)))
}

#[get("/api/account/<uniq_id>/balances/<user_id>")]
pub(crate) async fn get_user_balance(
    conn: MainDbConn,
    uniq_id: UniqId,
    user_id: uuid::Uuid,
) -> Result<Json<UserBalance>, Error> {
    // TODO limit data fetch to user
    let uuid: uuid::Uuid = uniq_id.into();
    let debts: Vec<(Expenditure, Vec<Debt>)> = conn
        .run::<_, Result<Vec<(Expenditure, Vec<Debt>)>, diesel::result::Error>>(move |c| {
            let expenditures = rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::account_id.eq(uuid))
                .load(c)?;
            let debts = Debt::belonging_to(&expenditures)
                .load(c)?
                .grouped_by(&expenditures);
            let map: Vec<(Expenditure, Vec<Debt>)> = expenditures.into_iter().zip(debts).collect();
            Ok(map)
        })
        .await?;

    let repayments: Vec<Repayment> = conn
        .run(move |c| {
            rmmt::repayments::dsl::repayments
                .filter(rmmt::repayments::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;
    let users: Vec<User> = conn
        .run(move |c| {
            rmmt::users::dsl::users
                .filter(rmmt::users::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;

    let balance = Balance::from_account(&users, &debts, &repayments);
    Ok(Json(
        balance
            .user_balances
            .into_iter()
            .find(|b| b.user_id == user_id)
            .unwrap(),
    ))
}
