use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use gloo_net::http::Request;
use log;
use rmmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use yew_router::prelude::*;

use crate::components::utils::{Breadcrumb, NavBar};
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
    //UpdateAccount(Rc<RefCell<rmmt::Account>>),
    //LoadExpenditure {
    //    account_id: String,
    //    expenditure_id: Uuid,
    //},
    //UpdateExpenditure(rmmt::Expenditure, HashMap<Uuid, rmmt::Debt>),
    //LoadRepayment {
    //    account_id: String,
    //    repayment_id: Uuid,
    //},
    //UpdateRepayment(rmmt::Repayment),
    //ChangedUsers,
    //UpdateUsers(Rc<RefCell<HashMap<Uuid, rmmt::User>>>),
    //UpdateBalance(Rc<RefCell<rmmt::Balance>>),
    //ChangedExpenditures,
    //UpdateExpenditures(Rc<RefCell<HashMap<Uuid, (rmmt::Expenditure, HashMap<Uuid, rmmt::Debt>)>>>),
    //ChangedRepayments,
    //UpdateRepayments(Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>),
}

#[function_component(Account)]
pub fn account(props: &AccountProps) -> HtmlResult {
    let account_id = props.id.clone();
    let res: UseFutureHandle<Result<rmmt::Account, _>> = use_future(|| async move {
            Request::get(&format!("/api/account/{}", account_id))
                .send()
                .await?
                .json()
                .await
    })?;

    let account: rmmt::Account = res.as_ref().unwrap().clone();

    Ok(html! {<div>{"Hello, "}{account.name}</div>})
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
    pub account: Option<Rc<RefCell<rmmt::Account>>>,
}

#[function_component(AccountTitle)]
pub fn account_title(AccountTitleProps { id, account }: &AccountTitleProps) -> Html {
    html! {
        <h2 class="title is-1">
            <Link<Route> to={Route::Account { account_id: id.clone() }}>
                <span class="icon">
                    <i class="fas fa-bank"/>
                </span>
                <span>
                {
                    match account {
                        Some(account) => {
                            let account = &*account.borrow();
                            account.name.clone()
                        }
                        None => "Loading...".to_string(),
                    }
                }
                </span>
            </Link<Route>>
        </h2>
    }
}
