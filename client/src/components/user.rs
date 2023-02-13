use std::collections::HashMap;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    account::{AccountMsg, AccountTitle},
    expenditure::ExpendituresList,
    repayment::RepaymentsList,
    utils::Amount,
};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct UsernameProps {
    pub account_id: String,
    pub users: HashMap<Uuid, rmmt::User>,
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
        error!("Unknown user {}", id);
        html! {}
    }
}

#[derive(PartialEq, Properties)]
pub struct CreateUserProps {
    pub account_id: String,
}

pub enum CreateUserMsg {
    Submit,
    Created { user: rmmt::User },
}

pub struct CreateUser {
    creating: bool,
    input_name: NodeRef,
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
            let created_user: rmmt::User = Request::post(&url)
                .json(&user)
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            CreateUserMsg::Created { user: created_user }
        });
    }

    fn clear(&mut self) {
        self.creating = false;
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
            CreateUserMsg::Created { user } => {
                info!("Created user: {}", user.name);
                self.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            CreateUserMsg::Submit
        });

        html! {
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
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct UserProps {
    pub account_id: String,
    pub user_id: Uuid,
    pub account: rmmt::Account,
    pub expenditures: HashMap<Uuid, (rmmt::Expenditure, HashMap<Uuid, rmmt::Debt>)>,
    pub repayments: HashMap<Uuid, rmmt::Repayment>,
    pub users: HashMap<Uuid, rmmt::User>,
    pub balance: rmmt::Balance,
}

pub enum UserMsg {
    AccountMsg(AccountMsg),
    Edit,
    Edited { user: rmmt::User },
}

pub struct User {
    editing: bool,
    input_name: NodeRef,
}

impl User {
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
            let edited_user: rmmt::User = Request::put(&url)
                .json(&user)
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            UserMsg::Edited { user: edited_user }
        });
    }

    fn clear(&mut self) {
        self.editing = false;
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for User {
    type Message = UserMsg;
    type Properties = UserProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            editing: false,
            input_name: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateAccount => true,
                AccountMsg::UpdateUsers => true,
                AccountMsg::UpdateExpenditures => true,
                AccountMsg::UpdateRepayments => true,
                AccountMsg::UpdateBalance => true,
                _ => false,
            },
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let edit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            UserMsg::Edit
        });

        let user_name = ctx
            .props()
            .users
            .get(&ctx.props().user_id)
            .unwrap()
            .name
            .clone();

        let payed_expenditures = ctx
            .props()
            .expenditures
            .clone()
            .into_iter()
            .filter(|e| e.1 .0.payer_id == ctx.props().user_id)
            .collect::<HashMap<_, _>>();

        let concerned_expenditures = ctx
            .props()
            .expenditures
            .clone()
            .into_iter()
            .filter(|e| e.1 .1.iter().any(|d| d.1.debtor_id == ctx.props().user_id))
            .collect::<HashMap<_, _>>();

        let concerned_repayments = ctx
            .props()
            .repayments
            .clone()
            .into_iter()
            .filter(|e| {
                e.1.payer_id == ctx.props().user_id || e.1.beneficiary_id == ctx.props().user_id
            })
            .collect::<HashMap<_, _>>();

        let concerned_balance = ctx
            .props()
            .balance
            .user_balances
            .clone()
            .into_iter()
            .filter(|e| e.user_id == ctx.props().user_id)
            .next()
            .unwrap();

        let (total_payed, total_debt) = {
            let user_id = &ctx.props().user_id;
            let mut total_payed = 0i64;
            let mut total_debt = 0i64;

            for (expenditure, debts) in ctx.props().expenditures.values() {
                if let Some(debt) = debts.get(user_id) {
                    let share_sum: i32 = debts.values().map(|d| d.share).sum();
                    total_debt +=
                        (expenditure.amount as f64 * (debt.share as f64 / share_sum as f64)) as i64;
                }

                if &expenditure.payer_id == user_id {
                    total_payed += expenditure.amount as i64;
                }
            }
            info!("expenditures debt {}", total_debt);
            info!("payed expenditures {}", total_payed);

            for repayment in ctx.props().repayments.values() {
                if &repayment.payer_id == user_id {
                    total_payed += repayment.amount as i64;
                } else if &repayment.beneficiary_id == user_id {
                    total_debt += repayment.amount as i64;
                }
            }
            info!("repayment debt {}", total_debt);
            info!("payed repayments {}", total_payed);

            (Some(total_payed), Some(total_debt))
        };

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
                        <form onsubmit={ edit }>
                            <div class="field has-addons">
                                <div class={classes!("control", self.editing.then(|| "is-loading"))}>
                                    <input ref={self.input_name.clone()} type="text" class="input is-primary" name="name" required=true placeholder={ user_name } />
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
                                    if let (Some(total_payed), Some(total_debt)) = (total_payed, total_debt) {
                                        <tr>
                                            <td class="is-vcentered">
                                                <div class="progress-wrapper">
                                                    <progress class="progress is-large is-danger is-revert" value={ total_debt.to_string() } max={ total_debt.to_string() }>
                                                        <Amount amount={ total_debt } />
                                                    </progress>
                                                    <p class="progress-value has-text-white"><Amount amount={ total_debt } /></p>
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
                                                    <progress class="progress is-large is-success" value={ total_payed.to_string() } max={ total_payed.to_string() }>
                                                        <Amount amount={ total_payed } />
                                                    </progress>
                                                    <p class="progress-value has-text-white"><Amount amount={ total_payed } /></p>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                    <tr>
                                        <td class="is-vcentered">
                                        if concerned_balance.amount < 0 {
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-danger is-revert" value={ concerned_balance.amount.abs().to_string() } max={ concerned_balance.amount.abs().to_string() }>
                                                    <Amount amount={ concerned_balance.amount } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ concerned_balance.amount } /></p>
                                            </div>
                                        }
                                        </td>
                                        <td class="is-vcentered has-text-centered">{ "Total" }</td>
                                        <td class="is-vcentered">
                                        if concerned_balance.amount > 0 {
                                            <div class="progress-wrapper">
                                                <progress class="progress is-large is-success" value={ concerned_balance.amount.abs().to_string() } max={ concerned_balance.amount.abs().to_string() }>
                                                    <Amount amount={ concerned_balance.amount } />
                                                </progress>
                                                <p class="progress-value has-text-white"><Amount amount={ concerned_balance.amount } /></p>
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
                                { payed_expenditures.len().to_string() }
                                { " dépenses payées" }
                            </span>
                        </h3>
                        <ExpendituresList account_id={ ctx.props().account_id.clone() } expenditures={ ctx.props().expenditures.clone() } users={ ctx.props().users.clone() } />
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                { concerned_expenditures.len().to_string() }
                                { " dépenses concernées" }
                            </span>
                        </h3>
                        <ExpendituresList account_id={ ctx.props().account_id.clone() } expenditures={ ctx.props().expenditures.clone() } users={ ctx.props().users.clone() } />
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                { concerned_repayments.len().to_string() }
                                { " remboursements" }
                            </span>
                        </h3>
                        <RepaymentsList account_id={ ctx.props().account_id.clone() } repayments={ ctx.props().repayments.clone() } users={ ctx.props().users.clone() } />
                    </div>
                </div>
            </div>
            </>
        }
    }
}
