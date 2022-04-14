#[cfg_attr(feature = "db", macro_use)]
#[cfg(feature = "db")]
extern crate diesel;

use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod prelude;
#[cfg(feature = "db")]
mod schema;

#[cfg(feature = "db")]
use schema::{accounts, debts, expenditures, repayments, users};

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
            info!("expenditure {} {}: {} += {}", expenditure.id, expenditure.payer_id, balance.amount, expenditure.amount);
            balance.amount += expenditure.amount;

            // Update deptors balances
            let share_sum: i32 = debts.iter().map(|d| d.share).sum();

            for debt in debts {
                let balance = balances.get_mut(&debt.debtor_id).unwrap();
                info!(
                    "debt {}: {} -= {}",
                    debt.debtor_id,
                    balance.amount,
                    (expenditure.amount as f64 * (debt.share as f64 / share_sum as f64)) as i32
                );
                balance.amount -=
                    (expenditure.amount as f64 * (debt.share as f64 / share_sum as f64)) as i32;
            }
        }

        for repayment in repayments {
            let balance = balances.get_mut(&repayment.payer_id).unwrap();
            balance.amount += repayment.amount;
            info!("repayment {}: {} += {}", repayment.payer_id, balance.amount, repayment.amount);

            let balance = balances.get_mut(&repayment.beneficiary_id).unwrap();
            balance.amount -= repayment.amount;
            info!("repayment {}: {} -= {}", repayment.beneficiary_id, balance.amount, repayment.amount);
        }

        let balances = balances.into_values().collect::<Vec<_>>();
        assert_eq!(0, balances.iter().map(|b| b.amount).sum(), "balance doesn't sum up to 0");

        balances
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    fn uuid(name: &str) -> Uuid {
        Uuid::new_v5(&uuid::NAMESPACE_X500, name)
    }

    fn user(name: &str) -> User {
        User {
            id: uuid(name),
            account_id: uuid("account"),
            name: name.to_string(),
        }
    }

    fn expenditure(
        payer: &str,
        amount: i32,
        debtors: Vec<(&str, i32)>,
    ) -> (Expenditure, Vec<Debt>) {
        let expenditure_id = Uuid::new_v4();
        (
            Expenditure {
                id: expenditure_id,
                account_id: uuid("account"),
                payer_id: uuid(payer),
                amount,
                name: format!("{}", expenditure_id),
                date: NaiveDate::from_yo(2015, 73),
            },
            debtors
                .iter()
                .map(|(user, share)| Debt {
                    id: Uuid::new_v4(),
                    expenditure_id,
                    debtor_id: uuid(user),
                    share: *share,
                })
                .collect::<Vec<_>>(),
        )
    }

    fn repayment(payer: &str, beneficiary: &str, amount: i32) -> Repayment {
        Repayment {
            id: Uuid::new_v4(),
            account_id: uuid("account"),
            date: NaiveDate::from_yo(2015, 73),
            payer_id: uuid(payer),
            beneficiary_id: uuid(beneficiary),
            amount,
        }
    }

    fn assert_balance(balances: Vec<Balance>, reference: Vec<(&str, i32)>) {
        let map_balances = balances
            .iter()
            .map(|b| (b.user_id.clone(), b))
            .collect::<HashMap<_, _>>();
        assert_eq!(0, balances.iter().map(|b| b.amount).sum(), "balance doesn't sum up to 0");
        for (user, amount) in reference {
            let balance = map_balances.get(&uuid(user)).unwrap();
            assert_eq!(
                balance.amount, amount,
                "invalid balance {} (expected {}) for {}",
                balance.amount, amount, user
            );
        }
    }

    #[test]
    fn balance_simple() {
        // Given
        let users = vec![user("user1"), user("user2")];
        let debts = vec![expenditure("user1", 10, vec![("user1", 1), ("user2", 1)])];
        let repayments = vec![];

        // When
        let balances = Balance::from_account(users, debts, repayments);

        // Then
        assert_balance(balances, vec![("user1", 5), ("user2", -5)]);
    }

    #[test]
    fn balance_with_repayment() {
        // Given
        let users = vec![user("user1"), user("user2")];
        let debts = vec![expenditure("user1", 10, vec![("user1", 1), ("user2", 1)])];
        let repayments = vec![repayment("user2", "user1", 5)];

        // When
        let balances = Balance::from_account(users, debts, repayments);

        // Then
        assert_balance(balances, vec![("user1", 0), ("user2", 0)]);
    }

    #[test]
    fn balance_with_few_expenditures() {
        // Given
        let users = vec![user("user1"), user("user2")];
        let debts = vec![
            expenditure("user1", 10, vec![("user1", 1), ("user2", 1)]),
            expenditure("user1", 100, vec![("user1", 7), ("user2", 3)]),
            expenditure("user2", 50, vec![("user1", 7), ("user2", 3)]),
        ];
        let repayments = vec![repayment("user2", "user1", 5)];

        // When
        let balances = Balance::from_account(users, debts, repayments);

        // Then
        assert_balance(balances, vec![("user1", -5), ("user2", 5)]);
    }
}
