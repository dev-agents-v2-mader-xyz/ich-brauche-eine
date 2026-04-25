use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::config::Config;

/// Claims embedded in a Supabase JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: Option<String>,
    pub iss: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
    pub exp: usize,
}

/// Request guard that validates a Supabase `Authorization: Bearer <jwt>` header.
pub struct AuthUser {
    pub user_id: String,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let config = match req.rocket().state::<Config>() {
            Some(c) => c,
            None => return Outcome::Error((Status::InternalServerError, ())),
        };

        let token = match req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "))
        {
            Some(t) => t,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let key = DecodingKey::from_secret(config.supabase_jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["authenticated"]);
        validation.set_issuer(&[format!("{}/auth/v1", config.supabase_url)]);

        match decode::<Claims>(token, &key, &validation) {
            Ok(data) => Outcome::Success(AuthUser {
                user_id: data.claims.sub,
                email: data.claims.email,
                role: data.claims.role,
            }),
            Err(e) => {
                tracing::debug!("JWT validation failed: {}", e);
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{encode, EncodingKey, Header as JwtHeader};
    use rocket::http::Header;
    use rocket::local::blocking::Client;
    use rocket::routes;

    use super::*;

    #[rocket::get("/protected")]
    fn protected(user: AuthUser) -> String {
        format!("hello {}", user.user_id)
    }

    fn make_jwt(secret: &str, sub: &str, valid: bool) -> String {
        let exp = if valid {
            chrono::Utc::now().timestamp() as usize + 3600
        } else {
            1 // expired
        };
        let claims = Claims {
            sub: sub.to_string(),
            aud: Some("authenticated".into()),
            iss: Some("https://test.supabase.co/auth/v1".into()),
            role: Some("authenticated".into()),
            email: Some("test@example.com".into()),
            exp,
        };
        encode(
            &JwtHeader::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    fn client(secret: &str) -> Client {
        let config = Config {
            database_url: "unused".into(),
            supabase_url: "https://test.supabase.co".into(),
            supabase_anon_key: "unused".into(),
            supabase_jwt_secret: secret.into(),
            supabase_schema: "public".into(),
            rust_log: "off".into(),
            port: 8000,
        };
        let rocket = rocket::build()
            .manage(config)
            .mount("/", routes![protected]);
        Client::tracked(rocket).expect("valid rocket")
    }

    #[test]
    fn valid_jwt_passes() {
        let c = client("test-secret");
        let token = make_jwt("test-secret", "user-123", true);
        let resp = c
            .get("/protected")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Ok);
        assert!(resp.into_string().unwrap().contains("user-123"));
    }

    #[test]
    fn expired_jwt_is_rejected() {
        let c = client("test-secret");
        let token = make_jwt("test-secret", "user-123", false);
        let resp = c
            .get("/protected")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    fn wrong_secret_is_rejected() {
        let c = client("correct-secret");
        let token = make_jwt("wrong-secret", "user-123", true);
        let resp = c
            .get("/protected")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    fn missing_header_is_rejected() {
        let c = client("test-secret");
        let resp = c.get("/protected").dispatch();
        assert_eq!(resp.status(), rocket::http::Status::Unauthorized);
    }
}
