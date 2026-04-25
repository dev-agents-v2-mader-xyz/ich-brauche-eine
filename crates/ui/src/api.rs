use crate::types::*;

#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;

fn auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}

#[cfg(target_arch = "wasm32")]
pub async fn supabase_login(email: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/v1/token?grant_type=password", SUPABASE_URL);
    let body = serde_json::json!({"email": email, "password": password});
    let resp = Request::post(&url)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<AuthResponse>().await.map_err(|e| e.to_string())
    } else {
        let msg: serde_json::Value = resp.json().await.unwrap_or_default();
        Err(msg["error_description"]
            .as_str()
            .or_else(|| msg["msg"].as_str())
            .unwrap_or("Login fehlgeschlagen")
            .to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn supabase_login(_email: &str, _password: &str) -> Result<AuthResponse, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn supabase_register(email: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/v1/signup", SUPABASE_URL);
    let body = serde_json::json!({"email": email, "password": password});
    let resp = Request::post(&url)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<AuthResponse>().await.map_err(|e| e.to_string())
    } else {
        let msg: serde_json::Value = resp.json().await.unwrap_or_default();
        Err(msg["error_description"]
            .as_str()
            .or_else(|| msg["msg"].as_str())
            .unwrap_or("Registrierung fehlgeschlagen")
            .to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn supabase_register(_email: &str, _password: &str) -> Result<AuthResponse, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_drink_types(token: &str) -> Result<Vec<DrinkType>, String> {
    let resp = Request::get("/api/drink-types")
        .header("Authorization", &auth_header(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<Vec<DrinkType>>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Status {}", resp.status()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_drink_types(_token: &str) -> Result<Vec<DrinkType>, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_today(token: &str) -> Result<TodayResponse, String> {
    let resp = Request::get("/api/drinks/today")
        .header("Authorization", &auth_header(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<TodayResponse>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Status {}", resp.status()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_today(_token: &str) -> Result<TodayResponse, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_history(token: &str, days: u32) -> Result<Vec<HistoryEntry>, String> {
    let url = format!("/api/drinks/history?days={}", days);
    let resp = Request::get(&url)
        .header("Authorization", &auth_header(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<Vec<HistoryEntry>>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Status {}", resp.status()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_history(_token: &str, _days: u32) -> Result<Vec<HistoryEntry>, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn log_drink(token: &str, drink_type_id: &str) -> Result<DrinkEntry, String> {
    let body = serde_json::json!({"drink_type_id": drink_type_id});
    let resp = Request::post("/api/drinks")
        .header("Authorization", &auth_header(token))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<DrinkEntry>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Status {}", resp.status()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn log_drink(_token: &str, _drink_type_id: &str) -> Result<DrinkEntry, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn delete_drink(token: &str, id: &str) -> Result<(), String> {
    let url = format!("/api/drinks/{}", id);
    let resp = Request::delete(&url)
        .header("Authorization", &auth_header(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Status {}", resp.status()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn delete_drink(_token: &str, _id: &str) -> Result<(), String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn create_drink_type(
    token: &str,
    name: &str,
    caffeine_mg: i32,
    emoji: &str,
) -> Result<DrinkType, String> {
    let body = serde_json::json!({"name": name, "caffeine_mg": caffeine_mg, "emoji": emoji});
    let resp = Request::post("/api/drink-types")
        .header("Authorization", &auth_header(token))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<DrinkType>().await.map_err(|e| e.to_string())
    } else {
        let msg: serde_json::Value = resp.json().await.unwrap_or_default();
        Err(msg["error"].as_str().unwrap_or("Fehler beim Erstellen").to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_drink_type(
    _token: &str,
    _name: &str,
    _caffeine_mg: i32,
    _emoji: &str,
) -> Result<DrinkType, String> {
    Err("not available on native".into())
}

#[cfg(target_arch = "wasm32")]
pub async fn delete_drink_type(token: &str, id: &str) -> Result<(), String> {
    let url = format!("/api/drink-types/{}", id);
    let resp = Request::delete(&url)
        .header("Authorization", &auth_header(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Status {}", resp.status()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn delete_drink_type(_token: &str, _id: &str) -> Result<(), String> {
    Err("not available on native".into())
}
