use super::*;
use test_log::test;

fn uuid(name_or_uuid: &str) -> Uuid {
    Uuid::parse_str(name_or_uuid)
        .unwrap_or(Uuid::new_v5(&Uuid::NAMESPACE_X500, name_or_uuid.as_bytes()))
}

fn user(name: &str) -> User {
    User {
        id: uuid(name),
        account_id: uuid("account"),
        name: name.to_string(),
    }
}

fn expenditure(payer: &str, amount: i32, debtors: Vec<(&str, i32)>) -> (Expenditure, Vec<Debt>) {
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

fn assert_balance(
    balances: Vec<UserBalance>,
    reference: Vec<(&str, i64)>,
    remaining: i64,
    remaining_ref: i64,
) {
    let map_balances = balances
        .iter()
        .map(|b| (b.user_id.clone(), b))
        .collect::<HashMap<_, _>>();
    for (user, amount) in reference {
        let balance = map_balances.get(&uuid(user)).unwrap();
        assert_eq!(
            balance.amount, amount,
            "invalid balance {} (expected {}) for {}",
            balance.amount, amount, user
        );
    }
    assert_eq!(
        remaining, remaining_ref,
        "balance remaining mismatch {} (expected {})",
        remaining, remaining_ref,
    );
}

fn user_balance(user: &str, amount: i64) -> UserBalance {
    UserBalance {
        user_id: uuid(user),
        amount,
    }
}

fn assert_balancing(balancing: Vec<Balancing>, reference: Vec<(&str, &str, i64)>) {
    let map_balancing = balancing
        .into_iter()
        .map(|b| ((b.payer_id.clone(), b.beneficiary_id.clone()), b.amount))
        .collect::<HashMap<_, _>>();
    for (payer, beneficiary, amount_ref) in reference {
        let amount = map_balancing.get(&(uuid(payer), uuid(beneficiary)));
        assert_eq!(
            amount,
            Some(&amount_ref),
            "invalid balancing {:?} (expected {}) from {} to {}",
            amount,
            amount_ref,
            payer,
            beneficiary
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
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

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
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

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
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

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
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 5), ("user2", -5)], remaining, 0);
}

#[test]
fn balance_with_even_remaining_favour_payers() {
    // Given
    let users = vec![user("user1"), user("user2")];
    let debts = vec![expenditure("user1", 1, vec![("user1", 1), ("user2", 1)])];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

    // Then
    assert_balance(balances, vec![("user1", 1), ("user2", -1)], remaining, 0);
}

#[test]
fn balance_with_fractional_remaining_favour_payers_and_have_remaining() {
    // Given
    let users = vec![user("user1"), user("user2"), user("user3")];
    let debts = vec![
        expenditure("user1", 100, vec![("user2", 1), ("user1", 1), ("user3", 1)]),
        expenditure("user2", 100, vec![("user1", 1), ("user2", 1), ("user3", 1)]),
    ];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

    // Then
    assert_balance(
        balances,
        vec![("user1", 34), ("user2", 34), ("user3", -67)],
        remaining,
        1,
    );
}

#[test]
fn balance_with_resolved_fractional_remaining_has_no_remaining() {
    // Given
    let users = vec![user("user1"), user("user2"), user("user3")];
    let debts = vec![
        expenditure("user1", 100, vec![("user2", 1), ("user1", 1), ("user3", 1)]),
        expenditure("user2", 100, vec![("user1", 1), ("user2", 1), ("user3", 1)]),
        expenditure("user3", 100, vec![("user1", 1), ("user2", 1), ("user3", 1)]),
    ];
    let repayments = vec![];

    // When
    let (balances, remaining) = Balance::get_user_balances(users, debts, repayments);

    // Then
    assert_balance(
        balances,
        vec![("user1", 0), ("user2", 0), ("user3", 0)],
        remaining,
        0,
    );
}

#[test]
fn balancing() {
    // Given
    let mut balances = vec![user_balance("user1", 100), user_balance("user2", -100)];

    // When
    let (balancing, _) = Balance::get_balancing(&mut balances);

    // Then
    assert_balancing(balancing, vec![("user2", "user1", 100)]);
}

#[test]
fn multi_balancing() {
    // Given
    let mut balances = vec![
        user_balance("user1", 100),
        user_balance("user2", -50),
        user_balance("user3", -50),
    ];

    // When
    let (balancing, _) = Balance::get_balancing(&mut balances);

    // Then
    assert_balancing(
        balancing,
        vec![("user2", "user1", 50), ("user3", "user1", 50)],
    );
}

#[test]
fn balance_is_stable() {
    // Given
    let users = vec![
        user("alice"),
        user("bob"),
        user("charlie"),
        user("daniel"),
        user("erwan"),
    ];
    let debts = vec![
        expenditure(
            "alice",
            215,
            vec![
                ("alice", 1),
                ("bob", 1),
                ("charlie", 1),
                ("daniel", 1),
                ("erwan", 1),
            ],
        ),
        expenditure(
            "bob",
            215,
            vec![
                ("alice", 1),
                ("bob", 1),
                ("charlie", 1),
                ("daniel", 1),
                ("erwan", 1),
            ],
        ),
    ];
    let repayments = vec![];
    let (mut balances, _remaining) =
        Balance::get_user_balances(users.clone(), debts.clone(), repayments);
    let (balancing, _) = Balance::get_balancing(&mut balances);
    assert_balancing(
        balancing,
        vec![
            ("charlie", "bob", 86),
            ("daniel", "alice", 86),
            ("erwan", "alice", 43),
            ("erwan", "bob", 43),
        ],
    );

    // When
    let repayments = vec![repayment("daniel", "alice", 86)];
    let (mut balances, _remaining) = Balance::get_user_balances(users, debts, repayments);
    let (balancing, _) = Balance::get_balancing(&mut balances);

    // Then
    assert_balancing(
        balancing,
        vec![
            ("charlie", "bob", 86),
            ("erwan", "alice", 43),
            ("erwan", "bob", 43),
        ],
    );
}

