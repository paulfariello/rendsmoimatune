use diesel::prelude::*;
use rmmt::{prelude::*, Repayment, NewRepayment};
use rocket::serde::json::Json;

use crate::error::Error;
use crate::MainDbConn;

#[post("/api/account/<account_id>/repayments", format = "json", data = "<repayment>")]
pub(crate) async fn post_repayment(
    conn: MainDbConn,
    account_id: UniqId,
    repayment: Json<NewRepayment>,
) -> Result<Json<Repayment>, Error> {
    if account_id != repayment.account_id {
        Err(Error::IdError)
    } else {
        let repayment: Repayment = conn
            .run(move |c| diesel::insert_into(repayments).values(repayment.into_inner()).get_result(c))
            .await?;
        Ok(Json(repayment))
    }
}

#[get("/api/account/<account_id>/repayments")]
pub(crate) async fn get_repayments(conn: MainDbConn, account_id: UniqId) -> Result<Json<Vec<Repayment>>, Error> {
    let uuid: uuid::Uuid = account_id.into();
    let account_repayments: Vec<Repayment> = conn
        .run(move |c| repayments.filter(repayments_account_id.eq(uuid)).load(c))
        .await?;

    Ok(Json(account_repayments))
}
