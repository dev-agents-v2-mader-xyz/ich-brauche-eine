use rocket::http::Status;
use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};

use crate::auth::AuthUser;
use crate::error::ApiError;

#[derive(Debug, Serialize)]
pub struct DrinkTypeInfo {
    pub name: String,
    pub emoji: String,
    pub caffeine_mg: i32,
}

#[derive(Debug, Serialize)]
pub struct DrinkLogEntry {
    pub id: Uuid,
    pub drink_type: DrinkTypeInfo,
    pub consumed_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TodayResponse {
    pub entries: Vec<DrinkLogEntry>,
    pub total_caffeine_mg: i32,
    pub daily_limit_mg: i32,
}

#[derive(Debug, Serialize)]
pub struct DailySummary {
    pub date: String,
    pub total_caffeine_mg: i64,
    pub drink_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDrinkBody {
    pub drink_type_id: Uuid,
    pub consumed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(sqlx::FromRow)]
struct DrinkLogRow {
    id: Uuid,
    consumed_at: DateTime<Utc>,
    notes: Option<String>,
    dt_name: String,
    dt_emoji: String,
    dt_caffeine_mg: i32,
}

#[derive(sqlx::FromRow)]
struct SummaryRow {
    date: NaiveDate,
    total_caffeine_mg: i64,
    drink_count: i64,
}

#[derive(sqlx::FromRow)]
struct LogOwnerRow {
    user_id: Uuid,
}

fn row_to_entry(r: DrinkLogRow) -> DrinkLogEntry {
    DrinkLogEntry {
        id: r.id,
        drink_type: DrinkTypeInfo {
            name: r.dt_name,
            emoji: r.dt_emoji,
            caffeine_mg: r.dt_caffeine_mg,
        },
        consumed_at: r.consumed_at,
        notes: r.notes,
    }
}

#[get("/drinks/today")]
pub async fn get_today(
    db: &State<PgPool>,
    user: AuthUser,
) -> Result<Json<TodayResponse>, ApiError> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;

    let rows = sqlx::query_as::<_, DrinkLogRow>(
        "SELECT dl.id, dl.consumed_at, dl.notes,
                dt.name AS dt_name, dt.emoji AS dt_emoji, dt.caffeine_mg AS dt_caffeine_mg
         FROM drink_logs dl
         JOIN drink_types dt ON dt.id = dl.drink_type_id
         WHERE dl.user_id = $1
           AND dl.consumed_at::date = CURRENT_DATE
         ORDER BY dl.consumed_at ASC",
    )
    .bind(user_id)
    .fetch_all(db.inner())
    .await?;

    let total: i32 = rows.iter().map(|r| r.dt_caffeine_mg).sum();
    let entries = rows.into_iter().map(row_to_entry).collect();

    Ok(Json(TodayResponse {
        entries,
        total_caffeine_mg: total,
        daily_limit_mg: 400,
    }))
}

#[get("/drinks/history?<days>")]
pub async fn get_history(
    db: &State<PgPool>,
    user: AuthUser,
    days: Option<i64>,
) -> Result<Json<Vec<DailySummary>>, ApiError> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;
    let days = days.unwrap_or(30).clamp(1, 90);

    let rows = sqlx::query_as::<_, SummaryRow>(
        "SELECT dl.consumed_at::date AS date,
                SUM(dt.caffeine_mg) AS total_caffeine_mg,
                COUNT(*) AS drink_count
         FROM drink_logs dl
         JOIN drink_types dt ON dt.id = dl.drink_type_id
         WHERE dl.user_id = $1
           AND dl.consumed_at >= NOW() - (INTERVAL '1 day' * $2)
         GROUP BY dl.consumed_at::date
         ORDER BY dl.consumed_at::date DESC",
    )
    .bind(user_id)
    .bind(days)
    .fetch_all(db.inner())
    .await?;

    let summaries = rows
        .into_iter()
        .map(|r| DailySummary {
            date: r.date.format("%Y-%m-%d").to_string(),
            total_caffeine_mg: r.total_caffeine_mg,
            drink_count: r.drink_count,
        })
        .collect();

    Ok(Json(summaries))
}

