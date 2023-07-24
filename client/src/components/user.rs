use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{Context as _, Error, Result};
use log;
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use yew_router::prelude::*;

use crate::components::{
    account::AccountTitle,
    ctx::{AccountAction, AccountCtx},
    expenditure::ExpendituresList,
    repayment::RepaymentsList,
    utils::{Amount, FetchError},
};
use crate::utils;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct UsernameProps {
    pub account_id: String,
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
    pub id: Uuid,
    #[prop_or_else(|| "primary".to_string())]
    pub color: String,
}

#[function_component(UserName)]
pub fn user_name(
    UsernameProps {
        account_id,
        users,
        id,
        color,
    }: &UsernameProps,
) -> Html {
    let text_color = format!("has-text-{}", color);
    if let Some(user) = users.get(&id) {
        html! {
            <Link<Route> to={Route::User { account_id: account_id.clone(), user_id: id.clone() } } classes={ classes!(text_color) }>
                { &user.name }
            </Link<Route>>
        }
    } else {
        log::error!("Unknown user {}", id);
        html! {}
    }
}

#[derive(PartialEq, Properties)]
pub struct CreateUserProps {
    pub account_id: String,
}

pub enum CreateUserMsg {
    Submit,
    Created(rmmt::User),
    Reloaded {
        users: HashMap<Uuid, rmmt::User>,
        balance: rmmt::Balance,
    },
    AccountCtxUpdated(AccountCtx),
    Error(Error),
}

pub struct CreateUser {
    creating: bool,
    input_name: NodeRef,
    error: Option<Error>,
}

impl CreateUser {
    fn create_user(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account_id: UniqId = ctx.props().account_id.clone().try_into().unwrap();
        let user = rmmt::NewUser {
            account_id: account_id.into(),
            name,
        };
        let url = format!("/api/account/{}/users", ctx.props().account_id);
        ctx.link().send_future(async move {
            let created_user: Result<rmmt::User> = utils::post(&url, &user).await;
            match created_user {
                Ok(user) => CreateUserMsg::Created(user),
                Err(error) => CreateUserMsg::Error(error),
            }
        });
    }

    fn reload_users(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let users_url = format!("/api/account/{}/users", ctx.props().account_id);
        let balance_url = format!("/api/account/{}/balance", ctx.props().account_id);
        ctx.link().send_future(async move {
            // TODO parallelize
            let users: Result<Vec<rmmt::User>> = utils::get(&users_url).await;
            let balance: Result<rmmt::Balance> = utils::get(&balance_url).await;
            match (users, balance) {
                (Ok(users), Ok(balance)) => CreateUserMsg::Reloaded {
                    users: users
                        .iter()
                        .cloned()
                        .map(|u| (u.id.clone(), u))
                        .collect::<HashMap<_, _>>(),
                    balance,
                },
                (Ok(_), Err(error)) => CreateUserMsg::Error(error),
                (Err(error), Ok(_)) => CreateUserMsg::Error(error),
                (Err(error), Err(_)) => CreateUserMsg::Error(error),
            }
        });
    }

