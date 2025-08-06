use std::collections::HashMap;

use anyhow::Result;
use chrono::naive::NaiveDate;
use chrono::Local;
use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps, UseFutureHandle};
use yew_router::prelude::*;

use crate::components::{
    account::AccountTitle,
    ctx::{AccountAction, AccountCtx},
    user::UserName,
    utils::{Amount, FetchError},
};
use crate::utils;
use crate::Route;

#[function_component(Repayments)]
pub fn repayments() -> HtmlResult {
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
                <Link<Route> to={Route::Repayments { account_id: account_ctx.id.clone() }}>
                    <span class="icon-text">
                        <span class="icon"><i class="fas fa-exchange"></i></span>
                        <span>{ "Remboursements" }</span>
                    </span>
                </Link<Route>>
            </h3>
            <Suspense fallback={utils::loading()}>
                <RepaymentsList />
            </Suspense>
        </div>
        </>
    })
}

#[derive(Properties, PartialEq)]
pub struct RepaymentsListProps {
    #[prop_or_default]
    pub limit: Option<usize>,
    #[prop_or_default]
    pub user_id: Option<Uuid>,
    #[prop_or_default]
    pub buttons: bool,
}

#[function_component(RepaymentsList)]
pub fn repayments_list(props: &RepaymentsListProps) -> HtmlResult {
    let account_ctx = use_context::<AccountCtx>().unwrap();
    log::debug!("Rendering repayments list version: {}", account_ctx.version);

    let repayments_url = format!("/api/account/{}/repayments", account_ctx.id);
    let query: Vec<(&str, String)> = props
        .user_id
        .iter()
        .map(|id| ("user_id", id.hyphenated().to_string()))
        .collect();
    let repayments: UseFutureHandle<Result<Vec<rmmt::Repayment>, _>> =
        use_future_with_deps(|_| async move { utils::get_with_query(&repayments_url, query).await }, account_ctx.version)?;
    let mut repayments: Vec<rmmt::Repayment> = match *repayments {
        Ok(ref res) => res.iter().cloned().collect(), // TODO avoid clone
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    repayments.sort_by(|a, b| b.date.cmp(&a.date));
    let len = repayments.len();

    Ok(html! {
        <div class="is-relative block">
            {
                if len > 0 {
                    let map = |repayment: &rmmt::Repayment| {
                        html! {
                            <tr>
                                <td class="is-vcentered is-hidden-touch">{ &repayment.date }</td>
                                <td class="is-vcentered"><UserName id={ repayment.payer_id } /></td>
                                <td class="is-vcentered is-hidden-touch">{ "a remboursé" }</td>
                                <td class="is-vcentered"><Amount amount={ repayment.amount as i64 } /></td>
                                <td class="is-vcentered is-hidden-touch">{ "à" }</td>
                                <td class="is-vcentered"><UserName id={ repayment.beneficiary_id } /></td>
                                <td class="is-vcentered">
                                    <Link<Route> to={Route::EditRepayment { account_id: account_ctx.id.clone(), repayment_id: { repayment.id } }}>
                                        <a aria-label="Éditer" class="button is-primary" href="">
                                            <i class="fas fa-pencil fa-fw"></i>
                                        </a>
                                    </Link<Route>>
                                    <DeleteRepayment id={ repayment.id.clone() } />
                                </td>
                            </tr>
                        }
                    };
                    html! {
                        <table class="table is-fullwidth is-striped is-hoverable">
                            <thead>
                                <tr>
                                    <th class="is-hidden-touch">{ "Date" }</th>
                                    <th class="is-hidden-touch">{ "Payeur" }</th>
                                    <th class="is-hidden-desktop">{ "De" }</th>
                                    <th class="is-hidden-touch"></th>
                                    <th class="is-hidden-touch">{ "Montant" }</th>
                                    <th class="is-hidden-desktop"></th>
                                    <th class="is-hidden-touch"></th>
                                    <th class="is-hidden-touch">{ "Beneficiaire" }</th>
                                    <th class="is-hidden-desktop">{ "À" }</th>
                                    <th class="is-hidden-touch">{ "Actions" }</th>
                                    <th class="is-hidden-desktop"></th>
                                </tr>
                            </thead>
                        <tbody>
                        {
                            match props.limit {
                                Some(limit) => repayments.iter().take(limit).map(map).collect::<Html>(),
                                None => repayments.iter().map(map).collect::<Html>(),
                            }
                        }
                        </tbody>
                        </table>
                    }
                } else {
                    html! {
                        <div class="notification is-info is-light">
                            { "Aucun remboursement" }
                        </div>
                    }
                }
            }

            <div class="buttons">
                if let Some(limit) = props.limit {
                    if len > limit {
                        <Link<Route> to={Route::Repayments { account_id: account_ctx.id.clone() }} classes="button is-light">
                            { format!("Voir les {} autres", len - limit) }
                        </Link<Route>>
                    }
                }
                if props.buttons {
                    <Link<Route> to={Route::CreateRepayment { account_id: account_ctx.id.clone() }} classes="button is-primary">
                        <span class="icon">
                            <i class="fas fa-plus-circle" />
                        </span>
                        <span>{ "Nouveau remboursement" }</span>
                    </Link<Route>>
                }
            </div>
        </div>
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultRepayment {
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
            date: Local::now().naive_local().into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BaseEditRepaymentProps {
    #[prop_or_default]
    pub repayment_id: Option<Uuid>,
    pub repayment: Option<rmmt::Repayment>,
    pub default: Option<DefaultRepayment>,
}

pub enum EditRepaymentMsg {
    Submit,
    Edited { repayment: rmmt::Repayment },
    Error(String),
    ClearError,
    UpdateAccountCtx(AccountCtx),
}

pub struct BaseEditRepayment {
    account_ctx: AccountCtx,
    _ctx_listener: ContextHandle<AccountCtx>,
    select_payer: NodeRef,
    input_amount: NodeRef,
    select_beneficiary: NodeRef,
    input_date: NodeRef,
    creating: bool,
    error: Option<String>,
}

impl BaseEditRepayment {
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

        let account_id: UniqId = self.account_ctx.id.clone().try_into().unwrap();
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
                    self.account_ctx.id, id
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
                Request::post(&format!("/api/account/{}/repayments", self.account_ctx.id))
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

        let today = Local::now();
        let input_date = self.input_date.cast::<web_sys::HtmlInputElement>().unwrap();
        input_date.set_value(&format!("{}", today.format("%Y-%m-%d")));
    }
}

impl Component for BaseEditRepayment {
    type Message = EditRepaymentMsg;
    type Properties = BaseEditRepaymentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (account_ctx, ctx_listener) = ctx.link().context::<AccountCtx>(ctx.link().callback(EditRepaymentMsg::UpdateAccountCtx)).unwrap();
        Self {
            account_ctx,
            _ctx_listener: ctx_listener,
            select_payer: NodeRef::default(),
            input_amount: NodeRef::default(),
            select_beneficiary: NodeRef::default(),
            input_date: NodeRef::default(),
            creating: false,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
                self.clear();

                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Account {
                    account_id: self.account_ctx.id.clone(),
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
            EditRepaymentMsg::UpdateAccountCtx(account_ctx) => {
                self.account_ctx = account_ctx;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let default = match ctx.props().default.as_ref() {
            Some(default) => default.clone(),
            None => match ctx.link().location() {
                Some(location) if !location.query_str().is_empty() => match location.query::<rmmt::Balancing>() {
                    Err(err) => {
                        error!("Invalid query: {} \"{}\"", err, location.query_str());
                        Default::default()
                    }
                    Ok(balancing) => balancing.into(),
                },
                _ => {
                    let mut default: DefaultRepayment = Default::default();
                    let mut users = self.account_ctx.users.keys();
                    default.payer_id = users.next().copied();
                    default.beneficiary_id = users.next().copied();
                    default
                }
            },
        };

        let onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            EditRepaymentMsg::Submit
        });

        let navigator = ctx.link().navigator().unwrap();
        let previous = Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            navigator.back()
        });

        let delete_error = ctx.link().callback(|_| EditRepaymentMsg::ClearError);

        html! {
            <>
            <AccountTitle />
            <div class="box">
                if let Some(repayment_id) = ctx.props().repayment_id.clone() {
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::EditRepayment { account_id: self.account_ctx.id.clone(), repayment_id }}>
                            <span class="icon-text">
                                <span class="icon"><i class="fas fa-exchange"></i></span>
                                <span>{ "Remboursement" }</span>
                            </span>
                        </Link<Route>>
                    </h3>
                } else {
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::CreateRepayment { account_id: self.account_ctx.id.clone() }}>
                            <span class="icon-text">
                                <span class="icon"><i class="fas fa-exchange"></i></span>
                                <span>{ "Nouveau remboursement" }</span>
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
                    <div class="field is-horizontal">
                        <div class="field-body">
                            <div class="field">
                                <p class="control is-expanded has-icons-left">
                                    <div class="select is-fullwidth is-primary">
                                        <select ref={ self.select_payer.clone() } required=true>
                                        {
                                            self.account_ctx.users.iter().map(|(_, user)| html! {
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
                                            self.account_ctx.users.iter().map(|(_, user)| html! {
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
            </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct EditExistingRepaymentProps {
    #[prop_or_default]
    pub repayment_id: Uuid,
}

#[function_component(EditExistingRepayment)]
pub fn edit_existing_repayment(props: &EditExistingRepaymentProps) -> HtmlResult {
    let account_ctx = use_context::<AccountCtx>().unwrap();
    let repayment_url = format!(
        "/api/account/{}/repayments/{}",
        account_ctx.id, props.repayment_id
    );
    let repayment: UseFutureHandle<Result<rmmt::Repayment, _>> =
        use_future(|| async move { utils::get(&repayment_url).await })?;
    let repayment: &rmmt::Repayment = match *repayment {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    Ok(
        html! {<BaseEditRepayment repayment_id={ props.repayment_id } repayment={ Some(repayment.clone()) } />},
    )
}

#[derive(Properties, PartialEq)]
pub struct EditRepaymentProps {
    #[prop_or_default]
    pub repayment_id: Option<Uuid>,
}

#[function_component(EditRepayment)]
pub fn edit_repayment_with_account_and_users(props: &EditRepaymentProps) -> HtmlResult {
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

    if let Some(repayment_id) = props.repayment_id {
        Ok(html! {<EditExistingRepayment repayment_id={ repayment_id } />})
    } else {
        Ok(html! {<BaseEditRepayment repayment_id={ props.repayment_id } />})
    }
}

#[derive(Properties, PartialEq)]
pub struct DeleteRepaymentProps {
    pub id: Uuid,
}

pub enum DeleteRepaymentMsg {
    Delete,
    Deleted,
    Error(String),
}

struct DeleteRepayment {
    deleting: bool,
    error: Option<String>,
}

impl DeleteRepayment {
    fn delete_repayment(&mut self, ctx: &Context<Self>) {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        self.deleting = true;

        let url = format!(
            "/api/account/{}/repayments/{}",
            account_ctx.id,
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
                let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
                self.deleting = false;
                account_ctx.dispatch(AccountAction::BumpVersion);
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
