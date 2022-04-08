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
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">{ "Rends-moi ma thune" }<small>{ "beta" }</small></a>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" href="">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navbarBasicExample" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="/">{ "Home" }</a>
                </div>
            </div>
        </nav>
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

    let repayments = use_state(|| None);
    {
        let id = props.id.clone();
        let repayments = repayments.clone();
        use_effect_with_deps(
            move |_| {
                let repayments = repayments.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_repayments: Vec<rmmt::Repayment> =
                        Request::get(&format!("/api/account/{}/repayments", id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    repayments.set(Some(fetched_repayments));
                });
                || ()
            },
            (),
        );
    }

    let account = account.deref().clone();
    let expenditures = expenditures.deref().clone();
    let repayments = repayments.deref().clone();
    html! {
        <div class="container">
            <div class="columns">
                <div class="column">
                    <a href="/">
                        <h2 class="title is-1">
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

            <section class="section">
                <div class="columns">
                    <div class="column">
                        <h3 class="subtitle is-3">
                            <i class="fa fa-balance-scale fa-lg fa-fw"/>
                            { "Balance" }
                        </h3>
                        <div class="balance">
                        </div>
                    </div>
                </div>
            </section>

            <section class="section">
                <div class="columns">
                    <form>
                        <div class="column">
                            <h4 class="subtitle is-3">
                                <i class="fa fa-user fa-lg fa-fw"></i>
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
            </section>

            <section class="section">
                <div class="columns">
                    <div class="column">
                        <h3 class="subtitle is-3"><i class="fa fa-exchange fa-lg fa-fw"></i> { "Équilibrage" }</h3>
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
            </section>

            <section class="section">
                <div class="columns">
                    <div class="column">
                        <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                        if let Some(expenditures) = expenditures {
                            <ExpendituresList expenditures={expenditures} />
                        } else {
                            <Loading/>
                        }
                        <a href="">{ "Et XX autres…" }</a>
                        <a class="button is-info fa fa-plus-circle" href="">{ "Nouvelle dépense" }</a>
                    </div>
                </div>
            </section>

            <section class="section">
                <div class="columns">
                    <div class="column">
                        <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                        if let Some(repayments) = repayments {
                            <RepaymentsList repayments={repayments} />
                        } else {
                            <Loading/>
                        }
                        <a href="">{ "Et XX autres…" }</a>
                        <a class="button is-info fa fa-plus-circle" href="">{ "Nouveau remboursement" }</a>
                    </div>
                </div>
            </section>
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
        <table class="table is-fullwidth is-striped is-hoverable">
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
                                <a aria-label="Éditer" class="button is-info" href="">
                                    <i class="fa fa-pencil fa-lg"></i>
                                </a>
                                <button aria-label="Supprimer" class="button is-danger"><i class="fa fa-trash-o fa-lg"></i></button>
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

#[derive(Properties, PartialEq)]
struct RepaymentsListProps {
    repayments: Vec<rmmt::Repayment>,
}

#[function_component(RepaymentsList)]
fn repayments_list(RepaymentsListProps { repayments }: &RepaymentsListProps) -> Html {
    html! {
        <table class="table is-fullwidth is-striped is-hoverable">
            <thead>
                <tr>
                    <th>{ "Date" }</th>
                    <th>{ "De" }</th>
                    <th></th>
                    <th>{ "Montant" }</th>
                    <th></th>
                    <th>{ "Payeur" }</th>
                    <th>{ "Actions" }</th>
                </tr>
            </thead>
        <tbody>
        {
            repayments
                .iter()
                .map(|repayment| {
                    html! {
                        <tr>
                            <td>{ &repayment.date }</td>
                            <td>{ &repayment.payer_id }</td>
                            <td>{ "a remboursé" }</td>
                            <td>{ &repayment.amount }{ " €" }</td>
                            <td>{ "à" }</td>
                            <td>{ &repayment.beneficiary_id }</td>
                            <td>
                                <a aria-label="Éditer" class="button is-info" href="">
                                    <i class="fa fa-pencil fa-lg"></i>
                                </a>
                                <button aria-label="Supprimer" class="button is-danger"><i class="fa fa-trash-o fa-lg"></i></button>
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
