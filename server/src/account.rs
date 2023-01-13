use diesel::prelude::*;
use rmmt::{prelude::*, Account, NewAccount, FullAccount, User, Expenditure, Debt, Repayment, Balance};
use rocket::serde::json::Json;

use crate::error::Error;
use crate::MainDbConn;

#[post("/api/account", format = "json", data = "<account>")]
pub(crate) async fn post_account(
    conn: MainDbConn,
    account: Json<NewAccount>,
) -> Result<Json<String>, Error> {
    let account: Account = conn
        .run(move |c| {
            diesel::insert_into(rmmt::accounts::dsl::accounts)
                .values(account.into_inner())
                .get_result(c)
        })
        .await?;
    let uniq_id: UniqId = account.id.into();
    Ok(Json(uniq_id.to_string()))
}

#[get("/api/account/<account_id>")]
pub(crate) async fn get_account(
    conn: MainDbConn,
    account_id: UniqId,
) -> Result<Json<Account>, Error> {
    let uuid: uuid::Uuid = account_id.into();
    let account: Account = conn
        .run(move |c| rmmt::accounts::dsl::accounts.find(uuid).first(c))
        .await?;
    Ok(Json(account))
}

#[get("/api/full_account/<account_id>")]
pub(crate) async fn get_full_account(
    conn: MainDbConn,
    account_id: UniqId,
) -> Result<Json<FullAccount>, Error> {
    let uuid: uuid::Uuid = account_id.into();
    let account: Account = conn
        .run(move |c| rmmt::accounts::dsl::accounts.find(uuid).first(c))
        .await?;
    let users: Vec<User> = conn
        .run(move |c| {
            rmmt::users::dsl::users
                .filter(rmmt::users::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;
    let expenditures: Vec<(Expenditure, Vec<Debt>)> = conn
        .run::<_, Result<_, diesel::result::Error>>(move |c| {
            let expenditures: Vec<Expenditure> = rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::account_id.eq(uuid))
                .load(c)?;
            let debts = Debt::belonging_to(&expenditures)
                .load(c)?
                .grouped_by(&expenditures);
            Ok(expenditures.into_iter().zip(debts).collect())
        })
        .await?;
    let repayments: Vec<Repayment> = conn
        .run(move |c| {
            rmmt::repayments::dsl::repayments
                .filter(rmmt::repayments::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;
    let balance = Balance::from_account(&users, &expenditures, &repayments);
    Ok(Json(FullAccount {
        account,
        users,
        expenditures,
        repayments,
        balance
    }))
}
