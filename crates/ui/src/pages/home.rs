use crate::api::{delete_drink, fetch_drink_types, fetch_today, log_drink, create_drink_type};
use crate::auth::{load_daily_limit, AuthContext};
use crate::components::drink_button::DrinkButton;
use crate::components::log_entry::LogEntry;
use crate::components::modal::Modal;
use crate::components::progress_bar::ProgressBar;
use crate::routes::Route;
use crate::types::{DrinkType, TodayResponse};
use crate::utils::caffeine_color;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let navigator = use_navigator().unwrap();

    if !auth.is_authenticated() {
        navigator.push(&Route::Login);
        return html! {};
    }

    let today = use_state(|| Option::<TodayResponse>::None);
    let drink_types = use_state(Vec::<DrinkType>::new);
    let error = use_state(|| Option::<String>::None);
    let show_modal = use_state(|| false);

    // Custom drink form state
    let new_name = use_state(String::new);
    let new_mg = use_state(|| 80i32);
    let new_emoji = use_state(|| "☕".to_string());

    let daily_limit = use_state(load_daily_limit);

    let token = auth.token.clone().unwrap_or_default();

    // Initial data load
    {
        let today = today.clone();
        let drink_types = drink_types.clone();
        let token = token.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let today = today.clone();
            let drink_types = drink_types.clone();
            let token = token.clone();
            let error = error.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_today(&token).await {
                    Ok(data) => today.set(Some(data)),
                    Err(e) => error.set(Some(e)),
                }
                match fetch_drink_types(&token).await {
                    Ok(data) => drink_types.set(data),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_log_drink = {
        let today = today.clone();
        let token = token.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let today = today.clone();
            let token = token.clone();
            let error = error.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match log_drink(&token, &id).await {
                    Ok(entry) => {
                        today.set(today.as_ref().map(|t| {
                            let mut updated = t.clone();
                            updated.total_caffeine_mg += entry.drink_type.caffeine_mg;
                            updated.entries.insert(0, entry);
                            updated
                        }));
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    let on_delete = {
        let today = today.clone();
        let token = token.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let today = today.clone();
            let token = token.clone();
            let error = error.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match delete_drink(&token, &id).await {
                    Ok(()) => {
                        today.set(today.as_ref().map(|t| {
                            let mut updated = t.clone();
                            if let Some(removed) = updated.entries.iter().find(|e| e.id == id) {
                                updated.total_caffeine_mg -= removed.drink_type.caffeine_mg;
                            }
                            updated.entries.retain(|e| e.id != id);
                            updated
                        }));
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    let on_open_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(true))
    };

    let on_close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(false))
    };

    let on_new_name = {
        let new_name = new_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_name.set(input.value());
        })
    };

    let on_new_mg = {
        let new_mg = new_mg.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<i32>() {
                new_mg.set(v);
            }
        })
    };

    let on_new_emoji = {
        let new_emoji = new_emoji.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_emoji.set(input.value());
        })
    };

    let on_create_custom = {
        let new_name = new_name.clone();
        let new_mg = new_mg.clone();
        let new_emoji = new_emoji.clone();
        let drink_types = drink_types.clone();
        let token = token.clone();
        let error = error.clone();
        let show_modal = show_modal.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = (*new_name).clone();
            let mg = *new_mg;
            let emoji = (*new_emoji).clone();
            let drink_types = drink_types.clone();
            let token = token.clone();
            let error = error.clone();
            let show_modal = show_modal.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match create_drink_type(&token, &name, mg, &emoji).await {
                    Ok(dt) => {
                        let mut updated = (*drink_types).clone();
                        updated.push(dt);
                        drink_types.set(updated);
                        show_modal.set(false);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    let total = today.as_ref().map(|t| t.total_caffeine_mg).unwrap_or(0);
    let limit = *daily_limit;
    let color_class = format!("total-display total-display--{}", caffeine_color(total, limit));

    html! {
        <div class="page">
            <header class="page-header">
                <h1 class="page-title">{ "Heute" }</h1>
            </header>

            <main class="page-content">
                if let Some(err) = (*error).clone() {
                    <div class="alert alert--error">{ err }</div>
                }

                <div class={color_class}>
                    <span class="total-display__number">{ total }</span>
                    <span class="total-display__unit">{ " mg Koffein" }</span>
                </div>

                <ProgressBar {total} {limit} />

                <p class="limit-label">
                    { format!("{} / {} mg", total, limit) }
                </p>

                <section class="section">
                    <h2 class="section-title">{ "Schnell hinzufügen" }</h2>
                    <div class="drink-grid">
                        { for (*drink_types).iter().map(|dt| {
                            html! {
                                <DrinkButton
                                    key={dt.id.clone()}
                                    drink={dt.clone()}
                                    on_click={on_log_drink.clone()}
                                />
                            }
                        }) }
                        <button class="drink-btn drink-btn--add" onclick={on_open_modal}>
                            <span class="drink-btn__emoji">{ "+" }</span>
                            <span class="drink-btn__name">{ "Eigener" }</span>
                        </button>
                    </div>
                </section>

                <section class="section">
                    <h2 class="section-title">{ "Heutige Einträge" }</h2>
                    {
                        if let Some(t) = (*today).as_ref() {
                            if t.entries.is_empty() {
                                html! {
                                    <div class="empty-state">
                                        <div class="empty-state__icon">{ "☕" }</div>
                                        <p class="empty-state__text">
                                            { "Noch kein Koffein heute. Füge deinen ersten Drink hinzu!" }
                                        </p>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="log-list">
                                        { for t.entries.iter().map(|e| html! {
                                            <LogEntry
                                                key={e.id.clone()}
                                                entry={e.clone()}
                                                on_delete={on_delete.clone()}
                                            />
                                        }) }
                                    </div>
                                }
                            }
                        } else {
                            html! { <div class="loading">{ "Laden…" }</div> }
                        }
                    }
                </section>
            </main>

            if *show_modal {
                <Modal title="Eigenes Getränk" on_close={on_close_modal}>
                    <form onsubmit={on_create_custom} class="modal-form">
                        <div class="form-group">
                            <label class="form-label">{ "Name" }</label>
                            <input
                                type="text"
                                class="form-input"
                                value={(*new_name).clone()}
                                oninput={on_new_name}
                                required=true
                                placeholder="z.B. Matcha Latte"
                            />
                        </div>
                        <div class="form-row">
                            <div class="form-group">
                                <label class="form-label">{ "Emoji" }</label>
                                <input
                                    type="text"
                                    class="form-input form-input--emoji"
                                    value={(*new_emoji).clone()}
                                    oninput={on_new_emoji}
                                    maxlength="2"
                                />
                            </div>
                            <div class="form-group">
                                <label class="form-label">{ "Koffein (mg)" }</label>
                                <input
                                    type="number"
                                    class="form-input"
                                    value={(*new_mg).to_string()}
                                    oninput={on_new_mg}
                                    min="1"
                                    max="2000"
                                    required=true
                                />
                            </div>
                        </div>
                        <button type="submit" class="btn btn--primary btn--full">
                            { "Hinzufügen" }
                        </button>
                    </form>
                </Modal>
            }
        </div>
    }
}
