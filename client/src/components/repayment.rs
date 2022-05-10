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
    repayments: Option<Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>>,
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
                            <RepaymentsList account_id={ ctx.props().account_id.clone() } { repayments } { users } loading=false />
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
    pub account_id: String,
    pub limit: Option<usize>,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub repayments: Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>,
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
            let map = |(_, repayment): (&Uuid, &rmmt::Repayment)| {
                html! {
                    <tr>
                        <td class="is-vcentered">{ &repayment.date }</td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ repayment.payer_id } /></td>
                        <td class="is-vcentered">{ "a remboursé" }</td>
                        <td class="is-vcentered"><Amount amount={ repayment.amount as i64 } /></td>
                        <td class="is-vcentered">{ "à" }</td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ repayment.beneficiary_id } /></td>
                        <td class="is-vcentered">
                            <Link<Route> to={Route::EditRepayment { account_id: ctx.props().account_id.clone(), repayment_id: { repayment.id } }}>
                                <a aria-label="Éditer" class="button is-primary" href="">
                                    <i class="fa fa-pencil fa-lg"></i>
                                </a>
                            </Link<Route>>
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

#[derive(Debug, Clone)]
struct DefaultRepayment {
    id: Option<Uuid>,
    payer_id: Option<Uuid>,
    beneficiary_id: Option<Uuid>,
    amount: i32,
    date: NaiveDate,
}

impl From<rmmt::Repayment> for DefaultRepayment {
    fn from(repayment: rmmt::Repayment) -> Self {
        Self {
            id: Some(repayment.id),
            payer_id: Some(repayment.payer_id),
            beneficiary_id: Some(repayment.beneficiary_id),
            amount: repayment.amount,
            date: repayment.date,
        }
    }
}

impl From<rmmt::Balancing> for DefaultRepayment {
    fn from(balancing: rmmt::Balancing) -> Self {
        Self {
            payer_id: Some(balancing.payer_id),
            beneficiary_id: Some(balancing.beneficiary_id),
            amount: balancing.amount as i32,
            ..Default::default()
        }
    }
}

