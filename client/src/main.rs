use reqwasm::http::Request;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use uuid::Uuid;
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

    let users = use_state(|| None);
    {
        let id = props.id.clone();
        let users = users.clone();
        use_effect_with_deps(
            move |_| {
                let users = users.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users: Vec<rmmt::User> =
                        Request::get(&format!("/api/account/{}/users", id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    let users_map: HashMap<Uuid, rmmt::User> = fetched_users
                        .into_iter()
                        .map(|u| (u.id.clone(), u))
                        .collect();
                    users.set(Some(Rc::new(users_map)));
                });
                || ()
            },
            (),
        );
    }

    let balance = use_state(|| None);
    {
        let id = props.id.clone();
        let balance = balance.clone();
        use_effect_with_deps(
            move |_| {
                let balance = balance.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_balance: Vec<rmmt::Balance> =
                        Request::get(&format!("/api/account/{}/balance", id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    balance.set(Some(fetched_balance));
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
    let users = users.deref().clone();
    let balance = balance.deref().clone();
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
                        if let (Some(users), Some(balance)) = (users.clone(), balance) {
                            <BalanceList balance={ balance } users={ users } />
                        } else {
                            <Loading/>
                        }
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
                        if let (Some(users), Some(expenditures)) = (users.clone(), expenditures) {
                            <ExpendituresList expenditures={ expenditures } users={ users } limit=10 />
                        } else {
                            <Loading/>
                        }
                        <a class="button is-info fa fa-plus-circle" href="">{ "Nouvelle dépense" }</a>
                    </div>
                </div>
            </section>

            <section class="section">
                <div class="columns">
                    <div class="column">
                        <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                        if let (Some(users), Some(repayments)) = (users.clone(), repayments) {
                            <RepaymentsList repayments={ repayments } users={ users } limit=10 />
                        } else {
                            <Loading/>
                        }
                        <a class="button is-info fa fa-plus-circle" href="">{ "Nouveau remboursement" }</a>
                    </div>
                </div>
            </section>
        </div>
    }
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

#[function_component(CreateAccount)]
fn create_account() -> Html {
    html! {
        <div class="cover">
            { "Hello world" }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BalanceListProps {
    balance: Vec<rmmt::Balance>,
    users: Rc<HashMap<Uuid, rmmt::User>>,
}

#[function_component(BalanceList)]
fn balance_list(
    BalanceListProps {
        balance,
        users,
    }: &BalanceListProps,
) -> Html {
    let max = balance.iter().map(|b| b.amount).max().unwrap_or(0).to_string();
    let mut balance = balance.clone();
    balance.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());

    html! {
        <table class="table is-fullwidth is-striped is-hoverable">
            <tbody>
                {
                    balance.iter().map(|balance| {
                        html! {
                            <tr>
                                <td class="is-vcentered">
                                if balance.amount < 0 {
                                    <div class="progress-wrapper">
                                        <progress class="progress is-large is-danger is-revert" value={ balance.amount.abs().to_string() } max={ max.clone() }>
                                            <Amount amount={ balance.amount } />
                                        </progress>
                                        <p class="progress-value has-text-white"><Amount amount={ balance.amount } /></p>
                                    </div>
                                }
                                </td>
                                <td class="is-vcentered has-text-centered"><UserName users={ users.clone() } id={ balance.user_id }/></td>
                                <td class="is-vcentered">
                                if balance.amount > 0 {
                                    <div class="progress-wrapper">
                                        <progress class="progress is-large is-success" value={ balance.amount.abs().to_string() } max={ max.clone() }>
                                            <Amount amount={ balance.amount } />
                                        </progress>
                                        <p class="progress-value has-text-white"><Amount amount={ balance.amount } /></p>
                                    </div>
                                }
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
struct ExpendituresListProps {
    expenditures: Vec<rmmt::Expenditure>,
    users: Rc<HashMap<Uuid, rmmt::User>>,
    limit: Option<usize>,
}

#[function_component(ExpendituresList)]
fn expenditures_list(
    ExpendituresListProps {
        expenditures,
        users,
        limit,
    }: &ExpendituresListProps,
) -> Html {
    let len = expenditures.len();
    let map = |expenditure: &rmmt::Expenditure| {
        html! {
            <tr>
                <td class="is-vcentered">{ &expenditure.date }</td>
                <td class="is-vcentered">{ &expenditure.name }</td>
                <td class="is-vcentered"><Amount amount={ expenditure.amount } /></td>
                <td class="is-vcentered"><UserName users={ users.clone() } id={ expenditure.payer_id }/></td>
                <td class="is-vcentered">{ "todo" }</td>
                <td class="is-vcentered">
                <a aria-label="Éditer" class="button is-info" href="">
                <i class="fa fa-pencil fa-lg"></i>
                </a>
                <button aria-label="Supprimer" class="button is-danger"><i class="fa fa-trash-o fa-lg"></i></button>
                </td>
                </tr>
        }
    };
    html! {
        <>
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
                match limit {
                    None => expenditures.iter().map(map).collect::<Html>(),
                    Some(limit) => expenditures.iter().take(*limit).map(map).collect::<Html>(),
                }
            }
            </tbody>
            </table>
            if let Some(limit) = limit {
                <a href="">{ format!("Et {} autres…", len - limit) }</a>
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct RepaymentsListProps {
    repayments: Vec<rmmt::Repayment>,
    users: Rc<HashMap<Uuid, rmmt::User>>,
    limit: Option<usize>,
}

#[function_component(RepaymentsList)]
fn repayments_list(
    RepaymentsListProps {
        repayments,
        users,
        limit,
    }: &RepaymentsListProps,
) -> Html {
    let len = repayments.len();
    let map = |repayment: &rmmt::Repayment| {
        html! {
            <tr>
                <td class="is-vcentered">{ &repayment.date }</td>
                <td class="is-vcentered"><UserName users={ users.clone() } id={ repayment.payer_id } /></td>
                <td class="is-vcentered">{ "a remboursé" }</td>
                <td class="is-vcentered"><Amount amount={ repayment.amount } /></td>
                <td class="is-vcentered">{ "à" }</td>
                <td class="is-vcentered"><UserName users={ users.clone() } id={ repayment.beneficiary_id } /></td>
                <td class="is-vcentered">
                    <a aria-label="Éditer" class="button is-info" href="">
                        <i class="fa fa-pencil fa-lg"></i>
                    </a>
                    <button aria-label="Supprimer" class="button is-danger"><i class="fa fa-trash-o fa-lg"></i></button>
                </td>
            </tr>
        }
    };
    html! {
        <>
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
                match limit {
                    Some(limit) => repayments.iter().take(*limit).map(map).collect::<Html>(),
                    None => repayments.iter().map(map).collect::<Html>(),
                }
            }
            </tbody>
            </table>
            if let Some(limit) = limit {
                <a href="">{ format!("Et {} autres…", len - limit) }</a>
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct UserProps {
    users: Rc<HashMap<Uuid, rmmt::User>>,
    id: Uuid,
}

#[function_component(UserName)]
fn user_name(UserProps { users, id }: &UserProps) -> Html {
    html! {
        { &users.get(&id).unwrap().name }
    }
}

#[derive(Properties, PartialEq)]
struct AmountProps {
    amount: i32,
}

#[function_component(Amount)]
fn amount(AmountProps { amount }: &AmountProps) -> Html {
    html! {
        <>
        { *amount as f64 / 100f64 }{ " €" }
        </>
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
