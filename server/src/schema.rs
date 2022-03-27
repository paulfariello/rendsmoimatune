table! {
    _user (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
    }
}

table! {
    account (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    debt (id) {
        id -> Uuid,
        debtor_id -> Uuid,
        expenditure_id -> Uuid,
        share -> Int4,
    }
}

table! {
    expenditure (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
        date -> Date,
        amount -> Int4,
        payer_id -> Uuid,
    }
}

table! {
    repayment (id) {
        id -> Uuid,
        account_id -> Uuid,
        date -> Date,
        amount -> Int4,
        payer_id -> Uuid,
        beneficiary_id -> Uuid,
    }
}

joinable!(_user -> account (account_id));
joinable!(debt -> _user (debtor_id));
joinable!(debt -> expenditure (expenditure_id));
joinable!(expenditure -> _user (payer_id));
joinable!(expenditure -> account (account_id));
joinable!(repayment -> account (account_id));

allow_tables_to_appear_in_same_query!(
    _user,
    account,
    debt,
    expenditure,
    repayment,
);
