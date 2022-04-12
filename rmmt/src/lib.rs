#[cfg_attr(feature = "db", macro_use)]
#[cfg(feature = "db")]
extern crate diesel;

use chrono::NaiveDate;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::info;


#[cfg(feature = "db")]
mod schema;
pub mod prelude;

#[cfg(feature = "db")]
use schema::{accounts, expenditures, repayments, users, debts};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable))]
#[cfg_attr(feature = "db", table_name = "accounts")]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "db", belongs_to(Account))]
#[cfg_attr(feature = "db", table_name = "expenditures")]
pub struct Expenditure {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub amount: i32,
    pub payer_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "db", belongs_to(Account))]
#[cfg_attr(feature = "db", table_name = "repayments")]
pub struct Repayment {
    pub id: Uuid,
    pub account_id: Uuid,
    pub date: NaiveDate,
    pub amount: i32,
    pub payer_id: Uuid,
    pub beneficiary_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "db", belongs_to(Account))]
#[cfg_attr(feature = "db", table_name = "users")]
pub struct User {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "db", belongs_to(Expenditure))]
#[cfg_attr(feature = "db", belongs_to(User, foreign_key = "debtor_id"))]
#[cfg_attr(feature = "db", table_name = "debts")]
pub struct Debt {
    pub id: Uuid,
    pub debtor_id: Uuid,
    pub expenditure_id: Uuid,
    pub share: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Balance {
    pub user_id: Uuid,
    pub amount: i32,
}

impl Balance {
    pub fn from_account(
        users: Vec<User>,
        debts: Vec<(Expenditure, Vec<Debt>)>,
        repayments: Vec<Repayment>,
    ) -> Vec<Balance> {
        let mut balances: HashMap<Uuid, Balance> = users
            .iter()
            .map(|u| {
                (
                    u.id,
                    Balance {
                        user_id: u.id.clone(),
                        amount: 0,
                    },
                )
            })
            .collect();

        for (expenditure, debts) in debts {
            // Update payer balance
            let balance = balances.get_mut(&expenditure.payer_id).unwrap();
            info!("{}: +{}", expenditure.payer_id, expenditure.amount);
            balance.amount += expenditure.amount;

            // Update deptors balances
            let share_sum: i32 = debts.iter().map(|d| d.share).sum();

            for debt in debts {
                let balance = balances.get_mut(&debt.debtor_id).unwrap();
                info!("{}: -{}", debt.debtor_id, (expenditure.amount as f64 * (debt.share as f64 / share_sum as f64)) as i32);
                balance.amount -= (expenditure.amount as f64 * (debt.share as f64 / share_sum as f64)) as i32;
            }
        }


        for repayment in repayments {
            let balance = balances.get_mut(&repayment.payer_id).unwrap();
            balance.amount += repayment.amount;
            info!("{}: +{}", repayment.payer_id, repayment.amount);

            let balance = balances.get_mut(&repayment.beneficiary_id).unwrap();
            balance.amount -= repayment.amount;
            info!("{}: -{}", repayment.beneficiary_id, repayment.amount);
        }

        info!("{:?}", balances);
        balances.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance() {
        // Given
        let account_id = Uuid::new_v4();
        let users = vec![User {
            id: Uuid::new_v4(),
            account_id,
            name: "user1".to_string(),
        },
        User {
            id: Uuid::new_v4(),
            account_id,
            name: "user1".to_string(),
        }];

        let expenditure = Expenditure {
            id: Uuid::new_v4(),
            account_id,
            payer_id: users[0].id.clone(),
            amount: 10,
            name: "expenditure1".to_string(),
            date: NaiveDate::from_yo(2015, 73),
        };

        let debts = vec![(expenditure.clone(), vec![Debt {
            id: Uuid::new_v4(),
            expenditure_id: expenditure.id.clone(),
            debtor_id: users[0].id.clone(),
            share: 1,
        },
        Debt {
            id: Uuid::new_v4(),
            expenditure_id: expenditure.id.clone(),
            debtor_id: users[1].id.clone(),
            share: 1,
        }])];

        let repayments = vec![];

        // When
        let balances = Balance::from_account(users.clone(), debts, repayments);

        // Then
        assert_eq!(balances[0], Balance {
            user_id: users[0].id.clone(),
            amount: 5
        });
        assert_eq!(balances[1], Balance {
            user_id: users[1].id.clone(),
            amount: -5
        });

    }
}
