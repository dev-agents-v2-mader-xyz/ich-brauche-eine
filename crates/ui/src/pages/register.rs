use crate::api::supabase_register;
use crate::auth::{save_auth_to_storage, AuthAction, AuthContext};
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let navigator = use_navigator().unwrap();

    let email = use_state(String::new);
    let password = use_state(String::new);
    let confirm = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);

    let on_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };
    let on_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };
    let on_confirm = {
        let confirm = confirm.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            confirm.set(input.value());
        })
    };

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let confirm = confirm.clone();
        let error = error.clone();
        let loading = loading.clone();
        let auth = auth.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let confirm_val = (*confirm).clone();

            if password_val != confirm_val {
                error.set(Some("Passwörter stimmen nicht überein.".into()));
                return;
            }
            if password_val.len() < 6 {
                error.set(Some("Passwort muss mindestens 6 Zeichen lang sein.".into()));
                return;
            }

            let error = error.clone();
            let loading = loading.clone();
            let auth = auth.clone();
            let navigator = navigator.clone();

            loading.set(true);
            error.set(None);

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match supabase_register(&email_val, &password_val).await {
                    Ok(resp) => {
                        let user_email = resp
                            .user
                            .email
                            .clone()
                            .unwrap_or_else(|| email_val.clone());
                        save_auth_to_storage(&resp.access_token, &user_email);
                        auth.dispatch(AuthAction::Login {
                            token: resp.access_token,
                            email: user_email,
                        });
                        navigator.push(&Route::Home);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });
            #[cfg(not(target_arch = "wasm32"))]
            {
                loading.set(false);
                error.set(Some("Not supported".into()));
            }
        })
    };

    html! {
        <div class="auth-page">
            <div class="auth-card">
                <div class="auth-logo">{ "☕" }</div>
                <h1 class="auth-title">{ "Koffein-Tracker" }</h1>
                <p class="auth-subtitle">{ "Konto erstellen" }</p>
                if let Some(err) = (*error).clone() {
                    <div class="alert alert--error">{ err }</div>
                }
                <form {onsubmit} class="auth-form">
                    <div class="form-group">
                        <label class="form-label">{ "E-Mail" }</label>
                        <input
                            type="email"
                            class="form-input"
                            value={(*email).clone()}
                            oninput={on_email}
                            required=true
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label">{ "Passwort" }</label>
                        <input
                            type="password"
                            class="form-input"
                            value={(*password).clone()}
                            oninput={on_password}
                            required=true
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label">{ "Passwort bestätigen" }</label>
                        <input
                            type="password"
                            class="form-input"
                            value={(*confirm).clone()}
                            oninput={on_confirm}
                            required=true
                        />
                    </div>
                    <button type="submit" class="btn btn--primary btn--full" disabled={*loading}>
                        if *loading { { "Registrieren…" } } else { { "Registrieren" } }
                    </button>
                </form>
                <p class="auth-link">
                    { "Bereits ein Konto? " }
                    <Link<Route> to={Route::Login}>{ "Anmelden" }</Link<Route>>
                </p>
            </div>
        </div>
    }
}
