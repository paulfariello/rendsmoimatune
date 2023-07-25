use std::collections::HashMap;
use std::rc::Rc;

use async_trait::async_trait;
use bounce::{
    query::{Query, QueryResult},
    BounceStates,
};
use log;
use rmmt;
use uuid::Uuid;
use yew::prelude::*;

use crate::utils;
use crate::utils::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
    pub balance: Rc<rmmt::Balance>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AccountAction {
    UpdateName(String),
    UpdateUsers(HashMap<Uuid, rmmt::User>),
    UpdateBalance(rmmt::Balance),
}

impl Reducible for Account {
    type Action = AccountAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        log::info!("Update account ctx with {:?}", action);
        match action {
            AccountAction::UpdateName(name) => Self {
                id: self.id.clone(),
                name,
                users: self.users.clone(),
                balance: self.balance.clone(),
            },
            AccountAction::UpdateUsers(users) => Self {
                id: self.id.clone(),
                name: self.name.clone(),
                users: Rc::new(users),
                balance: self.balance.clone(),
            },
            AccountAction::UpdateBalance(balance) => Self {
                id: self.id.clone(),
                name: self.name.clone(),
                users: self.users.clone(),
                balance: Rc::new(balance),
            },
        }
        .into()
    }
}

pub type AccountCtx = UseReducerHandle<Account>;

#[derive(Properties, PartialEq)]
pub struct AccountProviderProps {
    pub id: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AccountProvider)]
pub fn account_provider(props: &AccountProviderProps) -> HtmlResult {
    let account_ctx = use_reducer(|| Account {
        id: props.id.clone(),
        name: String::new(),
        users: Rc::new(HashMap::new()),
        balance: Rc::new(rmmt::Balance::default()),
    });

    Ok(html! {
        <div class="container">
             <ContextProvider<AccountCtx> context={account_ctx}>
                {props.children.clone()}
            </ContextProvider<AccountCtx>>
        </div>
    })
}

#[derive(Debug, PartialEq)]
pub struct AccountQuery {
    pub id: Rc<String>,
    pub inner: rmmt::Account,
}

#[async_trait(?Send)]
impl Query for AccountQuery {
    type Input = String;
    type Error = Error;

    async fn query(_states: &BounceStates, input: Rc<String>) -> QueryResult<Self> {
        let account_url = format!("/api/account/{}", input);
        let account: rmmt::Account = utils::get(&account_url).await?;

        Ok(AccountQuery {
            id: input.clone(),
            inner: account,
        }
        .into())
    }
}
