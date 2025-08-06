// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    debts (id) {
        id -> Uuid,
        debtor_id -> Uuid,
        expenditure_id -> Uuid,
        share -> Int4,
    }
}

diesel::table! {
    expenditures (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
        date -> Date,
        amount -> Int4,
        payer_id -> Uuid,
    }
}

diesel::table! {
    repayments (id) {
        id -> Uuid,
        account_id -> Uuid,
        date -> Date,
        amount -> Int4,
        payer_id -> Uuid,
        beneficiary_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
    }
}

diesel::joinable!(debts -> expenditures (expenditure_id));
diesel::joinable!(debts -> users (debtor_id));
diesel::joinable!(expenditures -> accounts (account_id));
diesel::joinable!(expenditures -> users (payer_id));
diesel::joinable!(repayments -> accounts (account_id));
diesel::joinable!(users -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    debts,
    expenditures,
    repayments,
    users,
);
