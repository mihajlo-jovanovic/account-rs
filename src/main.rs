use axum::headers::UserAgent;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router, TypedHeader,
};
use diesel::{table, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;

// this embeds the migrations into the application binary
// the migration path is relative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

// normally part of your generated schema.rs file
table! {
    bank_accounts (id) {
        id -> Integer,
        name -> Text,
        stakeholder -> Text,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
struct BankAccount {
    id: i32,
    name: String,
    stakeholder: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = bank_accounts)]
struct NewAccount {
    name: String,
    stakeholder: String,
}

async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    String::from(user_agent.as_str())
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("couldn't connect to database");

    // run the migrations on server startup
    {
        let conn = pool.get().await.expect("couldn't connect to database");
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/account/create", post(create_account))
        .route("/account/list", get(list_accounts))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("--> LISTENING ON {addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_account(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_account): Json<NewAccount>,
) -> Result<Json<BankAccount>, (StatusCode, String)> {
    let con = pool.get().await.map_err(internal_error)?;
    let _res = con
        .interact(|con| {
            diesel::insert_into(bank_accounts::table)
                .values(new_account)
                .returning(BankAccount::as_returning())
                .get_result(con)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(_res))
}

async fn list_accounts(
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
