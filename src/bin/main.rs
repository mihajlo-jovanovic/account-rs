use std::net::SocketAddr;

use axum::headers::UserAgent;
use axum::{
    routing::{get, post},
    Router, TypedHeader,
};
use diesel::table;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use account_rs::*;

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