#[test]
fn non_reg() {
    // Given
    let users = vec![
        user("1720536c-9a51-46d4-952f-202544b914a1"),
        user("46106d30-b41a-4e3e-bb2a-81e3bff8978f"),
        user("9535f524-291f-48e7-b6a9-4be24fa06c7f"),
        user("b5535a59-94d8-4885-8cef-6418ec5c7e58"),
        user("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5"),
    ];
    let debts = vec![
        expenditure(
            "b5535a59-94d8-4885-8cef-6418ec5c7e58",
            2300,
            vec![
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "1720536c-9a51-46d4-952f-202544b914a1",
            42288,
            vec![
                ("1720536c-9a51-46d4-952f-202544b914a1", 1),
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "9535f524-291f-48e7-b6a9-4be24fa06c7f",
            3600,
            vec![
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "cf99aaff-8f7e-43b6-86bf-cb6026ed08a5",
            3500,
            vec![
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "9535f524-291f-48e7-b6a9-4be24fa06c7f",
            4100,
            vec![
                ("1720536c-9a51-46d4-952f-202544b914a1", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
            ],
        ),
        expenditure(
            "46106d30-b41a-4e3e-bb2a-81e3bff8978f",
            10300,
            vec![
                ("1720536c-9a51-46d4-952f-202544b914a1", 1),
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "cf99aaff-8f7e-43b6-86bf-cb6026ed08a5",
            6425,
            vec![
                ("1720536c-9a51-46d4-952f-202544b914a1", 1),
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "cf99aaff-8f7e-43b6-86bf-cb6026ed08a5",
            27600,
            vec![
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "b5535a59-94d8-4885-8cef-6418ec5c7e58",
            10300,
            vec![
                ("1720536c-9a51-46d4-952f-202544b914a1", 1),
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "b5535a59-94d8-4885-8cef-6418ec5c7e58",
            2980,
            vec![
                ("1720536c-9a51-46d4-952f-202544b914a1", 1),
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
        expenditure(
            "46106d30-b41a-4e3e-bb2a-81e3bff8978f",
            30900,
            vec![
                ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 1),
                ("b5535a59-94d8-4885-8cef-6418ec5c7e58", 1),
                ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 1),
            ],
        ),
    ];
    let repayments = vec![];
    let (mut balances, remaining) =
        Balance::get_user_balances(users.clone(), debts.clone(), repayments);
    assert_balance(
        balances.clone(),
        vec![
            ("1720536c-9a51-46d4-952f-202544b914a1", 25631),
            ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 6218),
            ("9535f524-291f-48e7-b6a9-4be24fa06c7f", -11800),
            ("b5535a59-94d8-4885-8cef-6418ec5c7e58", -20625),
            ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 576),
        ],
        remaining,
        0,
    );
    let (balancing, _) = Balance::get_balancing(&mut balances);
    assert_balancing(
        balancing,
        vec![
            (
                "9535f524-291f-48e7-b6a9-4be24fa06c7f",
                "1720536c-9a51-46d4-952f-202544b914a1",
                11800,
            ),
            (
                "b5535a59-94d8-4885-8cef-6418ec5c7e58",
                "cf99aaff-8f7e-43b6-86bf-cb6026ed08a5",
                576,
            ),
            (
                "b5535a59-94d8-4885-8cef-6418ec5c7e58",
                "46106d30-b41a-4e3e-bb2a-81e3bff8978f",
                6218,
            ),
            (
                "b5535a59-94d8-4885-8cef-6418ec5c7e58",
                "1720536c-9a51-46d4-952f-202544b914a1",
                13831,
            ),
        ],
    );

    // When
    let repayments = vec![repayment(
        "9535f524-291f-48e7-b6a9-4be24fa06c7f",
        "1720536c-9a51-46d4-952f-202544b914a1",
        11800,
    )];
    let (mut balances, remaining) = Balance::get_user_balances(users, debts, repayments);
    assert_balance(
        balances.clone(),
        vec![
            ("1720536c-9a51-46d4-952f-202544b914a1", 13831),
            ("46106d30-b41a-4e3e-bb2a-81e3bff8978f", 6218),
            ("9535f524-291f-48e7-b6a9-4be24fa06c7f", 0),
            ("b5535a59-94d8-4885-8cef-6418ec5c7e58", -20625),
            ("cf99aaff-8f7e-43b6-86bf-cb6026ed08a5", 576),
        ],
        remaining,
        0,
    );
    let (balancing, _) = Balance::get_balancing(&mut balances);

    // Then
    assert_balancing(
        balancing,
        vec![
            (
                "b5535a59-94d8-4885-8cef-6418ec5c7e58",
                "cf99aaff-8f7e-43b6-86bf-cb6026ed08a5",
                576,
            ),
            (
                "b5535a59-94d8-4885-8cef-6418ec5c7e58",
                "46106d30-b41a-4e3e-bb2a-81e3bff8978f",
                6218,
            ),
            (
                "b5535a59-94d8-4885-8cef-6418ec5c7e58",
                "1720536c-9a51-46d4-952f-202544b914a1",
                13831,
            ),
        ],
    );
}
