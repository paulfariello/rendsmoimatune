use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use uuid::Uuid;
use rmmt;
use yew::prelude::*;
use yew_agent::{
    Bridge, Bridged, Dispatched,
};
use yew_router::prelude::*;

use crate::components::{
    balance::BalanceList, expenditure::ExpendituresList, repayment::RepaymentsList,
    utils::Loading, user::CreateUser,
};
use crate::Route;
use crate::agent::{AccountMsg, AccountAgent};

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
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Account {
    type Message = AccountMsg;
    type Properties = AccountProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();
        let account_bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));
        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::FetchAccount(id.clone()));
        Self {
            account: None,
            users: None,
            balances: None,
            expenditures: None,
            repayments: None,
            fetching_users: false,
            _account_bridge: account_bridge,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountMsg::UpdateAccount(account) => {
                self.account = Some(account);
                true
            }
            AccountMsg::FetchUsers => {
                self.fetching_users = true;
                true
            }
            AccountMsg::UpdateUsers(users) => {
                self.fetching_users = false;
                self.users = Some(users);
                true
            }
            AccountMsg::UpdateBalances(balances) => {
                self.balances = Some(balances);
                true
            }
            AccountMsg::UpdateExpenditures(expenditures) => {
                self.expenditures = Some(expenditures);
                true
            }
            AccountMsg::UpdateRepayments(repayments) => {
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
                            <i class="fa fa-bank fa-lg fa-fw"/>
                            {
                                match &self.account {
                                    Some(account) => {
                                        let account = &*account.borrow();
                                        account.name.clone()
                                    }
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
                        if let (Some(users), Some(balances)) = (self.users.clone(), self.balances.clone()) {
                            <BalanceList users={ users } balances={ balances } loading={ self.fetching_users } />
                        } else {
                            <Loading />
                        }
                        <CreateUser account_id={ ctx.props().id.clone() } />
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
                        <Link<Route> to={Route::Expenditures { account_id: ctx.props().id.clone() }}>
                            <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                        </Link<Route>>
                        if let (Some(users), Some(expenditures)) = (self.users.clone(), self.expenditures.clone()) {
                            <ExpendituresList expenditures={ expenditures } users={ users } limit=10 />
                        } else {
                            <Loading />
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
                        if let (Some(users), Some(repayments)) = (self.users.clone(), self.repayments.clone()) {
                            <RepaymentsList users={ users } repayments={ repayments } limit=10 />
                        } else {
                            <Loading />
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
