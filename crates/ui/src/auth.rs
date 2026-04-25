use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub email: Option<String>,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}

pub type AuthContext = UseReducerHandle<AuthState>;

pub enum AuthAction {
    Login { token: String, email: String },
    Logout,
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AuthAction::Login { token, email } => AuthState {
                token: Some(token),
                email: Some(email),
            }
            .into(),
            AuthAction::Logout => AuthState::default().into(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_token_from_storage() -> Option<String> {
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::get::<String>(crate::types::STORAGE_TOKEN_KEY).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_token_from_storage() -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn load_email_from_storage() -> Option<String> {
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::get::<String>(crate::types::STORAGE_EMAIL_KEY).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_email_from_storage() -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn save_auth_to_storage(token: &str, email: &str) {
    use gloo_storage::{LocalStorage, Storage};
    let _ = LocalStorage::set(crate::types::STORAGE_TOKEN_KEY, token);
    let _ = LocalStorage::set(crate::types::STORAGE_EMAIL_KEY, email);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_auth_to_storage(_token: &str, _email: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn clear_auth_from_storage() {
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::delete(crate::types::STORAGE_TOKEN_KEY);
    LocalStorage::delete(crate::types::STORAGE_EMAIL_KEY);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_auth_from_storage() {}

#[cfg(target_arch = "wasm32")]
pub fn load_daily_limit() -> i32 {
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::get::<i32>(crate::types::STORAGE_LIMIT_KEY)
        .unwrap_or(crate::types::DEFAULT_DAILY_LIMIT)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_daily_limit() -> i32 {
    crate::types::DEFAULT_DAILY_LIMIT
}

#[cfg(target_arch = "wasm32")]
pub fn save_daily_limit(limit: i32) {
    use gloo_storage::{LocalStorage, Storage};
    let _ = LocalStorage::set(crate::types::STORAGE_LIMIT_KEY, limit);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_daily_limit(_limit: i32) {}
