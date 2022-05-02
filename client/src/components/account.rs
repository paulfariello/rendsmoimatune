use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt;
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{
    Agent, AgentLink, Bridge, Bridged, Context as AgentContext, Dispatched, HandlerId,
};
use yew_router::prelude::*;

use crate::components::user::CreateUser;
use crate::components::{
    balance::BalanceList, expenditure::ExpendituresList, repayment::RepaymentsList,
};
use crate::Route;

#[derive(Debug, Clone)]
pub enum AccountMsg {
    FetchAccount(String),
    UpdateAccount(Rc<RefCell<Option<rmmt::Account>>>),
    FetchUsers,
    UpdateUsers(Rc<RefCell<Option<HashMap<Uuid, rmmt::User>>>>),
    UpdateBalances(Rc<RefCell<Option<Vec<rmmt::Balance>>>>),
    UpdateExpenditures(Rc<RefCell<Option<Vec<rmmt::Expenditure>>>>),
    UpdateRepayments(Rc<RefCell<Option<Vec<rmmt::Repayment>>>>),
}

pub struct AccountAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    account_id: Option<String>,
    account: Rc<RefCell<Option<rmmt::Account>>>,
    users: Rc<RefCell<Option<HashMap<Uuid, rmmt::User>>>>,
    balances: Rc<RefCell<Option<Vec<rmmt::Balance>>>>,
    expenditures: Rc<RefCell<Option<Vec<rmmt::Expenditure>>>>,
    repayments: Rc<RefCell<Option<Vec<rmmt::Repayment>>>>,
}

impl AccountAgent {
    fn fetch_account(&mut self, account_id: String) {
        info!("Fetching account: {}", account_id);
        self.account_id = Some(account_id);
        let account_id = self.account_id.clone().unwrap();
        let account = self.account.clone();
        self.link.send_future(async move {
            account.replace(Some(
                Request::get(&format!("/api/account/{}", account_id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap(),
            ));
            AccountMsg::UpdateAccount(account)
        })
    }

    fn fetch_users(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching users for account: {}", account_id);
                let account_id = account_id.clone();
                let users = self.users.clone();
                self.link.send_future(async move {
                    let fetched_users: Vec<rmmt::User> =
                        Request::get(&format!("/api/account/{}/users", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!(
                        "Fetched {} users for account: {}",
                        fetched_users.len(),
                        account_id
                    );
                    users.replace(Some(
                        fetched_users
                            .into_iter()
                            .map(|u| (u.id.clone(), u))
                            .collect(),
                    ));
                    AccountMsg::UpdateUsers(users)
                });
            }
            None => error!("Cannot fetch users without account id"),
        }
    }

    fn fetch_balances(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching balances for account: {}", account_id);
                let account_id = account_id.clone();
                let balances = self.balances.clone();
                self.link.send_future(async move {
                    let mut fetched_balances: Vec<rmmt::Balance> =
                        Request::get(&format!("/api/account/{}/balance", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!(
                        "Fetched {} balances for account: {}",
                        fetched_balances.len(),
                        account_id
                    );
                    fetched_balances.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());
                    balances.replace(Some(fetched_balances));
                    AccountMsg::UpdateBalances(balances)
                });
            }
            None => error!("Cannot fetch balances without account id"),
        }
    }

    fn fetch_expenditures(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching expenditures for account: {}", account_id);
                let account_id = account_id.clone();
                let expenditures = self.expenditures.clone();
                self.link.send_future(async move {
                    let mut fetched_expenditures: Vec<rmmt::Expenditure> =
                        Request::get(&format!("/api/account/{}/expenditures", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!(
                        "Fetched {} expenditures for account: {}",
                        fetched_expenditures.len(),
                        account_id
                    );
                    fetched_expenditures.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());
                    expenditures.replace(Some(fetched_expenditures));
                    AccountMsg::UpdateExpenditures(expenditures)
                });
            }
            None => error!("Cannot fetch expenditures without account id"),
        }
    }

    fn fetch_repayments(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching repayments for account: {}", account_id);
                let account_id = account_id.clone();
                let repayments = self.repayments.clone();
                self.link.send_future(async move {
                    let mut fetched_repayments: Vec<rmmt::Repayment> =
                        Request::get(&format!("/api/account/{}/repayments", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!(
                        "Fetched {} repayments for account: {}",
                        fetched_repayments.len(),
                        account_id
                    );
                    fetched_repayments.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());
                    repayments.replace(Some(fetched_repayments));
                    AccountMsg::UpdateRepayments(repayments)
                });
            }
            None => error!("Cannot fetch repayments without account id"),
        }
    }

    fn broadcast(&self, msg: <Self as Agent>::Output) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }
}

impl Agent for AccountAgent {
    type Reach = AgentContext<Self>;
    type Message = AccountMsg;
    type Input = AccountMsg;
    type Output = AccountMsg;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            account_id: None,
            account: Rc::new(RefCell::new(None)),
            users: Rc::new(RefCell::new(None)),
            balances: Rc::new(RefCell::new(None)),
            expenditures: Rc::new(RefCell::new(None)),
            repayments: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        self.broadcast(msg);
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        debug!("Handle account msg: {:?}", msg);
        match &msg {
            AccountMsg::FetchAccount(id) => {
                self.fetch_account(id.clone());
                self.fetch_users();
                self.fetch_balances();
                self.fetch_expenditures();
                self.fetch_repayments();
            }
            _ => {}
        }

        self.broadcast(msg);
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

#[derive(Properties, PartialEq)]
pub struct AccountProps {
    pub id: String,
}

pub struct Account {
    account: Rc<RefCell<Option<rmmt::Account>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Account {
    type Message = AccountMsg;
    type Properties = AccountProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();
        // Must be created before dispatcher so we receive initial messages
        let bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));
        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::FetchAccount(id.clone()));
        dispatcher.send(AccountMsg::FetchUsers);
        Self {
            // TODO default?
            account: Rc::new(RefCell::new(None)),
            _account_bridge: bridge,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountMsg::UpdateAccount(account) => {
                self.account = account;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <div class="columns">
                    <div class="column">
                        <Link<Route> to={Route::Account { id: ctx.props().id.clone() }}>
                            <h2 class="title is-1">
                                <i class="fa fa-bank fa-lg fa-fw"/>
                                {
                                    match &*self.account.borrow() {
                                        Some(account) => &account.name,
                                        None => "Loading...",
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
                                <BalanceList />
                            </div>
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
                            <h3 class="subtitle is-3"><a href=""><i class="fa fa-credit-card fa-lg fa-fw"></i>{ "Dépenses" }</a></h3>
                            <ExpendituresList limit=10 />
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
                            <RepaymentsList limit=10 />
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