impl Default for DefaultRepayment {
    fn default() -> Self {
        Self {
            id: None,
            payer_id: None,
            beneficiary_id: None,
            amount: 0,
            date: Local::today().naive_local(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct EditRepaymentProps {
    pub account_id: String,
    #[prop_or_default]
    pub repayment_id: Option<Uuid>,
}

pub enum EditRepaymentMsg {
    AccountMsg(AccountMsg),
    Submit,
    Edited { repayment: rmmt::Repayment },
    Error(String),
    ClearError,
}

pub struct EditRepayment {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    default: Option<DefaultRepayment>,
    select_payer: NodeRef,
    input_amount: NodeRef,
    select_beneficiary: NodeRef,
    input_date: NodeRef,
    creating: bool,
    error: Option<String>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
    agent: Dispatcher<AccountAgent>,
}

impl EditRepayment {
    fn save_repayment(&mut self, ctx: &Context<Self>) {
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
        let req = match ctx.props().repayment_id {
            Some(id) => {
                let repayment = rmmt::Repayment {
                    id: id.clone(),
                    account_id: account_id.into(),
                    amount,
                    payer_id,
                    beneficiary_id,
                    date,
                };
                debug!("Update repayment: {:?}", repayment);
                Request::put(&format!("/api/account/{}/repayments/{}", ctx.props().account_id, id)).json(&repayment).unwrap()
            },
            None => {
                let repayment = rmmt::NewRepayment {
                    account_id: account_id.into(),
                    amount,
                    payer_id,
                    beneficiary_id,
                    date,
                };
                debug!("Create repayment: {:?}", repayment);
                Request::post(&format!("/api/account/{}/repayments", ctx.props().account_id)).json(&repayment).unwrap()
            },
        };

        ctx.link().send_future(async move {
            let resp = req.send().await;

            let resp = match resp {
                Err(err) => return EditRepaymentMsg::Error(format!("{}", err)),
                Ok(resp) => resp,
            };

            if !resp.ok() {
                return EditRepaymentMsg::Error(format!(
                    "{}: {}",
                    resp.status(),
                    resp.status_text()
                ));
            }

            let resp = resp.json::<rmmt::Repayment>().await;

            if let Err(err) = resp {
                return EditRepaymentMsg::Error(format!("{}", err));
            }
            EditRepaymentMsg::Edited {
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

impl Component for EditRepayment {
    type Message = EditRepaymentMsg;
    type Properties = EditRepaymentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge =
            AccountAgent::bridge(ctx.link().callback(EditRepaymentMsg::AccountMsg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));
        if let Some(repayment_id) = ctx.props().repayment_id.as_ref() {
            dispatcher.send(AccountMsg::LoadRepayment{ account_id: ctx.props().account_id.clone(), repayment_id: repayment_id.clone() });
        }

        Self {
            account: None,
            users: None,
            default: None,
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
            EditRepaymentMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateAccount(account) => {
                    self.account = Some(account);
                    true
                }
                AccountMsg::UpdateUsers(users) => {
                    self.users = Some(users);
                    true
                }
                AccountMsg::UpdateRepayment(repayment) => {
                    if Some(repayment.id) == ctx.props().repayment_id {
                        self.default = Some(repayment.into());
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            EditRepaymentMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.save_repayment(ctx);
                    true
                }
            }
            EditRepaymentMsg::Edited { repayment } => {
                info!("Edited repayment: {:?}", repayment);
                self.agent.send(AccountMsg::ChangedRepayments);
                self.clear();

                let history = ctx.link().history().unwrap();
                history.push(Route::Account {
                    account_id: ctx.props().account_id.clone(),
                });

                false
            }
            EditRepaymentMsg::Error(error) => {
                self.creating = false;
                self.error = Some(error);
                true
            }
            EditRepaymentMsg::ClearError => {
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let default = match self.default.as_ref() {
            Some(default) => default.clone(),
            None => match ctx.link().location() {
                Some(location) => match location.query::<rmmt::Balancing>() {
                    Err(err) => {
                        error!("Invalid query: {}", err);
                        Default::default()
                    },
                    Ok(balancing) => balancing.into()
                },
                None => Default::default(),
            }
        };

        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            EditRepaymentMsg::Submit
        });

        let delete_error = ctx.link().callback(|_| EditRepaymentMsg::ClearError);

        html! {
            <div class="columns">
                <div class="column">
                    <AccountTitle id={ ctx.props().account_id.clone() } account={ self.account.clone() } />
                    <div class="box">
                        if default.id.is_some() {
                            <Link<Route> to={Route::EditRepayment { account_id: ctx.props().account_id.clone(), repayment_id: ctx.props().repayment_id.unwrap().clone() }}>
                                <h3 class="subtitle is-3">
                                    <span class="icon-text">
                                        <span class="icon"><i class="fa fa-exchange"></i></span>
                                        <span>{ "Remboursement" }</span>
                                    </span>
                                </h3>
                            </Link<Route>>
                        } else {
                            <Link<Route> to={Route::CreateRepayment { account_id: ctx.props().account_id.clone() }}>
                                <h3 class="subtitle is-3">
                                    <span class="icon-text">
                                        <span class="icon"><i class="fa fa-exchange"></i></span>
                                        <span>{ "Nouveau remboursement" }</span>
                                    </span>
                                </h3>
                            </Link<Route>>
                        }
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
                                                            <option value={ user.id.to_string() } selected={ default.payer_id == Some(user.id) }>{ &user.name }</option>
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
                                            <input ref={ self.input_amount.clone() } type="number" min="0" step="0.01" class="input is-primary" required=true placeholder="montant" value={ (default.amount as f64 / 100f64).to_string() } />
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
                                                            <option value={ user.id.to_string() } selected={ default.beneficiary_id == Some(user.id) }>{ &user.name }</option>
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
                                        <input ref={self.input_date.clone()} type="date" class="input is-primary" required=true value={ format!("{}", default.date.format("%Y-%m-%d")) } />
                                    </div>
                                </div>
                                <div class="control">
                                    <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                                    if default.id.is_some() {
                                        <span class="icon">
                                            <i class="fa fa-save" />
                                        </span>
                                        <span>{ "Enregistrer" }</span>
                                    } else {
                                        <span class="icon">
                                            <i class="fa fa-user-plus" />
                                        </span>
                                        <span>{ "Ajouter" }</span>
                                    }
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
