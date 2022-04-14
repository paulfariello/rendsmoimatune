table! {
    accounts (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    debts (id) {
        id -> Uuid,
        debtor_id -> Uuid,
        expenditure_id -> Uuid,
        share -> Int4,
    }
}

table! {
    expenditures (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
        date -> Date,
        amount -> Int4,
        payer_id -> Uuid,
    }
}

table! {
    repayments (id) {
        id -> Uuid,
        account_id -> Uuid,
        date -> Date,
        amount -> Int4,
        payer_id -> Uuid,
        beneficiary_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
    }
}

joinable!(debts -> expenditures (expenditure_id));
joinable!(debts -> users (debtor_id));
joinable!(expenditures -> accounts (account_id));
joinable!(expenditures -> users (payer_id));
joinable!(repayments -> accounts (account_id));
joinable!(users -> accounts (account_id));

allow_tables_to_appear_in_same_query!(accounts, debts, expenditures, repayments, users,);
