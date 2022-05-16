use diesel::prelude::*;
use rmmt::{self, prelude::*, NewRepayment, Repayment};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::error::Error;
use crate::MainDbConn;

#[post(
    "/api/account/<account_id>/repayments",
    format = "json",
    data = "<repayment>"
)]
pub(crate) async fn post_repayment(
    conn: MainDbConn,
    account_id: UniqId,
    repayment: Json<NewRepayment>,
) -> Result<Json<Repayment>, Error> {
    if account_id != repayment.account_id {
        Err(Error::IdError)
    } else {
        let repayment: Repayment = conn
            .run(move |c| {
                diesel::insert_into(rmmt::repayments::dsl::repayments)
                    .values(repayment.into_inner())
                    .get_result(c)
            })
            .await?;
        Ok(Json(repayment))
    }
}

#[put(
    "/api/account/<account_id>/repayments/<repayment_id>",
    format = "json",
    data = "<repayment>"
)]
pub(crate) async fn put_repayment(
    conn: MainDbConn,
    account_id: UniqId,
    repayment_id: Uuid,
    repayment: Json<Repayment>,
) -> Result<Json<Repayment>, Error> {
    if account_id != repayment.account_id || repayment_id != repayment.id {
        Err(Error::IdError)
    } else {
        let repayment = repayment.into_inner();
        let repayment: Repayment = conn
            .run(move |c| {
                diesel::update(&repayment)
                    .set(&repayment)
                    .get_result(c)
            })
            .await?;
        Ok(Json(repayment))
    }
}

#[get("/api/account/<account_id>/repayments")]
pub(crate) async fn get_repayments(
    conn: MainDbConn,
    account_id: UniqId,
) -> Result<Json<Vec<Repayment>>, Error> {
    let uuid: uuid::Uuid = account_id.into();
    let account_repayments: Vec<Repayment> = conn
        .run(move |c| {
            rmmt::repayments::dsl::repayments
                .filter(rmmt::repayments::dsl::account_id.eq(uuid))
                .load(c)
        })
        .await?;

    Ok(Json(account_repayments))
}

#[delete("/api/account/<account_id>/repayments/<repayment_id>")]
pub(crate) async fn del_repayment(
    conn: MainDbConn,
    account_id: UniqId,
    repayment_id: uuid::Uuid,
) -> Result<(), Error> {
    let account_uuid: uuid::Uuid = account_id.into();
    conn.run(move |c| {
        diesel::delete(
            rmmt::repayments::dsl::repayments
                .filter(rmmt::repayments::dsl::id.eq(repayment_id))
                .filter(rmmt::repayments::dsl::account_id.eq(account_uuid)),
        )
        .execute(c)
    })
    .await?;

    Ok(())
}
