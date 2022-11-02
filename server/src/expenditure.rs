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
                c.transaction::<(Expenditure, Vec<Debt>), diesel::result::Error, _>(|conn| {
                    let expenditure: Expenditure =
                        diesel::insert_into(rmmt::expenditures::dsl::expenditures)
                            .values(expenditure)
                            .get_result(conn)?;

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
                        .get_results(conn)?;

                    Ok((expenditure, new_debts))
                })
            })
            .await?;

        Ok(Json((expenditure, new_debts)))
    }
}

#[put(
    "/api/account/<account_id>/expenditures/<expenditure_id>",
    format = "json",
    data = "<expenditure_and_debtors>"
)]
pub(crate) async fn put_expenditure(
    conn: MainDbConn,
    account_id: UniqId,
    expenditure_id: Uuid,
    expenditure_and_debtors: Json<(Expenditure, Vec<(Uuid, i32)>)>,
) -> Result<Json<(Expenditure, Vec<Debt>)>, Error> {
    let (expenditure, debtors) = expenditure_and_debtors.into_inner();

    if account_id != expenditure.account_id || expenditure_id != expenditure.id {
        Err(Error::IdError)
    } else {
        let (expenditure, new_debts): (Expenditure, Vec<Debt>) = conn
            .run(move |c| {
                c.transaction::<(Expenditure, Vec<Debt>), diesel::result::Error, _>(|conn| {
                    let expenditure: Expenditure = diesel::update(&expenditure)
                        .set(&expenditure)
                        .get_result(conn)?;

                    diesel::delete(
                        rmmt::debts::dsl::debts
                            .filter(rmmt::debts::dsl::expenditure_id.eq(expenditure_id)),
                    )
                    .execute(conn)?;

                    let new_debts = debtors
                        .into_iter()
                        .filter(|(_, share)| *share > 0)
                        .map(|(debtor_id, share)| NewDebt {
                            debtor_id,
                            expenditure_id: expenditure.id,
                            share,
                        })
                        .collect::<Vec<_>>();

                    let new_debts: Vec<Debt> = diesel::insert_into(rmmt::debts::dsl::debts)
                        .values(new_debts)
                        .get_results(conn)?;

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
) -> Result<Json<Vec<(Expenditure, Vec<Debt>)>>, Error> {
    let account_uuid: uuid::Uuid = account_id.into();
    let expenditures: Vec<(Expenditure, Vec<Debt>)> = conn
        .run::<_, Result<_, diesel::result::Error>>(move |c| {
            let expenditures: Vec<Expenditure> = rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::account_id.eq(account_uuid))
                .load(c)?;
            let debts = Debt::belonging_to(&expenditures)
                .load(c)?
                .grouped_by(&expenditures);
            Ok(expenditures.into_iter().zip(debts).collect())
        })
        .await?;

    Ok(Json(expenditures))
}

#[delete("/api/account/<account_id>/expenditures/<expenditure_id>")]
pub(crate) async fn del_expenditure(
    conn: MainDbConn,
    account_id: UniqId,
    expenditure_id: uuid::Uuid,
) -> Result<(), Error> {
    let account_uuid: uuid::Uuid = account_id.into();
    conn.run(move |c| {
        diesel::delete(
            rmmt::expenditures::dsl::expenditures
                .filter(rmmt::expenditures::dsl::id.eq(expenditure_id))
                .filter(rmmt::expenditures::dsl::account_id.eq(account_uuid)),
        )
        .execute(c)
    })
    .await?;

    Ok(())
}
