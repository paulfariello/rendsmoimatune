use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use chrono::naive::NaiveDate;
use chrono::Local;
use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};
use yew_router::prelude::*;

use crate::agent::{AccountAgent, AccountMsg};
use crate::components::{
    account::AccountTitle,
    user::UserName,
    utils::{Amount, Loading},
};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct RepaymentsProps {
    pub account_id: String,
}

pub struct Repayments {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    repayments: Option<Rc<RefCell<Vec<rmmt::Repayment>>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Repayments {
    type Message = AccountMsg;
    type Properties = RepaymentsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));

        Self {
            account: None,
            repayments: None,
            users: None,
            _account_bridge: account_bridge,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountMsg::UpdateAccount(account) => {
                self.account = Some(account);
                true
            }
            AccountMsg::UpdateUsers(users) => {
                self.users = Some(users);
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
            <div class="columns">
                <div class="column">
                    <Link<Route> to={Route::Account { account_id: ctx.props().account_id.clone() }}>
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
                    <div class="box">
                        <Link<Route> to={Route::Repayments { account_id: ctx.props().account_id.clone() }}>
                        <h3 class="subtitle is-3">
                            <span class="icon-text">
                                <span class="icon"><i class="fa fa-exchange"></i></span>
                                <span>{ "Remboursements" }</span>
                            </span>
                        </h3>
                        </Link<Route>>
                        if let (Some(users), Some(repayments)) = (self.users.clone(), self.repayments.clone()) {
                            <RepaymentsList { repayments } { users } loading=false />
                        } else {
                            <Loading />
                        }
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RepaymentsListProps {
    pub limit: Option<usize>,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub repayments: Rc<RefCell<Vec<rmmt::Repayment>>>,
    pub loading: bool,
}

pub struct RepaymentsList;

impl Component for RepaymentsList {
    type Message = ();
    type Properties = RepaymentsListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let repayments = &*ctx.props().repayments.borrow();
        let len = repayments.len();

        if len > 0 {
            let map = |repayment: &rmmt::Repayment| {
                html! {
                    <tr>
                        <td class="is-vcentered">{ &repayment.date }</td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ repayment.payer_id } /></td>
                        <td class="is-vcentered">{ "a remboursé" }</td>
                        <td class="is-vcentered"><Amount amount={ repayment.amount as i64 } /></td>
                        <td class="is-vcentered">{ "à" }</td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ repayment.beneficiary_id } /></td>
                        <td class="is-vcentered">
                            <a aria-label="Éditer" class="button is-primary" href="">
                                <i class="fa fa-pencil fa-lg"></i>
                            </a>
                            <DeleteRepayment account_id={ repayment.account_id.clone() } id={ repayment.id.clone() } />
                        </td>
                    </tr>
                }
            };
            html! {
                <div class="is-relative block">
                    if ctx.props().loading {
                        <div class="loading-overlay">
                            <Loading />
                        </div>
                    }
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
                        match ctx.props().limit {
                            Some(limit) => repayments.iter().take(limit).map(map).collect::<Html>(),
                            None => repayments.iter().map(map).collect::<Html>(),
                        }
                    }
                    </tbody>
                    </table>
                    if let Some(limit) = ctx.props().limit {
                        if len > limit {
                            <a href="">{ format!("Et {} autres…", len - limit) }</a>
                        }
                    }
                </div>
            }
        } else {
            html! {}
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CreateRepaymentProps {
    pub account_id: String,
}

pub enum CreateRepaymentMsg {
    AccountMsg(AccountMsg),
    Submit,
    Created { repayment: rmmt::Repayment },
    Error(String),
    ClearError,
}

pub struct CreateRepayment {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    select_payer: NodeRef,
    input_amount: NodeRef,
    select_beneficiary: NodeRef,
    input_date: NodeRef,
    creating: bool,
    error: Option<String>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
    agent: Dispatcher<AccountAgent>,
}

impl CreateRepayment {
    fn create_repayment(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let select_payer = self
            .select_payer
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        let payer_id = Uuid::parse_str(&select_payer.value()).unwrap();

        let input_amount = self
            .input_amount
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        let amount = input_amount.value().parse::<f32>().unwrap();
        let amount = (amount * 100f32).round() as i32;

        let select_beneficiary = self
            .select_beneficiary
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        let beneficiary_id = Uuid::parse_str(&select_beneficiary.value()).unwrap();

        let input_date = self.input_date.cast::<web_sys::HtmlInputElement>().unwrap();
        let date = NaiveDate::parse_from_str(&input_date.value(), "%Y-%m-%d").unwrap();

        let account_id: UniqId = ctx.props().account_id.clone().try_into().unwrap();
        let repayment = rmmt::NewRepayment {
            account_id: account_id.into(),
            amount,
            payer_id,
            beneficiary_id,
            date,
        };
        let url = format!("/api/account/{}/repayments", ctx.props().account_id);
        ctx.link().send_future(async move {
            let resp = Request::post(&url).json(&repayment).unwrap().send().await;

            let resp = match resp {
                Err(err) => return CreateRepaymentMsg::Error(format!("{}", err)),
                Ok(resp) => resp,
            };

            if !resp.ok() {
                return CreateRepaymentMsg::Error(format!(
                    "{}: {}",
                    resp.status(),
                    resp.status_text()
                ));
            }

            let resp = resp.json::<rmmt::Repayment>().await;

            if let Err(err) = resp {
                return CreateRepaymentMsg::Error(format!("{}", err));
            }
            CreateRepaymentMsg::Created {
                repayment: resp.unwrap(),
            }
        });
    }

    fn clear(&mut self) {
        self.creating = false;
        self.error = None;

        let input_amount = self
            .input_amount
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        input_amount.set_value("0");

        let today = Local::today();
        let input_date = self.input_date.cast::<web_sys::HtmlInputElement>().unwrap();
        input_date.set_value(&format!("{}", today.format("%Y-%m-%d")));
    }
}

impl Component for CreateRepayment {
    type Message = CreateRepaymentMsg;
    type Properties = CreateRepaymentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge =
            AccountAgent::bridge(ctx.link().callback(CreateRepaymentMsg::AccountMsg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));

        Self {
            account: None,
            users: None,
            select_payer: NodeRef::default(),
            input_amount: NodeRef::default(),
            select_beneficiary: NodeRef::default(),
            input_date: NodeRef::default(),
            creating: false,
            error: None,
            _account_bridge: account_bridge,
            agent: AccountAgent::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateRepaymentMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateAccount(account) => {
                    self.account = Some(account);
                    true
                }
                AccountMsg::UpdateUsers(users) => {
                    self.users = Some(users);
                    true
                }
                _ => false,
            },
            CreateRepaymentMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.create_repayment(ctx);
                    true
                }
            }
            CreateRepaymentMsg::Created { repayment } => {
                info!("Created repayment: {:?}", repayment);
                self.agent.send(AccountMsg::ChangedRepayments);
                self.clear();

                let history = ctx.link().history().unwrap();
                history.push(Route::Account {
                    account_id: ctx.props().account_id.clone(),
                });

                false
            }
            CreateRepaymentMsg::Error(error) => {
                self.creating = false;
                self.error = Some(error);
                true
            }
            CreateRepaymentMsg::ClearError => {
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let balancing: Option<rmmt::Balancing> = match ctx.link().location() {
            Some(location) => location.query().ok(),
            None => None
        };

        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            CreateRepaymentMsg::Submit
        });

        let delete_error = ctx.link().callback(|_| CreateRepaymentMsg::ClearError);

        let today = format!("{}", Local::today().format("%Y-%m-%d"));

        html! {
            <div class="columns">
                <div class="column">
                    <AccountTitle id={ ctx.props().account_id.clone() } account={ self.account.clone() } />
                    <div class="box">
                        <Link<Route> to={Route::CreateRepayment { account_id: ctx.props().account_id.clone() }}>
                            <h3 class="subtitle is-3">
                                <span class="icon-text">
                                    <span class="icon"><i class="fa fa-exchange"></i></span>
                                    <span>{ "Nouveau remboursement" }</span>
                                </span>
                            </h3>
                        </Link<Route>>
                        if let Some(error) = self.error.as_ref() {
                            <div class="notification is-danger">
                              <button class="delete" onclick={delete_error}></button>
                              { error }
                            </div>
                        }
                        if let Some(users) = self.users.clone() {
                            <form {onsubmit}>
                                <div class="field is-horizontal">
                                    <div class="field-body">
                                        <div class="field">
                                            <p class="control is-expanded has-icons-left">
                                                <div class="select is-fullwidth is-primary">
                                                    <select ref={ self.select_payer.clone() } required=true>
                                                    {
                                                        (&*users.borrow()).iter().map(|(_, user)| html! {
                                                            <option value={ user.id.to_string() }>{ &user.name }</option>
                                                        }).collect::<Html>()
                                                    }
                                                    </select>
                                                </div>
                                                <span class="icon is-small is-left">
                                                    <i class="fa fa-user"></i>
                                                </span>
                                            </p>
                                        </div>
                                        <div class="field">
                                            <label class="label is-large">{ "rembourse" }</label>
                                        </div>
                                        <div class="field has-addons">
                                            <div class="control is-expanded">
                                            <input ref={ self.input_amount.clone() } type="number" min="0" class="input is-primary" required=true placeholder="montant" value={ balancing.map(|b| (b.amount as f64 / 100f64).to_string() ) } />
                                            </div>
                                            <div class="control">
                                                <p class="button is-static">{ "€" }</p>
                                            </div>
                                        </div>
                                        <div class="field">
                                            <label class="label is-large">{ "à" }</label>
                                        </div>
                                        <div class="field">
                                            <p class="control is-expanded has-icons-left">
                                                <div class="select is-fullwidth is-primary">
                                                    <select ref={ self.select_beneficiary.clone() } required=true>
                                                    {
                                                        (&*users.borrow()).iter().map(|(_, user)| html! {
                                                            <option value={ user.id.to_string() }>{ &user.name }</option>
                                                        }).collect::<Html>()
                                                    }
                                                    </select>
                                                </div>
                                                <span class="icon is-small is-left">
                                                    <i class="fa fa-user"></i>
                                                </span>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                                <div class="field">
                                    <div class="control">
                                        <input ref={self.input_date.clone()} type="date" class="input is-primary" required=true value={ today } />
                                    </div>
                                </div>
                                <div class="control">
                                    <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                                        <span class="icon">
                                            <i class="fa fa-user-plus" />
                                        </span>
                                        <span>{ "Ajouter" }</span>
                                    </button>
                                </div>
                            </form>
                        } else {
                            <Loading />
                        }
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DeleteRepaymentProps {
    pub account_id: Uuid,
    pub id: Uuid,
}

pub enum DeleteRepaymentMsg {
    Delete,
    Deleted,
    Error(String),
}

struct DeleteRepayment {
    deleting: bool,
    agent: Dispatcher<AccountAgent>,
    error: Option<String>,
}

impl DeleteRepayment {
    fn delete_repayment(&mut self, ctx: &Context<Self>) {
        self.deleting = true;

        let url = format!(
            "/api/account/{}/repayments/{}",
            UniqId::from(ctx.props().account_id),
            ctx.props().id
        );
        ctx.link().send_future(async move {
            let resp = Request::delete(&url).send().await;

            let resp = match resp {
                Err(err) => return DeleteRepaymentMsg::Error(format!("{}", err)),
                Ok(resp) => resp,
            };

            if !resp.ok() {
                return DeleteRepaymentMsg::Error(format!(
                    "{}: {}",
                    resp.status(),
                    resp.status_text()
                ));
            }

            DeleteRepaymentMsg::Deleted
        });
    }
}

impl Component for DeleteRepayment {
    type Message = DeleteRepaymentMsg;
    type Properties = DeleteRepaymentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            deleting: false,
            error: None,
            agent: AccountAgent::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeleteRepaymentMsg::Delete => {
                if self.deleting {
                    false
                } else {
                    self.error = None;
                    self.delete_repayment(ctx);
                    true
                }
            }
            DeleteRepaymentMsg::Deleted => {
                self.deleting = false;
                self.agent.send(AccountMsg::ChangedRepayments);
                true
            }
            DeleteRepaymentMsg::Error(error) => {
                error!("Cannot delete repayment: {}", error);
                self.deleting = false;
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| DeleteRepaymentMsg::Delete);

        html! {
            <button aria-label="Supprimer" class={ classes!("button", "is-danger", self.deleting.then(|| "is-loading")) } { onclick }>
                <i class="fa fa-trash-o fa-lg"></i>
            </button>
        }
    }
}
