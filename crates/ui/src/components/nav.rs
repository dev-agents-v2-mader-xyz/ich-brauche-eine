use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let route = use_route::<Route>();
    let is = |r: Route| if route.as_ref() == Some(&r) { "nav-item nav-item--active" } else { "nav-item" };

    html! {
        <nav class="bottom-nav">
            <Link<Route> to={Route::Home} classes={is(Route::Home)}>
                <span class="nav-item__icon">{ "☕" }</span>
                <span class="nav-item__label">{ "Heute" }</span>
            </Link<Route>>
            <Link<Route> to={Route::History} classes={is(Route::History)}>
                <span class="nav-item__icon">{ "📅" }</span>
                <span class="nav-item__label">{ "Verlauf" }</span>
            </Link<Route>>
            <Link<Route> to={Route::Settings} classes={is(Route::Settings)}>
                <span class="nav-item__icon">{ "⚙️" }</span>
                <span class="nav-item__label">{ "Einstellungen" }</span>
            </Link<Route>>
        </nav>
    }
}
