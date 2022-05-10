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
    balance::{BalanceList, BalancingList},
    expenditure::ExpendituresList,
    repayment::RepaymentsList,
    user::CreateUser,
    utils::Loading,
};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct AccountProps {
    pub id: String,
}

pub struct Account {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    balance: Option<Rc<RefCell<rmmt::Balance>>>,
    expenditures: Option<Rc<RefCell<HashMap<Uuid, rmmt::Expenditure>>>>,
    repayments: Option<Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>>,
    fetching_users: bool,
    fetching_expenditures: bool,
    fetching_repayments: bool,
    fetching_balance: bool,
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
            balance: None,
            expenditures: None,
            repayments: None,
            fetching_users: false,
            fetching_expenditures: false,
            fetching_repayments: false,
            fetching_balance: false,
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
                self.fetching_balance = true;
                true
            }
            AccountMsg::ChangedExpenditures => {
                self.fetching_expenditures = true;
                self.fetching_balance = true;
                true
            }
            AccountMsg::ChangedRepayments => {
                self.fetching_repayments = true;
                self.fetching_balance = true;
                true
            }
            AccountMsg::UpdateUsers(users) => {
                self.fetching_users = false;
                self.users = Some(users);
                true
            }
            AccountMsg::UpdateBalance(balance) => {
                self.fetching_balance = false;
                self.balance = Some(balance);
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
                        if let (Some(users), Some(balance)) = (self.users.clone(), self.balance.clone()) {
                            <BalanceList { users } { balance } loading={ self.fetching_balance } />
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
                        if let (Some(users), Some(balance)) = (self.users.clone(), self.balance.clone()) {
                            <BalancingList account_id={ ctx.props().id.clone() } { users } { balance } loading={ self.fetching_balance } />
                        } else {
                            <Loading />
                        }
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
                            <RepaymentsList account_id={ ctx.props().id.clone() } { users } { repayments } limit=10 loading={ self.fetching_repayments } />
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
