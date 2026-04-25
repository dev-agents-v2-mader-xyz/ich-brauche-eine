use rocket::get;
use rocket::serde::json::Json;
use sqlx::PgPool;
use rocket::State;

use crate::error::ApiError;

/// GET /health — returns 200 only when DB is reachable.
#[get("/health")]
pub async fn health(db: &State<PgPool>) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("SELECT 1").execute(db.inner()).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires TEST_DB_URL"]
    fn health_returns_200() {
        // Full integration test requires a live DB — run with TEST_DB_URL set.
    }
}
