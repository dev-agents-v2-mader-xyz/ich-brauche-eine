use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found">
            <div class="not-found__icon">{ "☕" }</div>
            <h1 class="not-found__title">{ "404" }</h1>
            <p class="not-found__text">{ "Diese Seite existiert nicht." }</p>
            <Link<Route> to={Route::Home} classes="btn btn--primary">
                { "Zurück zur Startseite" }
            </Link<Route>>
        </div>
    }
}
