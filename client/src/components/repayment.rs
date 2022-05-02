use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use rmmt;
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::components::{
    account::{AccountAgent, AccountMsg},
    user::UserName,
    utils::{Amount, Loading},
};

#[derive(Properties, PartialEq)]
pub struct RepaymentsListProps {
    pub limit: Option<usize>,
}

pub enum RepaymentsListMsg {
    AccountMsg(AccountMsg),
}

pub struct RepaymentsList {
    repayments: Rc<RefCell<Option<Vec<rmmt::Repayment>>>>,
    users: Rc<RefCell<Option<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for RepaymentsList {
    type Message = RepaymentsListMsg;
    type Properties = RepaymentsListProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            repayments: Rc::new(RefCell::new(None)),
            users: Rc::new(RefCell::new(None)),
            _account_bridge: AccountAgent::bridge(
                ctx.link().callback(RepaymentsListMsg::AccountMsg),
            ),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RepaymentsListMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateUsers(users) => {
                    self.users = users;
                    true
                }
                AccountMsg::UpdateRepayments(repayments) => {
                    self.repayments = repayments;
                    true
                }
                _ => false,
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(repayments) = &*self.repayments.borrow() {
            let len = repayments.len();

            if len > 0 {
                let map = |repayment: &rmmt::Repayment| {
                    html! {
                        <tr>
                            <td class="is-vcentered">{ &repayment.date }</td>
                            <td class="is-vcentered"><UserName users={ self.users.clone() } id={ repayment.payer_id } /></td>
                            <td class="is-vcentered">{ "a remboursé" }</td>
                            <td class="is-vcentered"><Amount amount={ repayment.amount } /></td>
                            <td class="is-vcentered">{ "à" }</td>
                            <td class="is-vcentered"><UserName users={ self.users.clone() } id={ repayment.beneficiary_id } /></td>
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
        } else {
            html! {
                <Loading />
            }
        }
    }
}
