use diesel::prelude::*;
use rmmt::{prelude::*, Debt, Expenditure, NewDebt, NewExpenditure};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::error::Error;
use crate::MainDbConn;

#[post(
    "/api/account/<account_id>/expenditures",
    format = "json",
    data = "<expenditure_and_debtors>"
)]
pub(crate) async fn post_expenditure(
    conn: MainDbConn,
    account_id: UniqId,
    expenditure_and_debtors: Json<(NewExpenditure, Vec<(Uuid, i32)>)>,
) -> Result<Json<(Expenditure, Vec<Debt>)>, Error> {
    let (expenditure, debtors) = expenditure_and_debtors.into_inner();

    if account_id != expenditure.account_id {
        Err(Error::IdError)
    } else {
        let (expenditure, new_debts): (Expenditure, Vec<Debt>) = conn
            .run(move |c| {
                c.transaction::<(Expenditure, Vec<Debt>), diesel::result::Error, _>(|| {
                    let expenditure: Expenditure = diesel::insert_into(rmmt::expenditures::dsl::expenditures)
                        .values(expenditure)
                        .get_result(c)?;

                    let new_debts = debtors
                        .into_iter()
                        .map(|(debtor_id, share)| NewDebt {
                            debtor_id,
                            expenditure_id: expenditure.id,
                            share,
                        })
                        .collect::<Vec<_>>();

                    let new_debts: Vec<Debt> = diesel::insert_into(rmmt::debts::dsl::debts)
                        .values(new_debts)
                        .get_results(c)?;

                    Ok((expenditure, new_debts))
                })
            })
            .await?;

        Ok(Json((expenditure, new_debts)))
    }
}

#[get("/api/account/<account_id>/expenditures")]
pub(crate) async fn get_expenditures(
    conn: MainDbConn,
    account_id: UniqId,
) -> Result<Json<Vec<Expenditure>>, Error> {
    let account_uuid: uuid::Uuid = account_id.into();
    let account_expenditures: Vec<Expenditure> = conn
        .run(move |c| {
            rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::account_id.eq(account_uuid))
                .load(c)
        })
        .await?;

    Ok(Json(account_expenditures))
}

#[delete("/api/account/<account_id>/expenditures/<expenditure_id>")]
pub(crate) async fn del_expenditure(
    conn: MainDbConn,
    account_id: UniqId,
    expenditure_id: uuid::Uuid,
) -> Result<(), Error> {
    let account_uuid: uuid::Uuid = account_id.into();
    conn.run(move |c| {
        diesel::delete(rmmt::expenditures::dsl::expenditures.filter(rmmt::expenditures::dsl::id.eq(expenditure_id))).execute(c)
    })
    .await?;

    Ok(())
}
