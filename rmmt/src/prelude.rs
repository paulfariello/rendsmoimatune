#[cfg(feature = "db")]
pub use crate::schema::accounts::dsl::*;
#[cfg(feature = "db")]
pub use crate::schema::expenditures::dsl::{expenditures, account_id as expenditures_account_id};
#[cfg(feature = "db")]
pub use crate::schema::repayments::dsl::{repayments, account_id as repayments_account_id};