#[post("/drinks", data = "<body>")]
pub async fn create_drink(
    db: &State<PgPool>,
    user: AuthUser,
    body: Json<CreateDrinkBody>,
) -> Result<(Status, Json<DrinkLogEntry>), ApiError> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;

    let accessible: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM drink_types
         WHERE id = $1 AND (is_preset = true OR user_id = $2)",
    )
    .bind(body.drink_type_id)
    .bind(user_id)
    .fetch_one(db.inner())
    .await?;

    if accessible == 0 {
        return Err(ApiError::NotFound);
    }

    let consumed_at = body.consumed_at.unwrap_or_else(Utc::now);

    let row = sqlx::query_as::<_, DrinkLogRow>(
        "WITH inserted AS (
             INSERT INTO drink_logs (id, user_id, drink_type_id, consumed_at, notes)
             VALUES (gen_random_uuid(), $1, $2, $3, $4)
             RETURNING id, consumed_at, notes, drink_type_id
         )
         SELECT i.id, i.consumed_at, i.notes,
                dt.name AS dt_name, dt.emoji AS dt_emoji, dt.caffeine_mg AS dt_caffeine_mg
         FROM inserted i
         JOIN drink_types dt ON dt.id = i.drink_type_id",
    )
    .bind(user_id)
    .bind(body.drink_type_id)
    .bind(consumed_at)
    .bind(&body.notes)
    .fetch_one(db.inner())
    .await?;

    Ok((Status::Created, Json(row_to_entry(row))))
}

#[delete("/drinks/<id>")]
pub async fn delete_drink(
    db: &State<PgPool>,
    user: AuthUser,
    id: String,
) -> Result<Status, ApiError> {
    let log_id = Uuid::parse_str(&id).map_err(|_| ApiError::NotFound)?;
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;

    let row: Option<LogOwnerRow> =
        sqlx::query_as("SELECT user_id FROM drink_logs WHERE id = $1")
            .bind(log_id)
            .fetch_optional(db.inner())
            .await?;

    let row = row.ok_or(ApiError::NotFound)?;

    if row.user_id != user_id {
        return Err(ApiError::Forbidden);
    }

    sqlx::query("DELETE FROM drink_logs WHERE id = $1")
        .bind(log_id)
        .execute(db.inner())
        .await?;

    Ok(Status::NoContent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use jsonwebtoken::{encode, EncodingKey, Header as JwtHeader};
    use rocket::http::Header;
    use rocket::local::blocking::Client;

    fn make_jwt(secret: &str) -> String {
        let claims = crate::auth::Claims {
            sub: "00000000-0000-0000-0000-000000000001".into(),
            aud: Some("authenticated".into()),
            iss: Some("https://test.supabase.co/auth/v1".into()),
            role: Some("authenticated".into()),
            email: Some("test@example.com".into()),
            exp: chrono::Utc::now().timestamp() as usize + 3600,
        };
        encode(
            &JwtHeader::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    fn test_client() -> Client {
        let config = Config {
            database_url: "unused".into(),
            supabase_url: "https://test.supabase.co".into(),
            supabase_anon_key: "unused".into(),
            supabase_jwt_secret: "test-secret".into(),
            supabase_schema: "public".into(),
            rust_log: "off".into(),
            port: 8000,
        };
        let rocket = rocket::build()
            .manage(config)
            .mount(
                "/api",
                rocket::routes![get_today, get_history, create_drink, delete_drink],
            );
        Client::untracked(rocket).expect("valid rocket")
    }

    #[test]
    #[ignore = "requires DB pool in test setup"]
    fn get_today_rejects_missing_auth() {
        let client = test_client();
        let resp = client.get("/api/drinks/today").dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    #[ignore = "requires DB pool in test setup"]
    fn get_history_rejects_missing_auth() {
        let client = test_client();
        let resp = client.get("/api/drinks/history").dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    #[ignore = "requires DB pool in test setup"]
    fn create_drink_rejects_missing_auth() {
        let client = test_client();
        let resp = client
            .post("/api/drinks")
            .header(rocket::http::ContentType::JSON)
            .body(r#"{"drink_type_id":"00000000-0000-0000-0000-000000000001"}"#)
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    #[ignore = "requires DB pool in test setup"]
    fn delete_drink_rejects_missing_auth() {
        let client = test_client();
        let resp = client
            .delete("/api/drinks/00000000-0000-0000-0000-000000000001")
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    #[ignore = "requires TEST_DB_URL"]
    fn get_today_returns_empty_for_new_user() {
        let token = make_jwt("test-secret");
        let client = test_client();
        let resp = client
            .get("/api/drinks/today")
            .header(Header::new("Authorization", format!("Bearer {token}")))
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Ok);
        let body: serde_json::Value = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        assert_eq!(body["daily_limit_mg"], 400);
        assert_eq!(body["total_caffeine_mg"], 0);
    }

    #[test]
    #[ignore = "requires TEST_DB_URL"]
    fn create_drink_returns_404_for_unknown_type() {
        let token = make_jwt("test-secret");
        let client = test_client();
        let resp = client
            .post("/api/drinks")
            .header(Header::new("Authorization", format!("Bearer {token}")))
            .header(rocket::http::ContentType::JSON)
            .body(r#"{"drink_type_id":"00000000-0000-0000-0000-000000000099"}"#)
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::NotFound);
    }
}
