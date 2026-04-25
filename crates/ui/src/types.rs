use serde::{Deserialize, Serialize};

pub const SUPABASE_URL: &str = "https://xoryyknrowwsodrdjtua.supabase.co";
pub const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Inhvcnl5a25yb3d3c29kcmRqdHVhIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NzY3MTQ4MzYsImV4cCI6MjA5MjI5MDgzNn0.LUT1MScw-Z0F6kfp5WLuR-5HsAKWBmEzcz9Jb8C6zfU";
pub const STORAGE_TOKEN_KEY: &str = "sb_token";
pub const STORAGE_EMAIL_KEY: &str = "sb_email";
pub const STORAGE_LIMIT_KEY: &str = "daily_limit_mg";
pub const DEFAULT_DAILY_LIMIT: i32 = 400;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrinkType {
    pub id: String,
    pub name: String,
    pub caffeine_mg: i32,
    pub emoji: String,
    pub is_preset: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrinkEntry {
    pub id: String,
    pub drink_type: DrinkType,
    pub consumed_at: String,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodayResponse {
    pub entries: Vec<DrinkEntry>,
    pub total_caffeine_mg: i32,
    pub daily_limit_mg: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub date: String,
    pub total_caffeine_mg: i32,
    pub drink_count: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: AuthUser,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: Option<String>,
}
