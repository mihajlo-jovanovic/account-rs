pub mod models;
pub mod schema;

use self::models::{BankAccount, NewAccount};
use crate::schema::bank_accounts;
use axum::{extract::State, http::StatusCode, response::Json};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
pub async fn create_account(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_account): Json<NewAccount>,
) -> Result<Json<BankAccount>, (StatusCode, String)> {
    let con = pool.get().await.map_err(internal_error)?;
    let res = con
        .interact(|con| {
            diesel::insert_into(bank_accounts::table)
                .values(new_account)
                .returning(BankAccount::as_returning())
                .get_result(con)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_accounts(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<BankAccount>>, (StatusCode, String)> {
    let con = pool.get().await.map_err(internal_error)?;
    let res = con
        .interact(|con| {
            bank_accounts::table
                .select(BankAccount::as_select())
                .load(con)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
