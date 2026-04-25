use rocket::http::Status;
use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DrinkType {
    pub id: Uuid,
    pub name: String,
    pub caffeine_mg: i32,
    pub emoji: String,
    pub is_preset: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateDrinkTypeBody {
    pub name: String,
    pub caffeine_mg: i32,
    pub emoji: String,
}

#[derive(sqlx::FromRow)]
struct OwnerRow {
    is_preset: bool,
    user_id: Option<Uuid>,
}

#[get("/drink-types")]
pub async fn list_drink_types(
    db: &State<PgPool>,
    user: AuthUser,
) -> Result<Json<Vec<DrinkType>>, ApiError> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;

    let rows = sqlx::query_as::<_, DrinkType>(
        "SELECT id, name, caffeine_mg, emoji, is_preset
         FROM drink_types
         WHERE is_preset = true OR user_id = $1
         ORDER BY is_preset DESC, name ASC",
    )
    .bind(user_id)
    .fetch_all(db.inner())
    .await?;

    Ok(Json(rows))
}

#[post("/drink-types", data = "<body>")]
pub async fn create_drink_type(
    db: &State<PgPool>,
    user: AuthUser,
    body: Json<CreateDrinkTypeBody>,
) -> Result<(Status, Json<DrinkType>), ApiError> {
    if body.caffeine_mg <= 0 {
        return Err(ApiError::BadRequest(
            "caffeine_mg must be greater than 0".into(),
        ));
    }

    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;

    let existing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM drink_types WHERE name = $1 AND user_id = $2",
    )
    .bind(&body.name)
    .bind(user_id)
    .fetch_one(db.inner())
    .await?;

    if existing > 0 {
        return Err(ApiError::Conflict(
            "a custom drink type with this name already exists".into(),
        ));
    }

    let row = sqlx::query_as::<_, DrinkType>(
        "INSERT INTO drink_types (id, name, caffeine_mg, emoji, is_preset, user_id)
         VALUES (gen_random_uuid(), $1, $2, $3, false, $4)
         RETURNING id, name, caffeine_mg, emoji, is_preset",
    )
    .bind(&body.name)
    .bind(body.caffeine_mg)
    .bind(&body.emoji)
    .bind(user_id)
    .fetch_one(db.inner())
    .await?;

    Ok((Status::Created, Json(row)))
}

#[delete("/drink-types/<id>")]
pub async fn delete_drink_type(
    db: &State<PgPool>,
    user: AuthUser,
    id: String,
) -> Result<Status, ApiError> {
    let drink_id = Uuid::parse_str(&id).map_err(|_| ApiError::NotFound)?;
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;

    let row: Option<OwnerRow> =
        sqlx::query_as("SELECT is_preset, user_id FROM drink_types WHERE id = $1")
            .bind(drink_id)
            .fetch_optional(db.inner())
            .await?;

    let row = row.ok_or(ApiError::NotFound)?;

    if row.is_preset || row.user_id != Some(user_id) {
        return Err(ApiError::Forbidden);
    }

    sqlx::query("DELETE FROM drink_types WHERE id = $1")
        .bind(drink_id)
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
                rocket::routes![list_drink_types, create_drink_type, delete_drink_type],
            );
        Client::tracked(rocket).expect("valid rocket")
    }

    #[test]
    fn list_drink_types_rejects_missing_auth() {
        let client = test_client();
        let resp = client.get("/api/drink-types").dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    fn create_drink_type_rejects_missing_auth() {
        let client = test_client();
        let resp = client
            .post("/api/drink-types")
            .header(rocket::http::ContentType::JSON)
            .body(r#"{"name":"Test","caffeine_mg":80,"emoji":"☕"}"#)
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    fn delete_drink_type_rejects_missing_auth() {
        let client = test_client();
        let resp = client
            .delete("/api/drink-types/00000000-0000-0000-0000-000000000001")
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    #[ignore = "requires TEST_DB_URL"]
    fn list_drink_types_returns_presets() {
        let token = make_jwt("test-secret");
        let client = test_client();
        let resp = client
            .get("/api/drink-types")
            .header(Header::new("Authorization", format!("Bearer {token}")))
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Ok);
    }

    #[test]
    #[ignore = "requires TEST_DB_URL"]
    fn create_drink_type_rejects_zero_caffeine() {
        let token = make_jwt("test-secret");
        let client = test_client();
        let resp = client
            .post("/api/drink-types")
            .header(Header::new("Authorization", format!("Bearer {token}")))
            .header(rocket::http::ContentType::JSON)
            .body(r#"{"name":"Test","caffeine_mg":0,"emoji":"☕"}"#)
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::BadRequest);
    }
}
