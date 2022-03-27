#[cfg_attr(feature = "db", macro_use)]
#[cfg(feature = "db")]
extern crate diesel;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[cfg(feature = "db")]
mod schema;
pub mod prelude;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Queryable))]
pub struct Account {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Queryable))]
pub struct Expenditure {
    pub name: String,
    pub date: DateTime<Utc>,
    pub amount: u64,
    pub payer: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
