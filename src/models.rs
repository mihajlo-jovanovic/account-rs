use crate::schema::bank_accounts;
use diesel::{Insertable, Queryable, Selectable};

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = bank_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BankAccount {
    id: i32,
    name: String,
    stakeholder: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = bank_accounts)]
pub struct NewAccount {
    name: String,
    stakeholder: String,
}
