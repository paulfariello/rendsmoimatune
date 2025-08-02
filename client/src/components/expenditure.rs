use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use chrono::naive::NaiveDate;
use chrono::Local;
use gloo_net::http::Request;
use itertools::Itertools;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use yew_router::prelude::*;

use crate::components::{
    account::AccountTitle,
    ctx::{AccountAction, AccountCtx},
    user::UserName,
    utils::{Amount, FetchError},
};
use crate::{utils, Route};

#[function_component(Expenditures)]
pub fn expenditures() -> HtmlResult {
    let account_ctx = use_context::<AccountCtx>().unwrap();

    let account_url = format!("/api/account/{}", account_ctx.id);
    let account: UseFutureHandle<Result<rmmt::Account, _>> =
        use_future(|| async move { utils::get(&account_url).await })?;
    let account: &rmmt::Account = match *account {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };
    account_ctx.dispatch(AccountAction::UpdateName(account.name.clone()));

    let users_url = format!("/api/account/{}/users", account_ctx.id);
    let users: UseFutureHandle<Result<Vec<rmmt::User>, _>> =
        use_future(|| async move { utils::get(&users_url).await })?;
    let users: HashMap<Uuid, rmmt::User> = match *users {
        Ok(ref res) => res.iter().cloned().map(|u| (u.id.clone(), u)).collect(),
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };
    account_ctx.dispatch(AccountAction::UpdateUsers(users));

    Ok(html! {
        <>
        <AccountTitle />
        <div class="box">
            <h3 class="subtitle is-3">
                <Link<Route> to={Route::Expenditures { account_id: account_ctx.id.clone() }}>
                    <span class="icon-text">
                        <span class="icon"><i class="fas fa-credit-card"></i></span>
                        <span>{ "Dépenses" }</span>
                    </span>
                </Link<Route>>
            </h3>
            <Suspense fallback={utils::loading()}>
                <ExpendituresList />
            </Suspense>
        </div>
        </>
    })
}

#[derive(Properties, PartialEq)]
pub struct ExpendituresListProps {
    #[prop_or_default]
    pub limit: Option<usize>,
    #[prop_or_default]
    pub payer_id: Option<Uuid>,
    #[prop_or_default]
    pub debtor_id: Option<Uuid>,
    #[prop_or_default]
    pub buttons: bool,
}

