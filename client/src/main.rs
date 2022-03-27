use reqwasm::http::Request;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

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

#[function_component(TopBar)]
fn top_bar() -> Html {
    html! {
        <div class="navbar">
          <div class="container-fluid">
            <h1>
                <a class="navbar-brand" href="/">{ "Rendsmoimatune" }</a>
                <small>{ "Beta" }</small>
            </h1>
          </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AccountProps {
    id: String,
}

#[function_component(Account)]
fn account(props: &AccountProps) -> Html {
    let account = use_state(|| None);
    {
        let id = props.id.clone();
        let account = account.clone();
        use_effect_with_deps(
            move |_| {
                let account = account.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_account: rmmt::Account =
                        Request::get(&format!("/api/account/{}", id))
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

    let expenditures = use_state(|| None);
    {
        let id = props.id.clone();
        let expenditures = expenditures.clone();
        use_effect_with_deps(
            move |_| {
                let expenditures = expenditures.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_expenditures: Vec<rmmt::Expenditure> =
                        Request::get(&format!("/api/account/{}/expenditures", id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    expenditures.set(Some(fetched_expenditures));
                });
                || ()
            },
            (),
        );
    }

    let account = account.deref().clone();
    let expenditures = expenditures.deref().clone();
    html! {
        <div class="container">
            <div class="row">
                <div class="col">
                    <a href="/">
                        <h2>
                            <i class="fa fa-bank fa-lg fa-fw"/>
                            {
                                match account {
                                    Some(account) => account.name,
                                    None => "Loading...".to_string(),
                                }
                            }
                        </h2>
                    </a>
                </div>
            </div>

            <div class="row">
                <div class="col">
                    <h3>
                        <i class="fa fa-balance-scale fa-lg fa-fw"/>
                        { "Balance" }
                    </h3>
                    <div class="balance">
                    </div>
                </div>
            </div>

            <div class="row">
                <form>
                    <div class="col">
                        <h4>
                            { "Nouveau participant" }
                        </h4>
                        <div class="input-group">
                            <input type="text" class="input-group-field" required=true/>
                            <div class="input-group-button">
                                <button type="submit" class="button fa fa-user-plus">{ "Ajouter" }</button>
                            </div>
                        </div>
                    </div>
                </form>
            </div>

            <div class="row">
                <div class="col">
                    <h3><i class="fa fa-exchange fa-lg fa-fw"></i> { "Équilibrage" }</h3>
                    <table>
                        <thead>
                            <tr>
                                <th> { "De" }</th>
                                <th></th>
                                <th> { "Montant" }</th>
                                <th></th>
                                <th>{ "À" }</th>
                                <th>{ "Action" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{ "john" }</td>
                                <td>{ "doit" }</td>
                                <td>{ 2970.65 }{ " €" }</td>
                                <td>{ "à" }</td>
                                <td>{ "john" }</td>
                                <td><a class="fa fa-plus-circle button" href="">{ "Ajouter" }</a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="row">
                <div class="col">
                    <h3><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                    if let Some(expenditures) = expenditures {
                        <ExpendituresList expenditures={expenditures} />
                    } else {
                        <Loading/>
                    }
                    <a href="">{ "Et 83 autres…" }</a>
                    <a class="button float-right fa fa-plus-circle" href="">{ "Nouvelle dépense" }</a>
                </div>
            </div>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Account { id } => html! {
            <Account id={ id.clone() } />
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
    html! {
        <table>
            <thead>
                <tr>
                    <th>{ "Date" }</th>
                    <th>{ "Nom" }</th>
                    <th>{ "Montant" }</th>
                    <th>{ "Payeur" }</th>
                    <th>{ "Participants" }</th>
                    <th>{ "Actions" }</th>
                </tr>
            </thead>
        <tbody>
        {
            expenditures
                .iter()
                .map(|expenditure| {
                    html! {
                        <tr>
                            <td>{ &expenditure.date }</td>
                            <td>{ &expenditure.name }</td>
                            <td>{ &expenditure.amount }{ " €" }</td>
                            <td>{ &expenditure.payer_id }</td>
                            <td>{ "todo" }</td>
                            <td>
                                <a aria-label="Éditer" class="button" href="">
                                    <i class="fa fa-pencil fa-lg"></i>
                                </a>
                                <button aria-label="Supprimer" class="button alert"><i class="fa fa-trash-o fa-lg"></i></button>
                            </td>
                        </tr>
                    }
                })
                .collect::<Html>()
        }
        </tbody>
        </table>
    }
}

#[function_component(Loading)]
fn loading() -> Html {
    html! {
        <div class="loading">
            { "Loading..." }
        </div>
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
    yew::start_app::<App>();
}
