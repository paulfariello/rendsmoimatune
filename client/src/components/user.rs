use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    account::{AccountTitle, AccountMsg},
    expenditure::ExpendituresList,
    repayment::RepaymentsList,
    utils::{Amount, Loading},
};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct UsernameProps {
    pub account_id: String,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
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
    let users = &*users.borrow();
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
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
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
}

pub enum UserMsg {
    AccountMsg(AccountMsg),
    Edit,
    Edited { user: rmmt::User },
}

pub struct User {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    expenditures:
        Option<Rc<RefCell<HashMap<Uuid, (rmmt::Expenditure, HashMap<Uuid, rmmt::Debt>)>>>>,
    repayments: Option<Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    balance: Option<Rc<RefCell<rmmt::Balance>>>,
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
            account: None,
            expenditures: None,
            repayments: None,
            balance: None,
            users: None,
            editing: false,
            input_name: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateAccount => {
                    true
                }
                AccountMsg::UpdateUsers => {
                    true
                }
                AccountMsg::UpdateExpenditures => {
                    true
                }
                AccountMsg::UpdateRepayments => {
                    true
                }
                AccountMsg::UpdateBalance => {
                    true
                }
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
        let edit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            UserMsg::Edit
        });

        let user_name = match self.users.as_ref() {
            Some(users) => users
                .borrow()
                .get(&ctx.props().user_id)
                .unwrap()
                .name
                .clone(),
            None => String::new(),
        };

        let payed_expenditures = match self.expenditures.as_ref() {
            Some(expenditures) => {
                let expenditures = expenditures.borrow();
                Some(Rc::new(RefCell::new(
                    expenditures
                        .clone()
                        .into_iter()
                        .filter(|e| e.1 .0.payer_id == ctx.props().user_id)
                        .collect::<HashMap<_, _>>(),
                )))
            }
            None => None,
        };

        let concerned_expenditures = match self.expenditures.as_ref() {
            Some(expenditures) => {
                let expenditures = expenditures.borrow();
                Some(Rc::new(RefCell::new(
                    expenditures
                        .clone()
                        .into_iter()
                        .filter(|e| e.1 .1.iter().any(|d| d.1.debtor_id == ctx.props().user_id))
                        .collect::<HashMap<_, _>>(),
                )))
            }
            None => None,
        };

        let concerned_repayments = match self.repayments.as_ref() {
            Some(repayments) => {
                let repayments = repayments.borrow();
                Some(Rc::new(RefCell::new(
                    repayments
                        .clone()
                        .into_iter()
                        .filter(|e| {
                            e.1.payer_id == ctx.props().user_id
                                || e.1.beneficiary_id == ctx.props().user_id
                        })
                        .collect::<HashMap<_, _>>(),
                )))
            }
            None => None,
        };

        let concerned_balance = match self.balance.as_ref() {
            Some(balance) => {
                let balance = balance.borrow();
                Some(
                    balance
                        .user_balances
                        .clone()
                        .into_iter()
                        .filter(|e| e.user_id == ctx.props().user_id)
                        .next()
                        .unwrap(),
                )
            }
            None => None,
        };

        let (total_payed, total_debt) = match (&self.expenditures, &self.repayments) {
            (Some(expenditures), Some(repayments)) => {
                let user_id = &ctx.props().user_id;
                let mut total_payed = 0i64;
                let mut total_debt = 0i64;

                for (expenditure, debts) in expenditures.borrow().values() {
                    if let Some(debt) = debts.get(user_id) {
                        let share_sum: i32 = debts.values().map(|d| d.share).sum();
                        total_debt += (expenditure.amount as f64
                            * (debt.share as f64 / share_sum as f64))
                            as i64;
                    }

                    if &expenditure.payer_id == user_id {
                        total_payed += expenditure.amount as i64;
                    }
                }
                info!("expenditures debt {}", total_debt);
                info!("payed expenditures {}", total_payed);

                for repayment in repayments.borrow().values() {
                    if &repayment.payer_id == user_id {
                        total_payed += repayment.amount as i64;
                    } else if &repayment.beneficiary_id == user_id {
                        total_debt += repayment.amount as i64;
                    }
                }
                info!("repayment debt {}", total_debt);
                info!("payed repayments {}", total_payed);

                (Some(total_payed), Some(total_debt))
            }
            _ => (None, None),
        };

        html! {
            <>
            <AccountTitle id={ ctx.props().account_id.clone() } account={ self.account.clone() } />
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-user"></i></span>
                            if let Some(users) = self.users.clone() {
                                <span><UserName account_id={ ctx.props().account_id.clone() } { users } id={ ctx.props().user_id.clone() }/></span>
                            } else {
                                <Loading />
                            }
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
                                    if let Some(balance) = concerned_balance.clone() {
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
                                    } else {
                                        <Loading />
                                    }
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
                                if let Some(expenditures) = payed_expenditures.as_ref() {
                                    { expenditures.borrow().len().to_string() }
                                    { " dépenses payées" }
                                } else {
                                    { "Dépenses payées" }
                                }
                            </span>
                        </h3>
                        if let (Some(users), Some(expenditures)) = (self.users.clone(), payed_expenditures) {
                            <ExpendituresList account_id={ ctx.props().account_id.clone() } { expenditures } { users } />
                        } else {
                            <Loading />
                        }
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                if let Some(expenditures) = concerned_expenditures.as_ref() {
                                    { expenditures.borrow().len().to_string() }
                                    { " dépenses concernées" }
                                } else {
                                    { "Dépenses concernées" }
                                }
                            </span>
                        </h3>
                        if let (Some(users), Some(expenditures)) = (self.users.clone(), concerned_expenditures) {
                            <ExpendituresList account_id={ ctx.props().account_id.clone() } { expenditures } { users } />
                        } else {
                            <Loading />
                        }
                    </div>
                </div>
            </div>

            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <h3 class="subtitle is-3">
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>
                                if let Some(repayments) = concerned_repayments.as_ref() {
                                    { repayments.borrow().len().to_string() }
                                    { " remboursements" }
                                } else {
                                    { "Remboursements" }
                                }
                            </span>
                        </h3>
                        if let (Some(users), Some(repayments)) = (self.users.clone(), concerned_repayments) {
                            <RepaymentsList account_id={ ctx.props().account_id.clone() } { repayments } { users } />
                        } else {
                            <Loading />
                        }
                    </div>
                </div>
            </div>
            </>
        }
    }
}