#[function_component(ExpendituresList)]
pub fn expenditures_list(props: &ExpendituresListProps) -> HtmlResult {
    let account_ctx = use_context::<AccountCtx>().unwrap();

    let expenditures_url = format!("/api/account/{}/expenditures", account_ctx.id);
    let mut payer_query: Vec<(&str, String)> = props
        .payer_id
        .iter()
        .map(|id| ("payer_id", id.hyphenated().to_string()))
        .collect();
    let mut debtor_query: Vec<(&str, String)> = props
        .debtor_id
        .iter()
        .map(|id| ("debtor_id", id.hyphenated().to_string()))
        .collect();

    let mut query = vec![];
    query.append(&mut payer_query);
    query.append(&mut debtor_query);

    let expenditures: UseFutureHandle<Result<Vec<(rmmt::Expenditure, Vec<rmmt::Debt>)>, _>> =
        use_future(|| async move { utils::get_with_query(&expenditures_url, query).await })?;
    let mut expenditures: Vec<(rmmt::Expenditure, Vec<rmmt::Debt>)> = match *expenditures {
        Ok(ref res) => res.iter().cloned().collect(), // TODO avoid clone
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    expenditures.sort_by(|a, b| b.0.date.cmp(&a.0.date));
    let len = expenditures.len();

    Ok(html! {
        <div class="is-relative block">
            {
                if len > 0 {
                    let map = |(expenditure, debts): &(rmmt::Expenditure, Vec<rmmt::Debt>)| {
                        html! {
                            <tr key={ expenditure.id.to_string() }>
                                <td class="is-vcentered is-hidden-touch">{ &expenditure.date }</td>
                                <td class="is-vcentered">{ &expenditure.name }</td>
                                <td class="is-vcentered"><Amount amount={ expenditure.amount as i64} /></td>
                                <td class="is-vcentered is-hidden-touch"><UserName id={ expenditure.payer_id }/></td>
                                <td class="is-vcentered is-hidden-touch">
                                    <Debtors debts={ debts.clone() } users={ account_ctx.users.clone() } />
                                </td>
                                <td class="is-vcentered">
                                    <Link<Route> to={Route::EditExpenditure { account_id: account_ctx.id.clone(), expenditure_id: { expenditure.id } }} classes="button is-primary">
                                        <i class="fas fa-pencil fa-fw"></i>
                                    </Link<Route>>
                                    <DeleteExpenditure id={ expenditure.id.clone() } />
                                </td>
                            </tr>
                        }
                    };
                    html! {
                        <table class="table is-fullwidth is-striped is-hoverable">
                            <thead>
                                <tr>
                                    <th class="is-hidden-touch">{ "Date" }</th>
                                    <th>{ "Nom" }</th>
                                    <th>{ "Montant" }</th>
                                    <th class="is-hidden-touch">{ "Payeur" }</th>
                                    <th class="is-hidden-touch">{ "Participants" }</th>
                                    <th>{ "Actions" }</th>
                                </tr>
                            </thead>
                        <tbody>
                        {
                            match props.limit {
                                Some(limit) => expenditures.iter().take(limit).map(map).collect::<Html>(),
                                None => expenditures.iter().map(map).collect::<Html>(),
                            }
                        }
                        </tbody>
                        </table>
                    }
                } else {
                    html! {
                        <div class="notification is-info is-light">
                            { "Aucune dépense" }
                        </div>
                    }
                }
            }
            <div class="buttons">
                if let Some(limit) = props.limit {
                    if len > limit {
                        <Link<Route> to={ Route::Expenditures { account_id: account_ctx.id.clone() } } classes="button is-light">
                            { format!("Voir les {} autres", len - limit) }
                        </Link<Route>>
                    }
                }
                if props.buttons {
                    <Link<Route> to={Route::CreateExpenditure { account_id: account_ctx.id.clone() }} classes="button is-primary">
                        <span class="icon">
                            <i class="fas fa-plus-circle" />
                        </span>
                        <span>{ "Nouvelle dépense" }</span>
                    </Link<Route>>
                }
            </div>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct BaseEditExpenditureProps {
    #[prop_or_default]
    pub expenditure_id: Option<Uuid>,
    pub expenditure: Option<rmmt::Expenditure>,
    pub debts: Option<Vec<rmmt::Debt>>,
}

pub enum EditExpenditureMsg {
    Submit,
    Edited {
        expenditure: rmmt::Expenditure,
        debts: Vec<rmmt::Debt>,
    },
    Error(String),
    ClearError,
}

pub struct BaseEditExpenditure {
    input_name: NodeRef,
    input_date: NodeRef,
    input_amount: NodeRef,
    select_payer: NodeRef,
    debtors_checkbox: HashMap<Uuid, NodeRef>,
    debtors_input_share: HashMap<Uuid, NodeRef>,
    creating: bool,
    error: Option<String>,
}

impl BaseEditExpenditure {
    fn save_expenditure(&mut self, ctx: &Context<Self>) {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
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

        let account_id: UniqId = account_ctx.id.clone().try_into().unwrap();

        let mut debtors = Vec::new();
        for (id, user) in account_ctx.users.iter() {
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
                    account_ctx.id, id
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
                Request::post(&format!("/api/account/{}/expenditures", account_ctx.id))
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

        let today = Local::now();
        let input_date = self.input_date.cast::<web_sys::HtmlInputElement>().unwrap();
        input_date.set_value(&format!("{}", today.format("%Y-%m-%d")));
    }
}

impl Component for BaseEditExpenditure {
    type Message = EditExpenditureMsg;
    type Properties = BaseEditExpenditureProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        let debtors_checkbox = account_ctx
            .users
            .iter()
            .map(|(id, _)| (id.clone(), NodeRef::default()))
            .collect();
        let debtors_input_share = account_ctx
            .users
            .iter()
            .map(|(id, _)| (id.clone(), NodeRef::default()))
            .collect();

        Self {
            input_name: NodeRef::default(),
            input_date: NodeRef::default(),
            input_amount: NodeRef::default(),
            select_payer: NodeRef::default(),
            debtors_checkbox,
            debtors_input_share,
            creating: false,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        match msg {
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
                self.clear();

                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Account {
                    account_id: account_ctx.id.clone(),
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
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        let onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            EditExpenditureMsg::Submit
        });

        let navigator = ctx.link().navigator().unwrap();
        let previous = Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            navigator.back()
        });

        let delete_error = ctx.link().callback(|_| EditExpenditureMsg::ClearError);

        let debts: Option<HashMap<Uuid, rmmt::Debt>> =
            ctx.props().debts.as_ref().and_then(|debts| {
                Some(
                    debts
                        .iter()
                        .cloned()
                        .map(|d| (d.debtor_id.clone(), d))
                        .collect(),
                )
            });

        html! {
            <>
            <AccountTitle />
            <div class="box">
                if let Some(expenditure_id) = ctx.props().expenditure_id.clone() {
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::EditExpenditure { account_id: account_ctx.id.clone(), expenditure_id }}>
                            <span class="icon-text">
                                <span class="icon"><i class="fas fa-exchange"></i></span>
                                <span>{ "Dépense" }</span>
                            </span>
                        </Link<Route>>
                    </h3>
                } else {
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::CreateExpenditure { account_id: account_ctx.id.clone() }}>
                            <span class="icon-text">
                                <span class="icon"><i class="fas fa-exchange"></i></span>
                                <span>{ "Nouvelle dépense" }</span>
                            </span>
                        </Link<Route>>
                    </h3>
                }
                if let Some(error) = self.error.as_ref() {
                    <div class="notification is-danger">
                      <button class="delete" onclick={delete_error}></button>
                      { error }
                    </div>
                }
                <form {onsubmit}>
                    <div class="field">
                        <label class="label">{ "Nom" }</label>
                        <div class="control">
                            <input ref={ self.input_name.clone() } class="input is-primary" type="text" placeholder="Baguette de pain" required=true value={ ctx.props().expenditure.as_ref().map(|e| e.name.clone()) }/>
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Montant" }</label>
                        <div class="field has-addons">
                            <div class="control is-expanded">
                                <input ref={ self.input_amount.clone() } type="number" min="0" step="0.01" class="input is-primary" required=true placeholder="montant" value={ ctx.props().expenditure.as_ref().map(|e| (e.amount as f64 / 100f64).to_string()) }/>
                            </div>
                            <div class="control">
                                <p class="button is-static">{ "€" }</p>
                            </div>
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Date" }</label>
                        <div class="control">
                            <input ref={self.input_date.clone()} type="date" class="input is-primary" required=true value={ format!("{}", ctx.props().expenditure.as_ref().map(|e| e.date).unwrap_or(Local::now().naive_local().into()).format("%Y-%m-%d")) } />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Payeur" }</label>
                        <p class="control is-expanded has-icons-left">
                            <div class="select is-fullwidth is-primary">
                                <select ref={ self.select_payer.clone() } required=true>
                                {
                                    account_ctx.users.iter().map(|(_, user)| html! {
                                        <option value={ user.id.to_string() } selected={ ctx.props().expenditure.as_ref().map(|e| e.payer_id) == Some(user.id) }>{ &user.name }</option>
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
                        <label class="label">{ "Bénéficiaires" }</label>
                        {
                            account_ctx.users.iter().map(|(id, user)| html! {
                                <DebtorInput name={ user.name.clone() } state_ref={ self.debtors_checkbox.get(&id).clone().unwrap() } share_ref={ self.debtors_input_share.get(&id).clone().unwrap() } debt={ debts.as_ref().and_then(|debts| debts.get(&id).cloned()) }/>
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="control buttons">
                        <button type="button" class="button is-light" onclick={ previous }>
                            { "Annuler" }
                        </button>
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
            </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct EditExistingExpenditureProps {
    #[prop_or_default]
    pub expenditure_id: Uuid,
}

#[function_component(EditExistingExpenditure)]
pub fn edit_existing_expenditure(props: &EditExistingExpenditureProps) -> HtmlResult {
    let account_ctx = use_context::<AccountCtx>().unwrap();
    let expenditure_url = format!(
        "/api/account/{}/expenditures/{}",
        account_ctx.id, props.expenditure_id
    );
    let expenditure: UseFutureHandle<Result<(rmmt::Expenditure, Vec<rmmt::Debt>), _>> =
        use_future(|| async move { utils::get(&expenditure_url).await })?;
    let (expenditure, debts): &(rmmt::Expenditure, Vec<rmmt::Debt>) = match *expenditure {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    Ok(
        html! {<BaseEditExpenditure expenditure_id={ props.expenditure_id } expenditure={ Some(expenditure.clone()) } debts={ Some(debts.clone()) } />},
    )
}

#[derive(Properties, PartialEq)]
pub struct EditExpenditureProps {
    #[prop_or_default]
    pub expenditure_id: Option<Uuid>,
}

#[function_component(EditExpenditure)]
pub fn edit_expenditure_with_account_and_users(props: &EditExpenditureProps) -> HtmlResult {
    let account_ctx = use_context::<AccountCtx>().unwrap();

    let account_url = format!("/api/account/{}", account_ctx.id);
    let account: UseFutureHandle<Result<rmmt::Account, _>> =
        use_future(|| async move { utils::get(&account_url).await })?;
    let account: &rmmt::Account = match *account {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };
    account_ctx.dispatch(AccountAction::UpdateName(account.name.clone()));

    let users_url = format!("/api/account/{}/users", account_ctx.id);
    let users: UseFutureHandle<Result<Vec<rmmt::User>, _>> =
        use_future(|| async move { utils::get(&users_url).await })?;
    let users: HashMap<Uuid, rmmt::User> = match *users {
        Ok(ref res) => res.iter().cloned().map(|u| (u.id.clone(), u)).collect(),
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };
    account_ctx.dispatch(AccountAction::UpdateUsers(users));

    if let Some(expenditure_id) = props.expenditure_id {
        Ok(html! {<EditExistingExpenditure expenditure_id={ expenditure_id } />})
    } else {
        Ok(html! {<BaseEditExpenditure expenditure_id={ props.expenditure_id } />})
    }
}

#[derive(Properties, PartialEq)]
pub struct DebtorInputProps {
    pub name: String,
    pub state_ref: NodeRef,
    pub share_ref: NodeRef,
    #[prop_or_default]
    pub debt: Option<rmmt::Debt>,
}

pub enum DebtorInputMsg {
    Switch,
}

struct DebtorInput {
    checked: bool,
}

impl Component for DebtorInput {
    type Message = DebtorInputMsg;
    type Properties = DebtorInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let checked = match &ctx.props().debt {
            Some(debt) => debt.share > 0,
            None => false,
        };
        Self { checked }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DebtorInputMsg::Switch => {
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
        let onclick = ctx.link().callback(|_| DebtorInputMsg::Switch);

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
                        <input ref={ ctx.props().share_ref.clone() } type="number" min="0" step="1" class="input is-primary" value={ ctx.props().debt.as_ref().map(|d| d.share.to_string()).or(Some(1.to_string())) } />
                    </div>
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DeleteExpenditureProps {
    pub id: Uuid,
}

pub enum DeleteExpenditureMsg {
    Delete,
    Deleted,
    Error(String),
}

struct DeleteExpenditure {
    deleting: bool,
    error: Option<String>,
}

impl DeleteExpenditure {
    fn delete_expenditure(&mut self, ctx: &Context<Self>) {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        self.deleting = true;

        let url = format!(
            "/api/account/{}/expenditures/{}",
            account_ctx.id,
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
                <i class="fas fa-trash fa-fw"></i>
            </button>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DebtorsProps {
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
    pub debts: Vec<rmmt::Debt>,
}

#[function_component(Debtors)]
fn debtors(props: &DebtorsProps) -> Html {
    let debts: HashMap<Uuid, rmmt::Debt> = props
        .debts
        .iter()
        .cloned()
        .map(|debt| (debt.id.clone(), debt))
        .collect();
    let debts_count = props.debts.len();
    let users_count = props.users.len();
    html! {
        if debts_count == users_count {
            { "Tous" }
        } else if debts_count == 0 {
            { "Personne" }
        } else if debts_count > users_count / 2 {
            { "Tous sauf " }
            { props.users.iter().filter(|(id, _)| !debts.contains_key(id)).map(|(_, u)| &u.name).join(", ") }
        } else {
            { props.users.iter().filter(|(id, _)| debts.contains_key(id)).map(|(_, u)| &u.name).join(", ") }
        }
    }
}
