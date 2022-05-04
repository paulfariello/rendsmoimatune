use diesel::prelude::*;
use rmmt::{self, prelude::*, NewUser, User};
use rocket::serde::json::Json;

use crate::error::Error;
use crate::MainDbConn;

#[post("/api/account/<account_id>/users", format = "json", data = "<user>")]
pub(crate) async fn post_user(
    conn: MainDbConn,
    account_id: UniqId,
    user: Json<NewUser>,
) -> Result<Json<User>, Error> {
    if account_id != user.account_id {
        Err(Error::IdError)
    } else {
        let user: User = conn
            .run(move |c| {
                diesel::insert_into(rmmt::users::dsl::users)
                    .values(user.into_inner())
                    .get_result(c)
            })
            .await?;
        Ok(Json(user))
    }
}

#[get("/api/account/<account_id>/users")]
pub(crate) async fn get_users(
    conn: MainDbConn,
    account_id: UniqId,
) -> Result<Json<Vec<User>>, Error> {
    let uuid: uuid::Uuid = account_id.into();
    let account_users: Vec<User> = conn
        .run(move |c| rmmt::users::dsl::users.filter(rmmt::users::dsl::account_id.eq(uuid)).load(c))
        .await?;

    Ok(Json(account_users))
}
