use std::collections::HashMap;

use gloo_net::http::Request;
use log;
use rmmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use yew_router::prelude::*;

use crate::components::{
    balance::{BalanceList, BalancingList},
    expenditure::ExpendituresList,
    repayment::RepaymentsList,
    user::CreateUser,
};
use crate::Route;

#[derive(Clone, PartialEq)]
struct AccountCtx {
    account: rmmt::Account,
    users: HashMap<Uuid, rmmt::User>,
    balance: rmmt::Balance,
    expenditures: HashMap<Uuid, (rmmt::Expenditure, HashMap<Uuid, rmmt::Debt>)>,
    repayments: HashMap<Uuid, rmmt::Repayment>,
}

#[derive(Properties, PartialEq)]
pub struct AccountProps {
    pub id: String,
    pub route: Route,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountMsg {
    LoadAccount(String),
    UpdateAccount,
    LoadExpenditure,
    UpdateExpenditure,
    LoadRepayment,
    UpdateRepayment,
    ChangedUsers,
    UpdateUsers,
    UpdateBalance,
    ChangedExpenditures,
    UpdateExpenditures,
    ChangedRepayments,
    UpdateRepayments,
}

#[function_component(Account)]
pub fn account(props: &AccountProps) -> HtmlResult {
    let account_id = props.id.clone();
    let res: UseFutureHandle<Result<rmmt::FullAccount, _>> = use_future(|| async move {
        Request::get(&format!("/api/full_account/{}", account_id))
            .send()
            .await?
            .json()
            .await
    })?;

    let account: rmmt::FullAccount = res.as_ref().unwrap().clone();
    let users: HashMap<Uuid, rmmt::User> = todo!();
    let expenditures: HashMap<Uuid, (rmmt::Expenditure, HashMap<Uuid, rmmt::Debt>)> = todo!();
    let repayments: HashMap<Uuid, rmmt::Repayment> = todo!();

    Ok(html! {
        <>
        <AccountTitle id={ props.id.clone() } name={ account.account.name.clone() } />
        <div class="tile is-ancestor">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <span class="icon"><i class="fas fa-balance-scale"></i></span>
                        <span>{ "Balance" }</span>
                    </h3>
                    <BalanceList account_id={ props.id.clone() } users={ users.clone() } balance={ account.balance.clone() } />
                    <CreateUser account_id={ props.id.clone() } />
                </div>
            </div>

            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <span class="icon"><i class="fas fa-exchange"></i></span>
                        <span>{ "Équilibrage" }</span>
                    </h3>
                    <BalancingList account_id={ props.id.clone() } users={ users.clone() } balance={ account.balance.clone() } />
                </div>
            </div>
        </div>

        <div class="tile is-ancestor">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::Expenditures { account_id: props.id.clone() }}>
                            <span class="icon"><i class="fas fa-credit-card"></i></span>
                            <span>{ "Dépenses" }</span>
                        </Link<Route>>
                    </h3>
                    <ExpendituresList account_id={ props.id.clone() } { expenditures } users={ users.clone() } limit=10 buttons=true />
                </div>
            </div>
        </div>

        <div class="tile is-ancestor">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <h3 class="subtitle is-3">
                        <Link<Route> to={Route::Repayments { account_id: props.id.clone() }}>
                            <span class="icon"><i class="fas fa-exchange"></i></span>
                            <span>{ "Remboursements" }</span>
                        </Link<Route>>
                    </h3>
                    <RepaymentsList account_id={ props.id.clone() } users={ users.clone() } { repayments } limit=10 buttons=true />
                </div>
            </div>
        </div>
        </>
    })
}

//pub struct BaseAccount;
//
//impl Component for BaseAccount {
//    type Message = AccountMsg;
//    type Properties = AccountProps;
//
//    fn create(_ctx: &Context<Self>) -> Self {
//        Self
//    }
//
//    fn view(&self, ctx: &Context<Self>) -> Html {
//        // <Breadcrumb route={ ctx.props().route.clone() } />
//        html! {
//            <ContextProvider<Rc<AccountCtx>> context={ctx.props().account.clone()}>
//                <NavBar account_id={ Some(ctx.props().account.id.clone()) } />
//                <div class="container">
//                </div>
//            </ContextProvider<Rc<AccountCtx>>>
//        }
//    }
//}
//
//pub type Account = WithAccount<BaseAccount>;

#[derive(PartialEq, Properties)]
pub struct CreateAccountProps;

pub enum CreateAccountMsg {
    Submit,
    Created { id: String },
}

pub struct CreateAccount {
    creating: bool,
    input_name: NodeRef,
}

impl CreateAccount {
    fn create_account(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account = rmmt::NewAccount { name };
        ctx.link().send_future(async move {
            let created_account: String = Request::post("/api/account/")
                .json(&account)
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            CreateAccountMsg::Created {
                id: created_account,
            }
        });
    }

    fn clear(&mut self) {
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for CreateAccount {
    type Message = CreateAccountMsg;
    type Properties = CreateAccountProps;

    fn create(_ctx: &Context<Self>) -> Self {
        //if let Some(hash) = ctx.link().location().map(|l| l.hash()) {
        //    if hash.starts_with("#!/account/") {
        //        log::debug!("ahah old account");
        //        if let Some(mut captures) = hash.strip_prefix("#!/account/") {
        //            if let Some(end) = captures.find("/") {
        //                captures = &captures[1..end];
        //            }

        //            let account_id = captures.to_string();
        //            log::info!("Redirecting old account_id: {:?}", account_id);
        //            let navigator = ctx.link().navigator().unwrap();
        //            navigator.push(&Route::Account { account_id });
        //        }
        //    }
        //}
        Self {
            creating: false,
            input_name: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateAccountMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.create_account(ctx);
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
    pub id: String,
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
