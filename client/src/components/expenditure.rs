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
pub struct ExpendituresProps {
    pub account_id: String,
}

pub struct Expenditures {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    expenditures: Option<Rc<RefCell<HashMap<Uuid, rmmt::Expenditure>>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Expenditures {
    type Message = AccountMsg;
    type Properties = ExpendituresProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));

        Self {
            account: None,
            expenditures: None,
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
            AccountMsg::UpdateExpenditures(expenditures) => {
                self.expenditures = Some(expenditures);
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
                                    <i class="fas fa-bank"/>
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
                        <Link<Route> to={Route::Expenditures { account_id: ctx.props().account_id.clone() }}>
                            <h3 class="subtitle is-3">
                                <span class="icon-text">
                                    <span class="icon"><i class="fas fa-credit-card"></i></span>
                                    <span>{ "Dépenses" }</span>
                                </span>
                            </h3>
                        </Link<Route>>
                        if let (Some(users), Some(expenditures)) = (self.users.clone(), self.expenditures.clone()) {
                            <ExpendituresList account_id={ ctx.props().account_id.clone() } { expenditures } { users } loading=false />
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
pub struct ExpendituresListProps {
    pub account_id: String,
    pub expenditures: Rc<RefCell<HashMap<Uuid, rmmt::Expenditure>>>,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub limit: Option<usize>,
    pub loading: bool,
}

pub struct ExpendituresList;

impl Component for ExpendituresList {
    type Message = ();
    type Properties = ExpendituresListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let expenditures = &*ctx.props().expenditures.borrow();
        let len = expenditures.len();

        if len > 0 {
            let map = |(_, expenditure): (&Uuid, &rmmt::Expenditure)| {
                html! {
                    <tr key={ expenditure.id.to_string() }>
                        <td class="is-vcentered">{ &expenditure.date }</td>
                        <td class="is-vcentered">{ &expenditure.name }</td>
                        <td class="is-vcentered"><Amount amount={ expenditure.amount as i64} /></td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ expenditure.payer_id }/></td>
                        <td class="is-vcentered">{ "todo" }</td>
                        <td class="is-vcentered">
                            <Link<Route> to={Route::EditExpenditure { account_id: ctx.props().account_id.clone(), expenditure_id: { expenditure.id } }}>
                                <a aria-label="Éditer" class="button is-primary" href="">
                                    <i class="fas fa-pencil"></i>
                                </a>
                            </Link<Route>>
                            <DeleteExpenditure account_id={ expenditure.account_id.clone() } id={ expenditure.id.clone() } />
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
                                <th>{ "Nom" }</th>
                                <th>{ "Montant" }</th>
                                <th>{ "Payeur" }</th>
                                <th>{ "Participants" }</th>
                                <th>{ "Actions" }</th>
                            </tr>
                        </thead>
                    <tbody>
                    {
                        match ctx.props().limit {
                            None => expenditures.iter().map(map).collect::<Html>(),
                            Some(limit) => expenditures.iter().take(limit).map(map).collect::<Html>(),
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
pub struct EditExpenditureProps {
    pub account_id: String,
    #[prop_or_default]
    pub expenditure_id: Option<Uuid>,
}

pub enum EditExpenditureMsg {
    AccountMsg(AccountMsg),
    Submit,
    Edited {
        expenditure: rmmt::Expenditure,
        debts: Vec<rmmt::Debt>,
    },
    Error(String),
    ClearError,
}

pub struct EditExpenditure {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    expenditure: Option<rmmt::Expenditure>,
    input_name: NodeRef,
    input_date: NodeRef,
    input_amount: NodeRef,
    select_payer: NodeRef,
    debtors_checkbox: HashMap<Uuid, NodeRef>,
    debtors_input_share: HashMap<Uuid, NodeRef>,
    creating: bool,
    error: Option<String>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
    agent: Dispatcher<AccountAgent>,
}

impl EditExpenditure {
    fn save_expenditure(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let input_date = self.input_date.cast::<web_sys::HtmlInputElement>().unwrap();
        let date = NaiveDate::parse_from_str(&input_date.value(), "%Y-%m-%d").unwrap();

        let input_amount = self
            .input_amount
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        let amount = input_amount.value().parse::<f32>().unwrap();
        let amount = (amount * 100f32).round() as i32;

        let select_payer = self
            .select_payer
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        let payer_id = Uuid::parse_str(&select_payer.value()).unwrap();

        let account_id: UniqId = ctx.props().account_id.clone().try_into().unwrap();

        let mut debtors = Vec::new();
        for (id, user) in (&*self.users.as_ref().unwrap().borrow()).iter() {
            let checkbox = self.debtors_checkbox.get(id).unwrap();
            let enabled = checkbox
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .checked();
            if enabled {
                let input_share = self.debtors_input_share.get(id).unwrap();
                let share = input_share
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<i32>()
                    .unwrap();
                info!("{}: {}", user.name, share);
                debtors.push((id.clone(), share));
            }
        }

        let req = match ctx.props().expenditure_id {
            Some(id) => {
                let expenditure = rmmt::Expenditure {
                    id: id.clone(),
                    account_id: account_id.into(),
                    name,
                    date,
                    amount,
                    payer_id,
                };
                Request::put(&format!(
                    "/api/account/{}/expenditures/{}",
                    ctx.props().account_id,
                    id
                ))
                .json(&(expenditure, debtors))
                .unwrap()
            }
            None => {
                let expenditure = rmmt::NewExpenditure {
                    account_id: account_id.into(),
                    name,
                    date,
                    amount,
                    payer_id,
                };
                Request::post(&format!(
                    "/api/account/{}/expenditures",
                    ctx.props().account_id
                ))
                .json(&(expenditure, debtors))
                .unwrap()
            }
        };
        ctx.link().send_future(async move {
            let resp = req.send().await;

            let resp = match resp {
                Err(err) => return EditExpenditureMsg::Error(format!("{}", err)),
                Ok(resp) => resp,
            };

            if !resp.ok() {
                return EditExpenditureMsg::Error(format!(
                    "{}: {}",
                    resp.status(),
                    resp.status_text()
                ));
            }

            let resp = resp.json::<(rmmt::Expenditure, Vec<rmmt::Debt>)>().await;

            if let Err(err) = resp {
                return EditExpenditureMsg::Error(format!("{}", err));
            }

            let (expenditure, debts) = resp.unwrap();
            EditExpenditureMsg::Edited { expenditure, debts }
        });
    }

    fn clear(&mut self) {
        self.creating = false;
        self.error = None;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");

        let input_amount = self
            .input_amount
            .cast::<web_sys::HtmlInputElement>()
            .unwrap();
        input_amount.set_value("");

        let today = Local::today();
        let input_date = self.input_date.cast::<web_sys::HtmlInputElement>().unwrap();
        input_date.set_value(&format!("{}", today.format("%Y-%m-%d")));
    }
}

impl Component for EditExpenditure {
    type Message = EditExpenditureMsg;
    type Properties = EditExpenditureProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge =
            AccountAgent::bridge(ctx.link().callback(EditExpenditureMsg::AccountMsg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::LoadAccount(ctx.props().account_id.clone()));
        if let Some(expenditure_id) = ctx.props().expenditure_id.clone() {
            dispatcher.send(AccountMsg::LoadExpenditure {
                account_id: ctx.props().account_id.clone(),
                expenditure_id,
            });
        }

        Self {
            account: None,
            users: None,
            expenditure: None,
            input_name: NodeRef::default(),
            input_date: NodeRef::default(),
            input_amount: NodeRef::default(),
            select_payer: NodeRef::default(),
            debtors_checkbox: HashMap::new(),
            debtors_input_share: HashMap::new(),
            creating: false,
            error: None,
            _account_bridge: account_bridge,
            agent: AccountAgent::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditExpenditureMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateAccount(account) => {
                    self.account = Some(account);
                    true
                }
                AccountMsg::UpdateUsers(users) => {
                    self.users = Some(users);
                    self.debtors_checkbox = (&*self.users.as_ref().unwrap().borrow())
                        .iter()
                        .map(|(id, _)| (id.clone(), NodeRef::default()))
                        .collect();
                    self.debtors_input_share = (&*self.users.as_ref().unwrap().borrow())
                        .iter()
                        .map(|(id, _)| (id.clone(), NodeRef::default()))
                        .collect();
                    true
                }
                AccountMsg::UpdateExpenditure(expenditure) => {
                    if Some(expenditure.id) == ctx.props().expenditure_id {
                        self.expenditure = Some(expenditure);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            EditExpenditureMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.error = None;
                    self.save_expenditure(ctx);
                    true
                }
            }
            EditExpenditureMsg::Edited { expenditure, debts } => {
                info!(
                    "Edited expenditure: {:?} with debts: {:?}",
                    expenditure, debts
                );
                self.agent.send(AccountMsg::ChangedExpenditures);
                self.clear();

                let history = ctx.link().history().unwrap();
                history.push(Route::Account {
                    account_id: ctx.props().account_id.clone(),
                });

                false
            }
            EditExpenditureMsg::Error(error) => {
                error!("Cannot create expenditure: {}", error);
                self.creating = false;
                self.error = Some(error);
                true
            }
            EditExpenditureMsg::ClearError => {
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            EditExpenditureMsg::Submit
        });

        let delete_error = ctx.link().callback(|_| EditExpenditureMsg::ClearError);

        html! {
            <div class="columns">
                <div class="column">
                    <AccountTitle id={ ctx.props().account_id.clone() } account={ self.account.clone() } />
                    <div class="box">
                        if let Some(expenditure_id) = ctx.props().expenditure_id.clone() {
                            <Link<Route> to={Route::EditExpenditure { account_id: ctx.props().account_id.clone(), expenditure_id }}>
                                <h3 class="subtitle is-3">
                                    <span class="icon-text">
                                        <span class="icon"><i class="fas fa-exchange"></i></span>
                                        <span>{ "Dépense" }</span>
                                    </span>
                                </h3>
                            </Link<Route>>
                        } else {
                            <Link<Route> to={Route::CreateExpenditure { account_id: ctx.props().account_id.clone() }}>
                                <h3 class="subtitle is-3">
                                    <span class="icon-text">
                                        <span class="icon"><i class="fas fa-exchange"></i></span>
                                        <span>{ "Nouvelle dépense" }</span>
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
                                <div class="field">
                                    <label class="label">{ "Nom" }</label>
                                    <div class="control">
                                        <input ref={ self.input_name.clone() } class="input is-primary" type="text" placeholder="Baguette de pain" required=true value={ self.expenditure.as_ref().map(|e| e.name.clone()) }/>
                                    </div>
                                </div>

                                <div class="field">
                                    <label class="label">{ "Montant" }</label>
                                    <div class="field has-addons">
                                        <div class="control is-expanded">
                                            <input ref={ self.input_amount.clone() } type="number" min="0" step="0.01" class="input is-primary" required=true placeholder="montant" value={ self.expenditure.as_ref().map(|e| (e.amount as f64 / 100f64).to_string()) }/>
                                        </div>
                                        <div class="control">
                                            <p class="button is-static">{ "€" }</p>
                                        </div>
                                    </div>
                                </div>

                                <div class="field">
                                    <label class="label">{ "Date" }</label>
                                    <div class="control">
                                        <input ref={self.input_date.clone()} type="date" class="input is-primary" required=true value={ format!("{}", self.expenditure.as_ref().map(|e| e.date).unwrap_or(Local::today().naive_local()).format("%Y-%m-%d")) } />
                                    </div>
                                </div>

                                <div class="field">
                                    <label class="label">{ "Payeur" }</label>
                                    <p class="control is-expanded has-icons-left">
                                        <div class="select is-fullwidth is-primary">
                                            <select ref={ self.select_payer.clone() } required=true>
                                            {
                                                (&*users.borrow()).iter().map(|(_, user)| html! {
                                                    <option value={ user.id.to_string() } selected={ self.expenditure.as_ref().map(|e| e.payer_id) == Some(user.id) }>{ &user.name }</option>
                                                }).collect::<Html>()
                                            }
                                            </select>
                                        </div>
                                        <span class="icon is-small is-left">
                                            <i class="fas fa-user"></i>
                                        </span>
                                    </p>
                                </div>

                                {
                                    (&*users.borrow()).iter().map(|(id, user)| html! {
                                        <Debtor name={ user.name.clone() } state_ref={ self.debtors_checkbox.get(&id).clone().unwrap() } share_ref={ self.debtors_input_share.get(&id).clone().unwrap() } />
                                    }).collect::<Html>()
                                }
                                <div class="control">
                                    <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                                        if ctx.props().expenditure_id.is_some() {
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
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DebtorProps {
    pub name: String,
    pub state_ref: NodeRef,
    pub share_ref: NodeRef,
}

pub enum DebtorMsg {
    Switch,
}

struct Debtor {
    checked: bool,
}

impl Component for Debtor {
    type Message = DebtorMsg;
    type Properties = DebtorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { checked: true }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DebtorMsg::Switch => {
                self.checked = !self.checked;
                let input_state = ctx
                    .props()
                    .state_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap();
                input_state.set_checked(self.checked);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| DebtorMsg::Switch);

        html! {
            <div class="field has-addons">
                <div class="control">
                    <label class={ classes!("is-checkbox", match self.checked { true => "is-primary", false => "is-light" }) }>
                        <input ref={ ctx.props().state_ref.clone() } type="checkbox" checked={ self.checked } { onclick } />
                        <span class="icon checkmark">
                            <i class="fas fa-check"></i>
                        </span>
                        <span>{ &ctx.props().name }</span>
                    </label>
                </div>
                if self.checked {
                    <div class="control">
                        <input ref={ ctx.props().share_ref.clone() } type="number" min="0" step="1" class="input is-primary" value="1" />
                    </div>
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DeleteExpenditureProps {
    pub account_id: Uuid,
    pub id: Uuid,
}

pub enum DeleteExpenditureMsg {
    Delete,
    Deleted,
    Error(String),
}

struct DeleteExpenditure {
    deleting: bool,
    agent: Dispatcher<AccountAgent>,
    error: Option<String>,
}

impl DeleteExpenditure {
    fn delete_expenditure(&mut self, ctx: &Context<Self>) {
        self.deleting = true;

        let url = format!(
            "/api/account/{}/expenditures/{}",
            UniqId::from(ctx.props().account_id),
            ctx.props().id
        );
        ctx.link().send_future(async move {
            let resp = Request::delete(&url).send().await;

            let resp = match resp {
                Err(err) => return DeleteExpenditureMsg::Error(format!("{}", err)),
                Ok(resp) => resp,
            };

            if !resp.ok() {
                return DeleteExpenditureMsg::Error(format!(
                    "{}: {}",
                    resp.status(),
                    resp.status_text()
                ));
            }

            DeleteExpenditureMsg::Deleted
        });
    }
}

impl Component for DeleteExpenditure {
    type Message = DeleteExpenditureMsg;
    type Properties = DeleteExpenditureProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            deleting: false,
            error: None,
            agent: AccountAgent::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeleteExpenditureMsg::Delete => {
                if self.deleting {
                    false
                } else {
                    self.error = None;
                    self.delete_expenditure(ctx);
                    true
                }
            }
            DeleteExpenditureMsg::Deleted => {
                self.deleting = false;
                self.agent.send(AccountMsg::ChangedExpenditures);
                true
            }
            DeleteExpenditureMsg::Error(error) => {
                error!("Cannot delete expenditure: {}", error);
                self.deleting = false;
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| DeleteExpenditureMsg::Delete);

        html! {
            <button aria-label="Supprimer" class={ classes!("button", "is-danger", self.deleting.then(|| "is-loading")) } { onclick }>
                <i class="fas fa-trash"></i>
            </button>
        }
    }
}
