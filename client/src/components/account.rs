use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::components::{
    balance::BalanceList, expenditure::ExpendituresList, repayment::RepaymentsList, utils::Loading,
};
use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::user::CreateUser;
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
                    users.set(Some(users_map));
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
                        <CreateUser account_id={ props.id.clone() } />
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
                                    <td>
                                        <a class="button is-primary" href="">
                                            <span class="icon">
                                                <i class="fa fa-plus-circle" />
                                            </span>
                                            <span>{ "Rembourser" }</span>
                                        </a>
                                    </td>
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
                        <a class="button is-primary" href="">
                            <span class="icon">
                                <i class="fa fa-plus-circle" />
                            </span>
                            <span>{ "Nouvelle dépense" }</span>
                        </a>
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
                        <a class="button is-primary" href="">
                            <span class="icon">
                                <i class="fa fa-plus-circle" />
                            </span>
                            <span>{ "Nouveau remboursement" }</span>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CreateAccountProps;

pub enum CreateAccountMsg {
    Submit,
    Created { id: String },
}

pub struct CreateAccount {
    creating: bool,
    input_name: NodeRef,
}

impl CreateAccount {
    fn create_account(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account = rmmt::NewAccount { name };
        ctx.link().send_future(async move {
            let created_account: String = Request::post("/api/account/")
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
    }

    fn clear(&mut self) {
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for CreateAccount {
    type Message = CreateAccountMsg;
    type Properties = CreateAccountProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { creating: false, input_name: NodeRef::default() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateAccountMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.create_account(ctx);
                    true
                }
            }
            CreateAccountMsg::Created { id } => {
                info!("Created account: {}", id);
                self.clear();
                let history = ctx.link().history().unwrap();
                history.push(Route::Account { id });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            CreateAccountMsg::Submit
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
                                <form {onsubmit}>
                                    <div class="field has-addons">
                                        <div class={classes!("control", self.creating.then(|| "is-loading"))}>
                                            <input ref={self.input_name.clone()} class="input is-primary" type="text" placeholder="Nom" name="name" required=true />
                                        </div>
                                        <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                                            <span>{ "Créer" }</span>
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        }
    }
}
