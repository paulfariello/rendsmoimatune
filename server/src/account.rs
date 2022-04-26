use diesel::prelude::*;
use rmmt::{prelude::*, Account, NewAccount};
use rocket::serde::json::Json;

use crate::error::Error;
use crate::MainDbConn;

#[post("/api/account", format = "json", data = "<account>")]
pub(crate) async fn post_account(
    conn: MainDbConn,
    account: Json<NewAccount>,
) -> Result<Json<String>, Error> {
    let account: Account = conn
        .run(move |c| diesel::insert_into(accounts).values(account.into_inner()).get_result(c))
        .await?;
    let uniq_id: UniqId = account.id.into();
    Ok(Json(uniq_id.to_string()))
}

#[get("/api/account/<account_id>")]
pub(crate) async fn get_account(conn: MainDbConn, account_id: UniqId) -> Result<Json<Account>, Error> {
    let uuid: uuid::Uuid = account_id.into();
    let account: Account = conn.run(move |c| accounts.find(uuid).first(c)).await?;
    Ok(Json(account))
}
