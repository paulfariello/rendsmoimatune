use std::collections::HashMap;
use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;
use log;
use rmmt;
use uuid::Uuid;
use yew::prelude::*;

use crate::utils::{self, Error};

// TODO remove all this old stuffs
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

// TODO rename into Account
#[derive(Debug, PartialEq)]
pub struct AccountQuery {
    pub id: Rc<String>,
    pub name: String,
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
            name: account.name.clone(),
        }
        .into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Users(pub Rc<HashMap<Uuid, rmmt::User>>);

#[async_trait(?Send)]
impl Query for Users {
    type Input = String;
    type Error = Error;

    async fn query(_states: &BounceStates, input: Rc<String>) -> QueryResult<Self> {
        let users_url = format!("/api/account/{}/users", input);
        let users: Vec<rmmt::User> = utils::get(&users_url).await?;

        Ok(Self(
            users
                .iter()
                .cloned()
                .map(|u| (u.id.clone(), u))
                .collect::<HashMap<_, _>>()
                .into(),
        )
        .into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Expenditures(pub Rc<Vec<rmmt::Expenditure>>);

#[async_trait(?Send)]
impl Query for Expenditures {
    type Input = String;
    type Error = Error;

    async fn query(_states: &BounceStates, input: Rc<String>) -> QueryResult<Self> {
        let expenditures_url = format!("/api/account/{}/expenditures", input);
        let expenditures: Vec<rmmt::Expenditure> = utils::get(&expenditures_url).await?;

        Ok(Self(expenditures.into()).into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Repayments(pub Rc<Vec<rmmt::Repayment>>);

#[async_trait(?Send)]
impl Query for Repayments {
    type Input = String;
    type Error = Error;

    async fn query(_states: &BounceStates, input: Rc<String>) -> QueryResult<Self> {
        let repayments_url = format!("/api/account/{}/repayments", input);
        let repayments: Vec<rmmt::Repayment> = utils::get(&repayments_url).await?;

        Ok(Self(repayments.into()).into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Balance(pub Rc<rmmt::Balance>);

#[async_trait(?Send)]
impl Query for Balance {
    type Input = String;
    type Error = Error;

    async fn query(_states: &BounceStates, input: Rc<String>) -> QueryResult<Self> {
        let balance_url = format!("/api/account/{}/balance", input);
        let balance: rmmt::Balance = utils::get(&balance_url).await?;

        Ok(Self(balance.into()).into())
    }
}
