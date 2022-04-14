#[cfg(feature = "db")]
pub use crate::schema::accounts::dsl::*;
#[cfg(feature = "db")]
pub use crate::schema::expenditures::dsl::{account_id as expenditures_account_id, expenditures};
#[cfg(feature = "db")]
pub use crate::schema::repayments::dsl::{account_id as repayments_account_id, repayments};
#[cfg(feature = "db")]
pub use crate::schema::users::dsl::{account_id as users_account_id, users};
