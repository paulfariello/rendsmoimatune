use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use std::ops::Deref;

use rmmt;

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

#[function_component(Account)]
fn account() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| history.push(Route::Home));

    let account = use_state(|| None);
    {
        let account = account.clone();
        use_effect_with_deps(
            move |_| {
                let account = account.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_account: rmmt::Account =
                        Request::get("/account/41EBA85C-9A0A-4BE6-884C-1B31AA379232")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    account.set(Some(fetched_account));
                });
                || ()
            },
            (),
        );
    }

    let account = account.deref().clone();
    html! {
        <div>
            if let Some(account) = account {
                <div>
                    <h1>{ account.name }</h1>
                    <button {onclick}>{ "Go Home" }</button>
                </div>
                <div>
                   <h3>{"Expenditures"}</h3>
                   <ExpendituresList expenditures={account.expenditures} />
                </div>
            } else {
                <div>{ "Loading..." }</div>
            }
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Account { id } => html! {
            <Account />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
struct ExpendituresListProps {
    expenditures: Vec<rmmt::Expenditure>,
}

#[function_component(ExpendituresList)]
fn expenditures_list(ExpendituresListProps { expenditures }: &ExpendituresListProps) -> Html {
    expenditures
        .iter()
        .map(|expenditure| {
            html! {
                <p>{format!("{}: {}", expenditure.name, expenditure.payer)}</p>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <body>
            <h1>{ "Rendsmoimatune" }</h1>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </body>
    }
}

fn main() {
    yew::start_app::<App>();
}
