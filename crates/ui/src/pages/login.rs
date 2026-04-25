use crate::api::supabase_login;
use crate::auth::{save_auth_to_storage, AuthAction, AuthContext};
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let navigator = use_navigator().unwrap();

    let email = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);

    {
        let auth = auth.clone();
        let navigator = navigator.clone();
        use_effect_with(auth.token.clone(), move |token| {
            if token.is_some() {
                navigator.push(&Route::Home);
            }
        });
    }

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

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let auth = auth.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let auth = auth.clone();
            let navigator = navigator.clone();

            loading.set(true);
            error.set(None);

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match supabase_login(&email_val, &password_val).await {
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
                <p class="auth-subtitle">{ "Anmelden" }</p>
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
                            autocomplete="email"
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
                            autocomplete="current-password"
                        />
                    </div>
                    <button type="submit" class="btn btn--primary btn--full" disabled={*loading}>
                        if *loading { { "Anmelden…" } } else { { "Anmelden" } }
                    </button>
                </form>
                <p class="auth-link">
                    { "Noch kein Konto? " }
                    <Link<Route> to={Route::Register}>{ "Registrieren" }</Link<Route>>
                </p>
            </div>
        </div>
    }
}
