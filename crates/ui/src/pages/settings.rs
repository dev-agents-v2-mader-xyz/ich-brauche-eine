use crate::api::{delete_drink_type, fetch_drink_types};
use crate::auth::{clear_auth_from_storage, load_daily_limit, save_daily_limit, AuthAction, AuthContext};
use crate::routes::Route;
use crate::types::DrinkType;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let navigator = use_navigator().unwrap();

    if !auth.is_authenticated() {
        navigator.push(&Route::Login);
        return html! {};
    }

    let custom_types = use_state(Vec::<DrinkType>::new);
    let error = use_state(|| Option::<String>::None);
    let limit = use_state(load_daily_limit);

    let token = auth.token.clone().unwrap_or_default();

    {
        let custom_types = custom_types.clone();
        let token = token.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let custom_types = custom_types.clone();
            let token = token.clone();
            let error = error.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_drink_types(&token).await {
                    Ok(data) => {
                        let custom: Vec<DrinkType> =
                            data.into_iter().filter(|d| !d.is_preset).collect();
                        custom_types.set(custom);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_delete_type = {
        let custom_types = custom_types.clone();
        let token = token.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let custom_types = custom_types.clone();
            let token = token.clone();
            let error = error.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match delete_drink_type(&token, &id).await {
                    Ok(()) => {
                        let updated: Vec<DrinkType> =
                            (*custom_types).iter().filter(|d| d.id != id).cloned().collect();
                        custom_types.set(updated);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    let on_limit_change = {
        let limit = limit.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<i32>() {
                let clamped = v.clamp(200, 800);
                save_daily_limit(clamped);
                limit.set(clamped);
            }
        })
    };

    let on_logout = {
        let auth = auth.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            clear_auth_from_storage();
            auth.dispatch(AuthAction::Logout);
            navigator.push(&Route::Login);
        })
    };

    html! {
        <div class="page">
            <header class="page-header">
                <h1 class="page-title">{ "Einstellungen" }</h1>
            </header>
            <main class="page-content">
                if let Some(err) = (*error).clone() {
                    <div class="alert alert--error">{ err }</div>
                }

                <section class="section">
                    <h2 class="section-title">{ "Tageslimit" }</h2>
                    <div class="form-group">
                        <label class="form-label">
                            { format!("Tägliches Koffeinlimit: {} mg", *limit) }
                        </label>
                        <input
                            type="range"
                            class="range-input"
                            min="200"
                            max="800"
                            step="50"
                            value={(*limit).to_string()}
                            oninput={on_limit_change}
                        />
                        <div class="range-labels">
                            <span>{ "200 mg" }</span>
                            <span>{ "800 mg" }</span>
                        </div>
                    </div>
                </section>

                <section class="section">
                    <h2 class="section-title">{ "Eigene Getränke" }</h2>
                    if (*custom_types).is_empty() {
                        <p class="text-muted">{ "Keine eigenen Getränke vorhanden." }</p>
                    } else {
                        <div class="settings-list">
                            { for (*custom_types).iter().map(|dt| {
                                let id = dt.id.clone();
                                let on_delete = on_delete_type.clone();
                                html! {
                                    <div key={id.clone()} class="settings-item">
                                        <span class="settings-item__emoji">{ &dt.emoji }</span>
                                        <span class="settings-item__name">{ &dt.name }</span>
                                        <span class="settings-item__mg">
                                            { format!("{} mg", dt.caffeine_mg) }
                                        </span>
                                        <button
                                            class="btn btn--danger btn--sm"
                                            onclick={Callback::from(move |_| on_delete.emit(id.clone()))}
                                        >
                                            { "Löschen" }
                                        </button>
                                    </div>
                                }
                            }) }
                        </div>
                    }
                </section>

                <section class="section">
                    <h2 class="section-title">{ "Konto" }</h2>
                    <div class="account-info">
                        <p class="text-muted">
                            { "Angemeldet als: " }
                            <strong>{ auth.email.clone().unwrap_or_default() }</strong>
                        </p>
                    </div>
                    <button class="btn btn--danger btn--full" onclick={on_logout}>
                        { "Abmelden" }
                    </button>
                </section>
            </main>
        </div>
    }
}
