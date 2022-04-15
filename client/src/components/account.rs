use crate::components::{
    balance::BalanceList, expenditure::ExpendituresList, repayment::RepaymentsList, utils::Loading,
};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use gloo_net::http::Request;
use rmmt;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::FormData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct AccountProps {
    pub id: String,
}

#[function_component(Account)]
pub fn account(props: &AccountProps) -> Html {
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
                    <Link<Route> to={Route::Account { id: props.id.clone() }}>
                        <h2 class="title is-1">
                            <i class="fa fa-bank fa-lg fa-fw"/>
                            {
                                match account {
                                    Some(account) => account.name,
                                    None => "Loading...".to_string(),
                                }
                            }
                        </h2>
                    </Link<Route>>
                </div>

            </div>
            <div class="tile is-ancestor">
                <div class="tile is-parent is-6">
                    <div class="tile is-child box">
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
                        <form>
                            <div class="field has-addons">
                                <div class="control">
                                    <input type="text" class="input" required=true />
                                </div>
                                <div class="control">
                                    <button type="submit" class="button is-info fa fa-user-plus">{ "Ajouter" }</button>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>

                <div class="tile is-parent is-6">
                    <div class="tile is-child box">
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
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                        if let (Some(users), Some(expenditures)) = (users.clone(), expenditures) {
                            <ExpendituresList expenditures={ expenditures } users={ users } limit=10 />
                        } else {
                            <Loading/>
                        }
                        <a class="button is-info fa fa-plus-circle" href="">{ "Nouvelle dépense" }</a>
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Remboursements" }</a></h3>
                        if let (Some(users), Some(repayments)) = (users.clone(), repayments) {
                            <RepaymentsList repayments={ repayments } users={ users } limit=10 />
                        } else {
                            <Loading/>
                        }
                        <a class="button is-info fa fa-plus-circle" href="">{ "Nouveau remboursement" }</a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CreateAccountProps;

pub enum CreateAccountMsg {
    Submit { name: String },
    Created { id: String },
}

pub struct CreateAccount {
    creating: bool,
}

impl Component for CreateAccount {
    type Message = CreateAccountMsg;
    type Properties = CreateAccountProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { creating: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateAccountMsg::Submit { name } => {
                self.creating = true;
                let account = rmmt::NewAccount {
                    name
                };
                ctx.link().send_future(async move {
                    let created_account: String =
                        Request::post("/api/account/")
                            .json(&account)
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    CreateAccountMsg::Created {
                        id: created_account,
                    }
                });
                true
            }
            CreateAccountMsg::Created { id } => {
                info!("Created account: {}", id);
                let history = ctx.link().history().unwrap();
                history.push(Route::Account { id });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            let target = event
                .target()
                .unwrap()
                .dyn_ref::<web_sys::HtmlFormElement>()
                .unwrap()
                .clone();
            let data: FormData = FormData::new_with_form(&target).unwrap();
            CreateAccountMsg::Submit {
                name: data.get("name").as_string().unwrap(),
            }
        });

        html! {
            <div class="cover">
                <div class="container">
                    <section class="section">
                        <div class="columns">
                            <div class="column">
                                <h3 class="subtitle is-3">
                                    { "Créer un nouveau compte" }
                                </h3>
                                if self.creating {
                                    <Loading/>
                                } else {
                                    <form {onsubmit}>
                                        <div class="field has-addons">
                                            <div class="control">
                                                <input class="input" type="text" placeholder="Nom" name="name" />
                                            </div>
                                            <div class="control">
                                                <button class="button is-info" type="submit">{ "Créer" }</button>
                                            </div>
                                        </div>
                                    </form>
                                }
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        }
    }
}