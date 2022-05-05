use super::*;
use test_log::test;

fn uuid(name: &str) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_X500, name.as_bytes())
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

fn assert_balance(balances: Vec<Balance>, reference: Vec<(&str, i64)>, remaining: i64, remaining_ref: i64) {
    let map_balances = balances
        .iter()
        .map(|b| (b.user_id.clone(), b))
        .collect::<HashMap<_, _>>();
    assert_eq!(
        remaining,
        remaining_ref,
        "balance remaining mismatch {} (expected {})",
        remaining,
        remaining_ref,
    );
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
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 5), ("user2", -5)], remaining, 0);
}

#[test]
fn balance_with_repayment() {
    // Given
    let users = vec![user("user1"), user("user2")];
    let debts = vec![expenditure("user1", 10, vec![("user1", 1), ("user2", 1)])];
    let repayments = vec![repayment("user2", "user1", 5)];

    // When
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 0), ("user2", 0)], remaining, 0);
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
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", -5), ("user2", 5)], remaining, 0);
}

#[test]
fn balance_with_remaining() {
    // Given
    let users = vec![user("user1"), user("user2")];
    let debts = vec![expenditure("user1", 7, vec![("user1", 1), ("user2", 2)])];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 5), ("user2", -4)], remaining, 1);
}

#[test]
fn balance_with_even_remaining() {
    // Given
    let users = vec![user("user1"), user("user2")];
    let debts = vec![expenditure("user1", 1, vec![("user1", 1), ("user2", 1)])];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 1), ("user2", 0)], remaining, 1);
}

#[test]
fn resolved_remaining() {
    // Given
    let users = vec![user("user1"), user("user2"), user("user3")];
    let debts = vec![expenditure("user1", 100, vec![("user2", 1), ("user1", 1), ("user3", 1)]), expenditure("user2", 200, vec![("user1", 1), ("user2", 1), ("user3", 1)])];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 0), ("user2", 100), ("user3", -100)], remaining, 0);
}

#[test]
fn resolved_remaining_all_floored() {
    // Given
    let users = vec![user("user1"), user("user2"), user("user3")];
    let debts = vec![expenditure("user1", 100, vec![("user2", 1), ("user1", 1), ("user3", 1)]), expenditure("user2", 100, vec![("user1", 1), ("user2", 1), ("user3", 1)]), expenditure("user3", 100, vec![("user1", 1), ("user2", 1), ("user3", 1)])];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::from_account(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 0), ("user2", 0), ("user3", 0)], remaining, 0);
}
