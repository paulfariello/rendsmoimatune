use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

use components::{account::{Account, CreateAccount}, utils::TopBar};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/account/:id")]
    Account { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <CreateAccount /> },
        Route::Account { id } => html! {
            <Account id={ id.clone() } />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <body>
            <TopBar/>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
