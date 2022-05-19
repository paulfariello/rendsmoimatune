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
    _agent: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Repayments {
    type Message = AccountMsg;
    type Properties = RepaymentsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut agent = AccountAgent::bridge(ctx.link().callback(|msg| msg));

        agent.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));

        Self {
            account: None,
            repayments: None,
            users: None,
            _agent: agent,
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
            <>
            <AccountTitle id={ ctx.props().account_id.clone() } account={ self.account.clone() } />
            <div class="box">
                <Link<Route> to={Route::Repayments { account_id: ctx.props().account_id.clone() }}>
                <h3 class="subtitle is-3">
                    <span class="icon-text">
                        <span class="icon"><i class="fas fa-exchange"></i></span>
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
            </>
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
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let repayments = ctx.props().repayments.borrow();
        let mut sorted = repayments.keys().cloned().collect::<Vec<_>>();
        sorted.sort_by(|a, b| {
            repayments
                .get(b)
                .unwrap()
                .date
                .partial_cmp(&repayments.get(a).unwrap().date)
                .unwrap()
        });
        let len = repayments.len();

        html! {
            <div class="is-relative block">
                if ctx.props().loading {
                    <div class="loading-overlay">
                        <Loading />
                    </div>
                }
                {
                    if len > 0 {
                        let map = |id: &Uuid| {
                            let repayment = repayments.get(id).unwrap();
                            html! {
                                <tr>
                                    <td class="is-vcentered is-hidden-touch">{ &repayment.date }</td>
                                    <td class="is-vcentered"><UserName account_id={ ctx.props().account_id.clone() } users={ ctx.props().users.clone() } id={ repayment.payer_id } /></td>
                                    <td class="is-vcentered is-hidden-touch">{ "a remboursé" }</td>
                                    <td class="is-vcentered"><Amount amount={ repayment.amount as i64 } /></td>
                                    <td class="is-vcentered is-hidden-touch">{ "à" }</td>
                                    <td class="is-vcentered"><UserName account_id={ ctx.props().account_id.clone() } users={ ctx.props().users.clone() } id={ repayment.beneficiary_id } /></td>
                                    <td class="is-vcentered">
                                        <Link<Route> to={Route::EditRepayment { account_id: ctx.props().account_id.clone(), repayment_id: { repayment.id } }}>
                                            <a aria-label="Éditer" class="button is-primary" href="">
                                                <i class="fas fa-pencil fa-fw"></i>
                                            </a>
                                        </Link<Route>>
                                        <DeleteRepayment account_id={ repayment.account_id.clone() } id={ repayment.id.clone() } />
                                    </td>
                                </tr>
                            }
                        };
                        html! {
                            <table class="table is-fullwidth is-striped is-hoverable">
                                <thead>
                                    <tr>
                                        <th class="is-hidden-touch">{ "Date" }</th>
                                        <th>{ "Payeur" }</th>
                                        <th class="is-hidden-touch"></th>
                                        <th>{ "Montant" }</th>
                                        <th class="is-hidden-touch"></th>
                                        <th>{ "Beneficiaire" }</th>
                                        <th>{ "Actions" }</th>
                                    </tr>
                                </thead>
                            <tbody>
                            {
                                match ctx.props().limit {
                                    Some(limit) => sorted.iter().take(limit).map(map).collect::<Html>(),
                                    None => sorted.iter().map(map).collect::<Html>(),
                                }
                            }
                            </tbody>
                            </table>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="buttons">
                    if let Some(limit) = ctx.props().limit {
                        if len > limit {
                            <Link<Route> to={Route::Repayments { account_id: ctx.props().account_id.clone() }} classes="button is-light">
                                { format!("Voir les {} autres", len - limit) }
                            </Link<Route>>
                        }
                    }
                    <Link<Route> to={Route::CreateRepayment { account_id: ctx.props().account_id.clone() }} classes="button is-primary">
                        <span class="icon">
                            <i class="fas fa-plus-circle" />
                        </span>
                        <span>{ "Nouveau remboursement" }</span>
                    </Link<Route>>
                </div>
            </div>
        }
    }
}

#[derive(Debug, Clone)]
struct DefaultRepayment {
    payer_id: Option<Uuid>,
    beneficiary_id: Option<Uuid>,
    amount: i32,
    date: NaiveDate,
}

impl From<rmmt::Repayment> for DefaultRepayment {
    fn from(repayment: rmmt::Repayment) -> Self {
        Self {
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
    agent: Box<dyn Bridge<AccountAgent>>,
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
                Request::put(&format!(
                    "/api/account/{}/repayments/{}",
                    ctx.props().account_id,
                    id
                ))
                .json(&repayment)
                .unwrap()
            }
            None => {
                let repayment = rmmt::NewRepayment {
                    account_id: account_id.into(),
                    amount,
                    payer_id,
                    beneficiary_id,
                    date,
                };
                debug!("Create repayment: {:?}", repayment);
                Request::post(&format!(
                    "/api/account/{}/repayments",
                    ctx.props().account_id
                ))
                .json(&repayment)
                .unwrap()
            }
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
        let mut agent = AccountAgent::bridge(ctx.link().callback(EditRepaymentMsg::AccountMsg));

        agent.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));
        if let Some(repayment_id) = ctx.props().repayment_id.clone() {
            agent.send(AccountMsg::LoadRepayment {
                account_id: ctx.props().account_id.clone(),
                repayment_id,
            });
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
            agent,
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
                    }
                    Ok(balancing) => balancing.into(),
                },
                None => Default::default(),
            },
        };

        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            EditRepaymentMsg::Submit
        });

        let history = ctx.link().history().unwrap();
        let previous = Callback::once(move |event: MouseEvent| {
            event.prevent_default();
            history.go(-1)
        });

        let delete_error = ctx.link().callback(|_| EditRepaymentMsg::ClearError);

        html! {
            <>
            <AccountTitle id={ ctx.props().account_id.clone() } account={ self.account.clone() } />
            <div class="box">
                if let Some(repayment_id) = ctx.props().repayment_id.clone() {
                    <Link<Route> to={Route::EditRepayment { account_id: ctx.props().account_id.clone(), repayment_id }}>
                        <h3 class="subtitle is-3">
                            <span class="icon-text">
                                <span class="icon"><i class="fas fa-exchange"></i></span>
                                <span>{ "Remboursement" }</span>
                            </span>
                        </h3>
                    </Link<Route>>
                } else {
                    <Link<Route> to={Route::CreateRepayment { account_id: ctx.props().account_id.clone() }}>
                        <h3 class="subtitle is-3">
                            <span class="icon-text">
                                <span class="icon"><i class="fas fa-exchange"></i></span>
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
                                            <i class="fas fa-user"></i>
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
                                            <i class="fas fa-user"></i>
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
                        <div class="control buttons">
                            <button type="button" class="button is-light" onclick={ previous }>
                                { "Annuler" }
                            </button>
                            <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                                if ctx.props().repayment_id.is_some() {
                                    <span class="icon">
                                        <i class="fas fa-save" />
                                    </span>
                                    <span>{ "Enregistrer" }</span>
                                } else {
                                    <span class="icon">
                                        <i class="fas fa-plus" />
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
            </>
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
                <i class="fas fa-trash fa-fw"></i>
            </button>
        }
    }
}
