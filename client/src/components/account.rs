use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt;
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched};
use yew_router::prelude::*;

use crate::agent::{AccountAgent, AccountMsg};
use crate::components::{
    balance::BalanceList, expenditure::ExpendituresList, repayment::RepaymentsList,
    user::CreateUser, utils::Loading,
};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct AccountProps {
    pub id: String,
}

pub struct Account {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    balances: Option<Rc<RefCell<Vec<rmmt::Balance>>>>,
    expenditures: Option<Rc<RefCell<Vec<rmmt::Expenditure>>>>,
    repayments: Option<Rc<RefCell<Vec<rmmt::Repayment>>>>,
    fetching_users: bool,
    fetching_expenditures: bool,
    fetching_repayments: bool,
    fetching_balances: bool,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Account {
    type Message = AccountMsg;
    type Properties = AccountProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();
        let account_bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));
        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::LoadAccount(id.clone()));
        Self {
            account: None,
            users: None,
            balances: None,
            expenditures: None,
            repayments: None,
            fetching_users: false,
            fetching_expenditures: false,
            fetching_repayments: false,
            fetching_balances: false,
            _account_bridge: account_bridge,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountMsg::UpdateAccount(account) => {
                self.account = Some(account);
                true
            }
            AccountMsg::ChangedUsers => {
                self.fetching_users = true;
                self.fetching_balances = true;
                true
            }
            AccountMsg::ChangedExpenditures => {
                self.fetching_expenditures = true;
                self.fetching_balances = true;
                true
            }
            AccountMsg::ChangedRepayments => {
                self.fetching_repayments = true;
                self.fetching_balances = true;
                true
            }
            AccountMsg::UpdateUsers(users) => {
                self.fetching_users = false;
                self.users = Some(users);
                true
            }
            AccountMsg::UpdateBalances(balances) => {
                self.fetching_balances = false;
                self.balances = Some(balances);
                true
            }
            AccountMsg::UpdateExpenditures(expenditures) => {
                self.fetching_expenditures = false;
                self.expenditures = Some(expenditures);
                true
            }
            AccountMsg::UpdateRepayments(repayments) => {
                self.fetching_repayments = false;
                self.repayments = Some(repayments);
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="columns">
                <div class="column">
                    <Link<Route> to={Route::Account { account_id: ctx.props().id.clone() }}>
                        <h2 class="title is-1">
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="fa fa-bank"/>
                                </span>
                                <span>
                                {
                                    match &self.account {
                                        Some(account) => {
                                            let account = &*account.borrow();
                                            account.name.clone()
                                        }
                                        None => "Loading...".to_string(),
                                    }
                                }
                                </span>
                            </span>
                        </h2>
                    </Link<Route>>
                </div>
            </div>
            <div class="tile is-ancestor">
                <div class="tile is-parent is-6">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon-text">
                                <span class="icon"><i class="fa fa-balance-scale"></i></span>
                                <span>{ "Balance" }</span>
                            </span>
                        </h3>
                        if let (Some(users), Some(balances)) = (self.users.clone(), self.balances.clone()) {
                            <BalanceList { users } { balances } loading={ self.fetching_balances } />
                        } else {
                            <Loading />
                        }
                        <CreateUser account_id={ ctx.props().id.clone() } />
                    </div>
                </div>

                <div class="tile is-parent is-6">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon-text">
                                <span class="icon"><i class="fa fa-exchange"></i></span>
                                <span>{ "Équilibrage" }</span>
                            </span>
                        </h3>
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
                        <Link<Route> to={Route::Expenditures { account_id: ctx.props().id.clone() }}>
                            <h3 class="subtitle is-3">
                                <span class="icon-text">
                                    <span class="icon"><i class="fa fa-credit-card"></i></span>
                                    <span>{ "Dépenses" }</span>
                                </span>
                            </h3>
                        </Link<Route>>
                        if let (Some(users), Some(expenditures)) = (self.users.clone(), self.expenditures.clone()) {
                            <ExpendituresList { expenditures } { users } limit=10 loading={ self.fetching_expenditures } />
                        } else {
                            <Loading />
                        }
                        <Link<Route> to={Route::CreateExpenditure { account_id: ctx.props().id.clone() }}>
                            <a class="button is-primary" href="">
                                <span class="icon">
                                    <i class="fa fa-plus-circle" />
                                </span>
                                <span>{ "Nouvelle dépense" }</span>
                            </a>
                        </Link<Route>>
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <Link<Route> to={Route::Repayments { account_id: ctx.props().id.clone() }}>
                            <h3 class="subtitle is-3">
                                <span class="icon-text">
                                    <span class="icon"><i class="fa fa-exchange"></i></span>
                                    <span>{ "Remboursements" }</span>
                                </span>
                            </h3>
                        </Link<Route>>
                        if let (Some(users), Some(repayments)) = (self.users.clone(), self.repayments.clone()) {
                            <RepaymentsList { users } { repayments } limit=10 loading={ self.fetching_repayments } />
                        } else {
                            <Loading />
                        }
                        <Link<Route> to={Route::CreateRepayment { account_id: ctx.props().id.clone() }}>
                            <a class="button is-primary" href="">
                                <span class="icon">
                                    <i class="fa fa-plus-circle" />
                                </span>
                                <span>{ "Nouveau remboursement" }</span>
                            </a>
                        </Link<Route>>
                    </div>
                </div>
            </div>
            </>
        }
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
        Self {
            creating: false,
            input_name: NodeRef::default(),
        }
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
                history.push(Route::Account { account_id: id });
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

#[derive(Properties, PartialEq)]
pub struct AccountTitleProps {
    pub id: String,
    pub account: Option<Rc<RefCell<rmmt::Account>>>,
}

#[function_component(AccountTitle)]
pub fn account_title(AccountTitleProps { id, account }: &AccountTitleProps) -> Html {
    html! {
        <Link<Route> to={Route::Account { account_id: id.clone() }}>
            <h2 class="title is-1">
                <span class="icon-text">
                    <span class="icon">
                        <i class="fa fa-bank"/>
                    </span>
                    <span>
                    {
                        match account {
                            Some(account) => {
                                let account = &*account.borrow();
                                account.name.clone()
                            }
                            None => "Loading...".to_string(),
                        }
                    }
                    </span>
                </span>
            </h2>
        </Link<Route>>
    }
}