    fn clear(&mut self) {
        self.creating = false;
        self.error = None;
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for CreateUser {
    type Message = CreateUserMsg;
    type Properties = CreateUserProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            creating: false,
            input_name: NodeRef::default(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateUserMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.create_user(ctx);
                    true
                }
            }
            CreateUserMsg::Created(user) => {
                log::info!("Created user: {}", user.name);
                self.reload_users(ctx);
                false
            }
            CreateUserMsg::Reloaded { users, balance } => {
                let (account_ctx, _) = ctx
                    .link()
                    .context::<AccountCtx>(ctx.link().callback(CreateUserMsg::AccountCtxUpdated))
                    .unwrap();
                log::info!("Reloaded users: {:?}", users);
                account_ctx.dispatch(AccountAction::UpdateUsers(users));
                log::info!("Reloaded balance: {:?}", balance);
                account_ctx.dispatch(AccountAction::UpdateBalance(balance));
                self.clear();
                true
            }
            CreateUserMsg::Error(error) => {
                self.creating = false;
                self.error = Some(error);
                true
            }
            CreateUserMsg::AccountCtxUpdated(_) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            CreateUserMsg::Submit
        });

        html! {
            <>
            if let Some(error) = self.error.as_ref() {
                <FetchError error={ format!("{:?}", error) } />
            }
            <form {onsubmit}>
                <div class="field has-addons">
                    <div class={classes!("control", self.creating.then(|| "is-loading"))}>
                        <input ref={self.input_name.clone()} type="text" class="input is-primary" name="name" required=true placeholder="François" />
                    </div>
                    <div class="control">
                        <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                            <span class="icon">
                                <i class="fas fa-user-plus" />
                            </span>
                            <span>{ "Ajouter" }</span>
                        </button>
                    </div>
                </div>
            </form>
            </>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct BaseUserProps {
    pub account_id: String,
    pub account: rmmt::Account,
    pub user_id: Uuid,
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
    pub balance: rmmt::UserBalance,
}

pub enum UserMsg {
    Edit,
    Edited { user: rmmt::User },
    Error(Error),
}

pub struct BaseUser {
    editing: bool,
    input_name: NodeRef,
    error: Option<Error>,
}

impl BaseUser {
    fn edit_user(&mut self, ctx: &Context<Self>) {
        self.editing = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account_id: UniqId = ctx.props().account_id.clone().try_into().unwrap();
        let user = rmmt::User {
            id: ctx.props().user_id.clone(),
            account_id: account_id.into(),
            name,
        };
        let url = format!("/api/account/{}/users/{}", ctx.props().account_id, user.id);
        ctx.link().send_future(async move {
            let edited_user: Result<rmmt::User> = utils::put(&url, &user).await;
            match edited_user {
                Ok(user) => UserMsg::Edited { user },
                Err(error) => UserMsg::Error(error),
            }
        });
    }

    fn clear(&mut self) {
        self.editing = false;
        self.error = None;
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for BaseUser {
    type Message = UserMsg;
    type Properties = BaseUserProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            editing: false,
            input_name: NodeRef::default(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserMsg::Edit => {
                if self.editing {
                    false
                } else {
                    self.edit_user(ctx);
                    true
                }
            }
            UserMsg::Edited { user: _ } => {
                self.clear();
                true
            }
            UserMsg::Error(error) => {
                self.editing = false;
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let edit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            UserMsg::Edit
        });

        html! {
            <>
            <AccountTitle id={ ctx.props().account_id.clone() } name={ ctx.props().account.name.clone() } />
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-user"></i></span>
                            <span><UserName account_id={ ctx.props().account_id.clone() } users={ ctx.props().users.clone() } id={ ctx.props().user_id.clone() }/></span>
                        </h3>
                        if let Some(error) = self.error.as_ref() {
                            <FetchError error={ format!("{:?}", error) } />
                        }
                        <form onsubmit={ edit }>
                            <div class="field has-addons">
                                <div class={classes!("control", self.editing.then(|| "is-loading"))}>
                                    <input ref={self.input_name.clone()} type="text" class="input is-primary" name="name" required=true placeholder={ ctx.props().users.get(&ctx.props().user_id).unwrap().name.clone() } />
                                </div>
                                <div class="control">
                                    <button type="submit" class={classes!("button", "is-primary", self.editing.then(|| "is-loading"))}>
                                        <span class="icon">
                                            <i class="fas fa-pen" />
                                        </span>
                                        <span>{ "Éditer" }</span>
                                    </button>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-balance-scale"></i></span>
                            <span>{ "Balance" }</span>
                        </h3>
                            <table class="table is-fullwidth is-striped is-hoverable">
                                <tbody>
                                    <tr>
                                        <td class="is-vcentered">
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-danger is-revert" value={ ctx.props().balance.debit.to_string() } max={ ctx.props().balance.debit.to_string() }>
                                                    <Amount amount={ ctx.props().balance.debit } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ ctx.props().balance.debit } /></p>
                                            </div>
                                        </td>
                                        <td class="is-vcentered has-text-centered">{ "Dette" }</td>
                                        <td class="is-vcentered">
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="is-vcentered">
                                        </td>
                                        <td class="is-vcentered has-text-centered">{ "Avance" }</td>
                                        <td class="is-vcentered">
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-success" value={ ctx.props().balance.credit.to_string() } max={ ctx.props().balance.credit.to_string() }>
                                                    <Amount amount={ ctx.props().balance.credit } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ ctx.props().balance.credit } /></p>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="is-vcentered">
                                        if ctx.props().balance.amount < 0 {
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-danger is-revert" value={ ctx.props().balance.amount.abs().to_string() } max={ ctx.props().balance.amount.abs().to_string() }>
                                                    <Amount amount={ ctx.props().balance.amount } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ ctx.props().balance.amount } /></p>
                                            </div>
                                        }
                                        </td>
                                        <td class="is-vcentered has-text-centered">{ "Total" }</td>
                                        <td class="is-vcentered">
                                        if ctx.props().balance.amount > 0 {
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-success" value={ ctx.props().balance.amount.abs().to_string() } max={ ctx.props().balance.amount.abs().to_string() }>
                                                    <Amount amount={ ctx.props().balance.amount } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ ctx.props().balance.amount } /></p>
                                            </div>
                                        }
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
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                { "Dépenses payées" }
                            </span>
                        </h3>
                        <ExpendituresList account_id={ ctx.props().account_id.clone() } payer_id={ Some(ctx.props().user_id.clone()) } users={ ctx.props().users.clone() } />
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                { "Dépenses concernées" }
                            </span>
                        </h3>
                        <ExpendituresList account_id={ ctx.props().account_id.clone() } debtor_id={ Some(ctx.props().user_id.clone()) } users={ ctx.props().users.clone() } />
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                { "Remboursements" }
                            </span>
                        </h3>
                        <Suspense fallback={utils::loading()}>
                            <RepaymentsList account_id={ ctx.props().account_id.clone() } user_id={ Some(ctx.props().user_id.clone()) } users={ ctx.props().users.clone() } />
                        </Suspense>
                    </div>
                </div>
            </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct UserProps {
    pub user_id: Uuid,
}

#[function_component(User)]
pub fn user(props: &UserProps) -> HtmlResult {
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

    let balance_url = format!("/api/account/{}/balances/{}", account_ctx.id, props.user_id);
    let balance: UseFutureHandle<Result<rmmt::UserBalance, _>> =
        use_future(|| async move { utils::get(&balance_url).await })?;
    let balance: &rmmt::UserBalance = match *balance {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    Ok(
        html! {<BaseUser account_id={ account_ctx.id.clone() } account={ account.clone() } user_id={ props.user_id.clone() } users={ account_ctx.users.clone() } balance={ balance.clone() } />},
    )
}
