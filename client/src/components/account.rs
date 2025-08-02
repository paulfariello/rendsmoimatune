use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{Context as _, Error, Result};
use bounce::query::use_query;
use log;
use rmmt;
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use yew_router::prelude::*;

use crate::components::{
    balance::{BalanceList, BalancingList},
    ctx::AccountQuery,
    expenditure::ExpendituresList,
    repayment::RepaymentsList,
    user::CreateUser,
    utils::FetchError,
};
use crate::utils;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct AccountProps {
    pub id: String,
}

#[function_component(Account)]
pub fn account(props: &AccountProps) -> HtmlResult {
    log::debug!("Rerendering account");

    let account = use_query::<AccountQuery>(Rc::new(props.id.clone()))?;
    let account = match account.as_ref() {
        Ok(account) => account,
        // TODO check if we can pass utils::Error to FetchError
        Err(error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    let users_url = format!("/api/account/{}/users", props.id);
    let users: UseFutureHandle<Result<Vec<rmmt::User>, _>> =
        use_future(|| async move { utils::get(&users_url).await })?;
    let users: HashMap<Uuid, rmmt::User> = match *users {
        Ok(ref res) => res.iter().cloned().map(|u| (u.id.clone(), u)).collect(),
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };
    let users = Rc::new(users);
    //account_ctx.dispatch(AccountAction::UpdateUsers(users));

    let balance_url = format!("/api/account/{}/balance", props.id);
    let balance: UseFutureHandle<Result<rmmt::Balance, _>> =
        use_future(|| async move { utils::get(&balance_url).await })?;
    let balance: rmmt::Balance = match *balance {
        Ok(ref res) => res.clone(),
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };
    let balance = Rc::new(balance);
    //account_ctx.dispatch(AccountAction::UpdateBalance(balance.clone()));

    log::debug!("Rerendered account");

    Ok(html! {
        <>
        <AccountTitle id={ account.id.as_ref().clone() } name={ account.inner.name.clone() } />
        <div class="tile is-ancestor">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <span class="icon"><i class="fas fa-balance-scale"></i></span>
                        <span>{ "Balance" }</span>
                    </h3>
                    <BalanceList account_id={ account.id.as_ref().clone() } users={ users.clone() } balance={ balance.clone() } />
                    <CreateUser account_id={ account.id.as_ref().clone() } />
                </div>
            </div>

            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <span class="icon"><i class="fas fa-exchange"></i></span>
                        <span>{ "Équilibrage" }</span>
                    </h3>
                    <BalancingList account_id={ account.id.as_ref().clone() } users={ users.clone() } balance={ balance.clone() } />
                </div>
            </div>
        </div>

        <div class="tile is-ancestor">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::Expenditures { account_id: account.id.as_ref().clone() }}>
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>{ "Dépenses" }</span>
                        </Link<Route>>
                    </h3>
                    <Suspense fallback={utils::loading()}>
                        // TODO avoid cloning users
                        <ExpendituresList account_id={ account.id.as_ref().clone() } users={ users.clone() } limit=10 buttons=true />
                    </Suspense>
                </div>
            </div>
        </div>

        <div class="tile is-ancestor">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::Repayments { account_id: account.id.as_ref().clone() }}>
                            <span class="icon"><i class="fas fa-exchange"></i></span>
                            <span>{ "Remboursements" }</span>
                        </Link<Route>>
                    </h3>
                    <Suspense fallback={utils::loading()}>
                        // TODO avoid cloning users
                        <RepaymentsList account_id={ account.id.as_ref().clone() } users={ users.clone() } limit=10 buttons=true />
                    </Suspense>
                </div>
            </div>
        </div>
        </>
    })
}

#[derive(PartialEq, Properties)]
pub struct CreateAccountProps;

pub enum CreateAccountMsg {
    Submit,
    Created { id: String },
    Error(Error),
}

pub struct CreateAccount {
    creating: bool,
    input_name: NodeRef,
    error: Option<Error>,
}

impl CreateAccount {
    fn create_account(&mut self, ctx: &Context<Self>) -> Result<()> {
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account = rmmt::NewAccount { name };
        ctx.link().send_future(async move {
            let res: Result<String, _> = utils::post("/api/account", &account)
                .await
                .context("Can't create account");
            match res {
                Ok(id) => CreateAccountMsg::Created { id },
                Err(error) => CreateAccountMsg::Error(error),
            }
        });

        Ok(())
    }

    fn clear(&mut self) {
        self.creating = false;
        self.error = None;
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for CreateAccount {
    type Message = CreateAccountMsg;
    type Properties = CreateAccountProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            creating: false,
            input_name: NodeRef::default(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateAccountMsg::Submit => {
                if self.creating {
                    false
                } else {
                    if let Err(error) = self.create_account(ctx) {
                        self.error = Some(error);
                    }
                    true
                }
            }
            CreateAccountMsg::Created { id } => {
                log::info!("Created account: {}", id);
                self.clear();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Account { account_id: id });
                false
            }
            CreateAccountMsg::Error(error) => {
                log::info!("Creation error: {}", error);
                self.error = Some(error);
                self.creating = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            CreateAccountMsg::Submit
        });

        html! {
            <div class="cover">
                <div class="container">
                    <section class="section">
                        <div class="columns">
                            <div class="column">
                                <h3 class="subtitle is-3">
                                    { "Créer un nouveau compte" }
                                </h3>
                                if let Some(error) = self.error.as_ref() {
                                    <FetchError error={ format!("{:?}", error) } />
                                }
                                <form {onsubmit}>
                                    <div class="field has-addons">
                                        <div class={classes!("control", self.creating.then(|| "is-loading"))}>
                                            <input ref={self.input_name.clone()} class="input is-primary" type="text" placeholder="Week-end à Barcelone" name="name" required=true />
                                        </div>
                                        <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                                            <span>{ "Créer" }</span>
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AccountTitleProps {
    pub id: String, // TODO get Rc<String>
    pub name: String,
}

#[function_component(AccountTitle)]
pub fn account_title(AccountTitleProps { id, name }: &AccountTitleProps) -> Html {
    html! {
        <h2 class="title is-1">
            <Link<Route> to={Route::Account { account_id: id.clone() }}>
                <span class="icon">
                    <i class="fas fa-bank"/>
                </span>
                <span>
                    { name }
                </span>
            </Link<Route>>
        </h2>
    }
}
