use log;
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
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
    pub id: Uuid,
    #[prop_or_else(|| "primary".to_string())]
    pub color: String,
}

#[function_component(UserName)]
pub fn user_name(UsernameProps { id, color }: &UsernameProps) -> Html {
    let text_color = format!("has-text-{}", color);
    let account_ctx = use_context::<AccountCtx>().unwrap();
    if let Some(user) = account_ctx.users.get(&id) {
        html! {
            <Link<Route> to={Route::User { account_id: account_ctx.id.clone(), user_id: id.clone() } } classes={ classes!(text_color) }>
                { &user.name }
            </Link<Route>>
        }
    } else {
        log::error!("Unknown user {}", id);
        html! {}
    }
}

pub enum CreateUserMsg {
    Submit,
    Created(rmmt::User),
    Error(utils::Error),
}

pub struct CreateUser {
    creating: bool,
    input_name: NodeRef,
    error: Option<utils::Error>,
}

impl CreateUser {
    fn create_user(&mut self, ctx: &Context<Self>) {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account_id: UniqId = account_ctx.id.clone().try_into().unwrap();
        let user = rmmt::NewUser {
            account_id: account_id.into(),
            name,
        };
        let url = format!("/api/account/{}/users", account_ctx.id);
        ctx.link().send_future(async move {
            let created_user: Result<rmmt::User, _> = utils::post(&url, &user).await;
            match created_user {
                Ok(user) => CreateUserMsg::Created(user),
                Err(error) => CreateUserMsg::Error(error),
            }
        });
    }

    fn reload_users(&mut self, ctx: &Context<Self>) {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        account_ctx.dispatch(AccountAction::BumpVersion);
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
    type Properties = ();

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
                self.clear();
                self.reload_users(ctx);
                true
            }
            CreateUserMsg::Error(error) => {
                self.creating = false;
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("Rerender CreateUser creating = {}", self.creating);
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
    pub user_id: Uuid,
}

pub enum UserMsg {
    Edit,
    Edited,
    Error(utils::Error),
}

pub struct BaseUser {
    editing: bool,
    input_name: NodeRef,
    error: Option<utils::Error>,
}

impl BaseUser {
    fn edit_user(&mut self, ctx: &Context<Self>) {
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();
        self.editing = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account_id: UniqId = account_ctx.id.clone().try_into().unwrap();
        let user = rmmt::User {
            id: ctx.props().user_id.clone(),
            account_id: account_id.into(),
            name,
        };
        let url = format!("/api/account/{}/users/{}", account_ctx.id, user.id);
        ctx.link().send_future(async move {
            let edited_user: Result<rmmt::User, _> = utils::put(&url, &user).await;
            match edited_user {
                Ok(_) => UserMsg::Edited,
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
            UserMsg::Edited  => {
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
        let (account_ctx, _) = ctx.link().context::<AccountCtx>(Callback::noop()).unwrap();

        let edit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            UserMsg::Edit
        });

        let balance = account_ctx
            .balance
            .balancing_remaining
            .iter()
            .find(|balance| balance.user_id == ctx.props().user_id)
            .unwrap();

        html! {
            <>
            <AccountTitle />
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-user"></i></span>
                            <span><UserName id={ ctx.props().user_id.clone() }/></span>
                        </h3>
                        if let Some(error) = self.error.as_ref() {
                            <FetchError error={ format!("{:?}", error) } />
                        }
                        <form onsubmit={ edit }>
                            <div class="field has-addons">
                                <div class={classes!("control", self.editing.then(|| "is-loading"))}>
                                    <input ref={self.input_name.clone()} type="text" class="input is-primary" name="name" required=true placeholder={ account_ctx.users.get(&ctx.props().user_id).unwrap().name.clone() } />
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
                                                <progress class="progress is-large is-danger is-revert" value={ balance.debit.to_string() } max={ balance.debit.to_string() }>
                                                    <Amount amount={ balance.debit } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ balance.debit } /></p>
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
                                                <progress class="progress is-large is-success" value={ balance.credit.to_string() } max={ balance.credit.to_string() }>
                                                    <Amount amount={ balance.credit } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ balance.credit } /></p>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="is-vcentered">
                                        if balance.amount < 0 {
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-danger is-revert" value={ balance.amount.abs().to_string() } max={ balance.amount.abs().to_string() }>
                                                    <Amount amount={ balance.amount } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ balance.amount } /></p>
                                            </div>
                                        }
                                        </td>
                                        <td class="is-vcentered has-text-centered">{ "Total" }</td>
                                        <td class="is-vcentered">
                                        if balance.amount > 0 {
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-success" value={ balance.amount.abs().to_string() } max={ balance.amount.abs().to_string() }>
                                                    <Amount amount={ balance.amount } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ balance.amount } /></p>
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
                        <ExpendituresList payer_id={ Some(ctx.props().user_id.clone()) } />
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
                        <ExpendituresList debtor_id={ Some(ctx.props().user_id.clone()) } />
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
                            <RepaymentsList user_id={ Some(ctx.props().user_id.clone()) } />
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
    Ok(html! {<BaseUser user_id={ props.user_id.clone() } />})
}
