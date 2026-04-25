use crate::auth::{load_email_from_storage, load_token_from_storage, AuthAction, AuthState};
use crate::components::nav::Nav;
use crate::pages::history::HistoryPage;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::register::RegisterPage;
use crate::pages::settings::SettingsPage;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::History => html! { <HistoryPage /> },
        Route::Settings => html! { <SettingsPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Register => html! { <RegisterPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let auth = use_reducer(|| {
        let token = load_token_from_storage();
        let email = load_email_from_storage();
        if token.is_some() {
            AuthState { token, email }
        } else {
            AuthState::default()
        }
    });

    let show_nav = auth.is_authenticated();

    html! {
        <ContextProvider<crate::auth::AuthContext> context={auth}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
                if show_nav {
                    <Nav />
                }
            </BrowserRouter>
        </ContextProvider<crate::auth::AuthContext>>
    }
}
