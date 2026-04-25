// Struct fields declared for completeness/future use in this binary crate.
#![allow(dead_code)]

use rocket::fs::FileServer;
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod db;
mod error;
mod middleware;
mod routes;

use middleware::SecurityHeaders;
use routes::api::drink_types::{create_drink_type, delete_drink_type, list_drink_types};
use routes::api::drinks::{create_drink, delete_drink, get_history, get_today};
use routes::health::health;

#[rocket::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::Config::from_env().expect("configuration error");
    let pool = db::connect(&cfg.database_url, &cfg.supabase_schema)
        .await
        .expect("database connection failed");

    let _rocket = rocket::build()
        .attach(SecurityHeaders)
        .manage(pool)
        .manage(cfg)
        .mount("/", rocket::routes![health])
        .mount(
            "/api",
            rocket::routes![
                list_drink_types,
                create_drink_type,
                delete_drink_type,
                get_today,
                get_history,
                create_drink,
                delete_drink,
            ],
        )
        // Serve the compiled Yew WASM frontend from dist/
        .mount("/", FileServer::from("dist").rank(10))
        .launch()
        .await?;

    Ok(())
}
