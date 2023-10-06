// @generated automatically by Diesel CLI.

diesel::table! {
    bank_accounts (id) {
        id -> Int4,
        name -> Text,
        stakeholder -> Text,
    }
}
