use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use rmmt;
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched};
use yew_router::prelude::*;

use crate::components::{
    user::UserName,
    utils::{Amount, Loading},
};
use crate::agent::{AccountAgent, AccountMsg};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct ExpendituresProps {
    pub account_id: String,
}

pub struct Expenditures {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    expenditures: Option<Rc<RefCell<Vec<rmmt::Expenditure>>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Expenditures {
    type Message = AccountMsg;
    type Properties = ExpendituresProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::FetchAccount(ctx.props().account_id.clone()));

        Self {
            account: None,
            expenditures: None,
            users: None,
            _account_bridge: account_bridge,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountMsg::UpdateAccount(account) => {
                self.account = Some(account);
                true
            },
            AccountMsg::UpdateUsers(users) => {
                self.users = Some(users);
                true
            },
            AccountMsg::UpdateExpenditures(expenditures) => {
                self.expenditures = Some(expenditures);
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="columns">
                <div class="column">
                    <Link<Route> to={Route::Account { account_id: ctx.props().account_id.clone() }}>
                        <h2 class="title is-1">
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="fa fa-bank"/>
                                </span>
                                <span>
                                {
                                    match &self.account {
                                        Some(account) => {
                                            let account = &*account.borrow();
                                            account.name.clone()
                                        }
                                        None => "Loading...".to_string(),
                                    }
                                }
                                </span>
                            </span>
                        </h2>
                    </Link<Route>>
                    <div class="box">
                        <Link<Route> to={Route::Expenditures { account_id: ctx.props().account_id.clone() }}>
                            <h3 class="subtitle is-3">
                                <span class="icon-text">
                                    <span class="icon"><i class="fa fa-credit-card"></i></span>
                                    <span>{ "Dépenses" }</span>
                                </span>
                            </h3>
                        </Link<Route>>
                        if let (Some(users), Some(expenditures)) = (self.users.clone(), self.expenditures.clone()) {
                            <ExpendituresList { expenditures } { users }/>
                        } else {
                            <Loading />
                        }
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ExpendituresListProps {
    pub expenditures: Rc<RefCell<Vec<rmmt::Expenditure>>>,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub limit: Option<usize>,
}

pub struct ExpendituresList;

impl Component for ExpendituresList {
    type Message = ();
    type Properties = ExpendituresListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let expenditures = &*ctx.props().expenditures.borrow();
        let len = expenditures.len();

        if len > 0 {
            let map = |expenditure: &rmmt::Expenditure| {
                html! {
                    <tr>
                        <td class="is-vcentered">{ &expenditure.date }</td>
                        <td class="is-vcentered">{ &expenditure.name }</td>
                        <td class="is-vcentered"><Amount amount={ expenditure.amount } /></td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ expenditure.payer_id }/></td>
                        <td class="is-vcentered">{ "todo" }</td>
                        <td class="is-vcentered">
                        <a aria-label="Éditer" class="button is-primary" href="">
                        <i class="fa fa-pencil fa-lg"></i>
                        </a>
                        <button aria-label="Supprimer" class="button is-danger"><i class="fa fa-trash-o fa-lg"></i></button>
                        </td>
                        </tr>
                }
            };
            html! {
                <div class="is-relative block">
                    <table class="table is-fullwidth is-striped is-hoverable">
                        <thead>
                            <tr>
                                <th>{ "Date" }</th>
                                <th>{ "Nom" }</th>
                                <th>{ "Montant" }</th>
                                <th>{ "Payeur" }</th>
                                <th>{ "Participants" }</th>
                                <th>{ "Actions" }</th>
                            </tr>
                        </thead>
                    <tbody>
                    {
                        match ctx.props().limit {
                            None => expenditures.iter().map(map).collect::<Html>(),
                            Some(limit) => expenditures.iter().take(limit).map(map).collect::<Html>(),
                        }
                    }
                    </tbody>
                    </table>
                    if let Some(limit) = ctx.props().limit {
                        if len > limit {
                            <a href="">{ format!("Et {} autres…", len - limit) }</a>
                        }
                    }
                </div>
            }
        } else {
            html! {}
        }
    }
}
