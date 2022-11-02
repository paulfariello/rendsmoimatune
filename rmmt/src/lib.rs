#[cfg_attr(feature = "db", macro_use)]
#[cfg(feature = "db")]
extern crate diesel;

use std::cmp;
use std::collections::HashMap;

use chrono::NaiveDate;
use num::rational::Rational64;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod prelude;
#[cfg(feature = "db")]
mod schema;
pub mod uniqid;

#[cfg(feature = "db")]
pub use schema::{accounts, debts, expenditures, repayments, users};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable))]
#[cfg_attr(feature = "db", diesel(table_name = accounts))]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = accounts))]
pub struct NewAccount {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "db",
    derive(Identifiable, Queryable, Associations, AsChangeset)
)]
#[cfg_attr(feature = "db", diesel(belongs_to(Account)))]
#[cfg_attr(feature = "db", diesel(table_name = users))]
pub struct User {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = users))]
pub struct NewUser {
    pub account_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "db",
    derive(Identifiable, Queryable, Associations, AsChangeset)
)]
#[cfg_attr(feature = "db", diesel(belongs_to(Account)))]
#[cfg_attr(feature = "db", diesel(table_name = expenditures))]
pub struct Expenditure {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub amount: i32,
    pub payer_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = expenditures))]
pub struct NewExpenditure {
    pub account_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub amount: i32,
    pub payer_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "db",
    derive(Identifiable, Queryable, Associations, AsChangeset)
)]
#[cfg_attr(feature = "db", diesel(belongs_to(Account)))]
#[cfg_attr(feature = "db", diesel(table_name = repayments))]
pub struct Repayment {
    pub id: Uuid,
    pub account_id: Uuid,
    pub date: NaiveDate,
    pub amount: i32,
    pub payer_id: Uuid,
    pub beneficiary_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = repayments))]
pub struct NewRepayment {
    pub account_id: Uuid,
    pub date: NaiveDate,
    pub amount: i32,
    pub payer_id: Uuid,
    pub beneficiary_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "db", diesel(belongs_to(Expenditure)))]
#[cfg_attr(feature = "db", diesel(belongs_to(User, foreign_key = debtor_id)))]
#[cfg_attr(feature = "db", diesel(table_name = debts))]
pub struct Debt {
    pub id: Uuid,
    pub debtor_id: Uuid,
    pub expenditure_id: Uuid,
    pub share: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = debts))]
pub struct NewDebt {
    pub debtor_id: Uuid,
    pub expenditure_id: Uuid,
    pub share: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserBalance {
    pub user_id: Uuid,
    pub amount: i64,
}

struct TmpBalance {
    pub user_id: Uuid,
    pub credit: i64,
    pub debts: Vec<Rational64>,
}

impl TmpBalance {
    fn result(&self) -> i64 {
        // round debts in order to favor the ones who advanced money
        let debts = self.debts.iter().sum::<Rational64>();
        let debts = if Rational64::new(self.credit, 1) > debts {
            debts.floor().to_integer()
        } else {
            debts.ceil().to_integer()
        };

        self.credit - debts
    }
}

impl From<TmpBalance> for UserBalance {
    fn from(tmp: TmpBalance) -> UserBalance {
        let result = tmp.result();
        UserBalance {
            user_id: tmp.user_id,
            amount: result,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Balancing {
    pub payer_id: Uuid,
    pub beneficiary_id: Uuid,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Balance {
    /// Represent each user own balance.
    pub user_balances: Vec<UserBalance>,
    /// Difference between account and sum of user balances.
    /// Mainly due to rounding in expenditure shares.
    pub account_remaining: i64,
    /// List of possible repayments.
    pub balancing: Vec<Balancing>,
    /// Remaining balances once all proposed repayment are done.
    pub balancing_remaining: Vec<UserBalance>,
}

impl Balance {
    pub fn from_account(
        users: Vec<User>,
        debts: Vec<(Expenditure, Vec<Debt>)>,
        repayments: Vec<Repayment>,
    ) -> Self {
        let (mut user_balances, account_remaining) =
            Self::get_user_balances(users, debts, repayments);

        let (balancing, balancing_remaining) = Self::get_balancing(&mut user_balances);

        Self {
            user_balances,
            account_remaining,
            balancing,
            balancing_remaining,
        }
    }

    /// Create balancing from a vector of balances.
    ///
    /// Ensure balancing doesn't change given new repayments are done.
    fn get_balancing(balances: &mut Vec<UserBalance>) -> (Vec<Balancing>, Vec<UserBalance>) {
        let mut balancing = Vec::new();

        // Sort to ensure idempotence
        balances.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());

        let mut creditors = balances
            .iter()
            .filter(|b| b.amount > 0)
            .cloned()
            .collect::<Vec<_>>();
        let mut debtors = balances
            .iter()
            .filter(|b| b.amount < 0)
            .cloned()
            .collect::<Vec<_>>();

        while !creditors.is_empty() && !debtors.is_empty() {
            let mut debtor = debtors.pop().unwrap();
            let mut creditor = creditors.pop().unwrap();

            let amount = cmp::min(-debtor.amount, creditor.amount);

            balancing.push(Balancing {
                payer_id: debtor.user_id,
                beneficiary_id: creditor.user_id,
                amount,
            });

            debtor.amount += amount;
            creditor.amount -= amount;

            if debtor.amount < 0 {
                debtors.push(debtor);
            }
            if creditor.amount > 0 {
                creditors.push(creditor);
            }
        }

        let remaining = [&creditors[..], &debtors[..]].concat();
        (balancing, remaining)
    }

    /// Compute balance for each user.
    fn get_user_balances(
        users: Vec<User>,
        debts: Vec<(Expenditure, Vec<Debt>)>,
        repayments: Vec<Repayment>,
    ) -> (Vec<UserBalance>, i64) {
        let mut balances: HashMap<Uuid, TmpBalance> = users
            .iter()
            .map(|u| {
                (
                    u.id,
                    TmpBalance {
                        user_id: u.id.clone(),
                        credit: 0,
                        debts: Vec::new(),
                    },
                )
            })
            .collect();

        for (expenditure, debts) in debts {
            // Update payer balance
            let balance = Self::get_balance(&mut balances, &expenditure.payer_id);
            balance.credit += expenditure.amount as i64;

            // Update deptors balances
            let share_sum: i32 = debts.iter().map(|d| d.share).sum();

            for debt in &debts {
                let balance = Self::get_balance(&mut balances, &debt.debtor_id);
                balance.debts.push(Rational64::new(
                    expenditure.amount as i64 * debt.share as i64,
                    share_sum as i64,
                ));
            }
        }

        let mut balances: HashMap<Uuid, UserBalance> =
            balances.into_iter().map(|(k, v)| (k, v.into())).collect::<HashMap<_, _>>();

        for repayment in repayments {
            let balance = Self::get_balance(&mut balances, &repayment.payer_id);
            balance.amount += repayment.amount as i64;

            let balance = Self::get_balance(&mut balances, &repayment.beneficiary_id);
            balance.amount -= repayment.amount as i64;
        }

        let balances: Vec<UserBalance> =
            balances.into_values().collect::<Vec<_>>();

        let remaining: i64 = balances.iter().map(|b| b.amount).sum();

        (balances, remaining)
    }

    #[inline]
    fn get_balance<'a, T>(
        balances: &'a mut HashMap<Uuid, T>,
        id: &Uuid,
    ) -> &'a mut T {
        balances
            .get_mut(id)
            .expect(&format!("Corrupted db? Missing user {} in balances", id))
    }
}

#[cfg(test)]
mod tests;
