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
pub struct RepaymentsProps {
    pub account_id: String,
}

pub struct Repayments {
    account: Option<Rc<RefCell<rmmt::Account>>>,
    repayments: Option<Rc<RefCell<Vec<rmmt::Repayment>>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for Repayments {
    type Message = AccountMsg;
    type Properties = RepaymentsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let account_bridge = AccountAgent::bridge(ctx.link().callback(|msg| msg));

        let mut dispatcher = AccountAgent::dispatcher();
        dispatcher.send(AccountMsg::FetchAccount(ctx.props().account_id.clone()));

        Self {
            account: None,
            repayments: None,
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
            AccountMsg::UpdateRepayments(repayments) => {
                self.repayments = Some(repayments);
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
                        <Link<Route> to={Route::Repayments { account_id: ctx.props().account_id.clone() }}>
                        <h3 class="subtitle is-3">
                            <span class="icon-text">
                                <span class="icon"><i class="fa fa-exchange"></i></span>
                                <span>{ "Remboursements" }</span>
                            </span>
                        </h3>
                        </Link<Route>>
                        if let (Some(users), Some(repayments)) = (self.users.clone(), self.repayments.clone()) {
                            <RepaymentsList repayments={ repayments } users={ users }/>
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
pub struct RepaymentsListProps {
    pub limit: Option<usize>,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub repayments: Rc<RefCell<Vec<rmmt::Repayment>>>,
}

pub struct RepaymentsList;

impl Component for RepaymentsList {
    type Message = ();
    type Properties = RepaymentsListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let repayments = &*ctx.props().repayments.borrow();
        let len = repayments.len();

        if len > 0 {
            let map = |repayment: &rmmt::Repayment| {
                html! {
                    <tr>
                        <td class="is-vcentered">{ &repayment.date }</td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ repayment.payer_id } /></td>
                        <td class="is-vcentered">{ "a remboursé" }</td>
                        <td class="is-vcentered"><Amount amount={ repayment.amount } /></td>
                        <td class="is-vcentered">{ "à" }</td>
                        <td class="is-vcentered"><UserName users={ ctx.props().users.clone() } id={ repayment.beneficiary_id } /></td>
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
                <>
                    <table class="table is-fullwidth is-striped is-hoverable">
                        <thead>
                            <tr>
                                <th>{ "Date" }</th>
                                <th>{ "De" }</th>
                                <th></th>
                                <th>{ "Montant" }</th>
                                <th></th>
                                <th>{ "Payeur" }</th>
                                <th>{ "Actions" }</th>
                            </tr>
                        </thead>
                    <tbody>
                    {
                        match ctx.props().limit {
                            Some(limit) => repayments.iter().take(limit).map(map).collect::<Html>(),
                            None => repayments.iter().map(map).collect::<Html>(),
                        }
                    }
                    </tbody>
                    </table>
                    if let Some(limit) = ctx.props().limit {
                        if len > limit {
                            <a href="">{ format!("Et {} autres…", len - limit) }</a>
                        }
                    }
                </>
            }
        } else {
            html! {}
        }
    }
}
