use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_jwt_secret: String,
    /// Postgres schema to use (e.g. "app_my-project" in dev, "public" in prod)
    pub supabase_schema: String,
    pub rust_log: String,
    pub port: u16,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    Missing(String),
    #[error("invalid value for {key}: {reason}")]
    Invalid { key: String, reason: String },
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: required("SUPABASE_DB_URL")?,
            supabase_url: required("SUPABASE_URL")?,
            supabase_anon_key: required("SUPABASE_ANON_KEY")?,
            supabase_jwt_secret: required("SUPABASE_JWT_SECRET")?,
            supabase_schema: env::var("SUPABASE_SCHEMA").unwrap_or_else(|_| "public".into()),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".into())
                .parse::<u16>()
                .map_err(|_| ConfigError::Invalid {
                    key: "PORT".into(),
                    reason: "must be a valid port number".into(),
                })?,
        })
    }
}

fn required(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::Missing(key.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_var_returns_error() {
        env::remove_var("__TMPL_TEST_MISSING");
        assert!(matches!(
            required("__TMPL_TEST_MISSING"),
            Err(ConfigError::Missing(_))
        ));
    }

    #[test]
    fn default_schema_is_public() {
        env::remove_var("SUPABASE_SCHEMA");
        let schema = env::var("SUPABASE_SCHEMA").unwrap_or_else(|_| "public".into());
        assert_eq!(schema, "public");
    }
}
