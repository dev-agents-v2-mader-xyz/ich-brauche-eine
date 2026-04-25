use crate::api::fetch_history;
use crate::auth::AuthContext;
use crate::routes::Route;
use crate::types::HistoryEntry;
use crate::utils::{format_date, progress_fraction};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let navigator = use_navigator().unwrap();

    if !auth.is_authenticated() {
        navigator.push(&Route::Login);
        return html! {};
    }

    let entries = use_state(Vec::<HistoryEntry>::new);
    let error = use_state(|| Option::<String>::None);
    let days = use_state(|| 30u32);
    let expanded = use_state(|| Option::<String>::None);

    let token = auth.token.clone().unwrap_or_default();

    {
        let entries = entries.clone();
        let token = token.clone();
        let error = error.clone();
        let days_val = *days;
        use_effect_with(days_val, move |&d| {
            let entries = entries.clone();
            let token = token.clone();
            let error = error.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_history(&token, d).await {
                    Ok(data) => entries.set(data),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_load_more = {
        let days = days.clone();
        Callback::from(move |_| days.set(90))
    };

    html! {
        <div class="page">
            <header class="page-header">
                <h1 class="page-title">{ "Verlauf" }</h1>
            </header>
            <main class="page-content">
                if let Some(err) = (*error).clone() {
                    <div class="alert alert--error">{ err }</div>
                }
                if (*entries).is_empty() {
                    <div class="empty-state">
                        <div class="empty-state__icon">{ "📅" }</div>
                        <p class="empty-state__text">{ "Noch keine Einträge." }</p>
                    </div>
                } else {
                    <div class="history-list">
                        { for (*entries).iter().map(|e| {
                            let date_str = e.date.clone();
                            let expanded = expanded.clone();
                            let is_expanded = (*expanded).as_deref() == Some(&e.date);
                            let toggle = {
                                let expanded = expanded.clone();
                                let d = e.date.clone();
                                Callback::from(move |_| {
                                    if (*expanded).as_deref() == Some(&d) {
                                        expanded.set(None);
                                    } else {
                                        expanded.set(Some(d.clone()));
                                    }
                                })
                            };
                            let pct = (progress_fraction(e.total_caffeine_mg, 400) * 100.0) as u32;
                            html! {
                                <div key={date_str.clone()} class="history-day">
                                    <button class="history-day__header" onclick={toggle}>
                                        <span class="history-day__date">
                                            { format_date(&e.date) }
                                        </span>
                                        <div class="history-day__bar-wrap">
                                            <div
                                                class="history-day__bar"
                                                style={format!("width: {}%", pct)}
                                            />
                                        </div>
                                        <span class="history-day__total">
                                            { format!("{} mg", e.total_caffeine_mg) }
                                        </span>
                                        <span class="history-day__count">
                                            { format!("{} Drinks", e.drink_count) }
                                        </span>
                                        <span class="history-day__chevron">
                                            if is_expanded { { "▲" } } else { { "▼" } }
                                        </span>
                                    </button>
                                </div>
                            }
                        }) }
                    </div>
                    if *days < 90 {
                        <button class="btn btn--secondary btn--full" onclick={on_load_more}>
                            { "Mehr laden (90 Tage)" }
                        </button>
                    }
                }
            </main>
        </div>
    }
}
