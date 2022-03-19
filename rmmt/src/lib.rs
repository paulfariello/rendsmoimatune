use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub uuid: Uuid,
    pub name: String,
    pub expenditures: Vec<Expenditure>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
