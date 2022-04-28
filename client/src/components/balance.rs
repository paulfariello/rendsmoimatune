use crate::components::{user::UserName, utils::Amount};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
#[allow(unused_imports)]
use log::{debug, error, info, warn};

use crate::agent::{RmmtAgent, RmmtMsg};

#[derive(Properties, PartialEq)]
pub(crate) struct BalanceListProps {
    pub balance: Vec<rmmt::Balance>,
    pub users: HashMap<Uuid, rmmt::User>,
}

pub(crate) enum BalanceListMsg {
    RmmtMsg(RmmtMsg),
}

pub(crate) struct BalanceList {
    balance: Vec<rmmt::Balance>,
    users: HashMap<Uuid, rmmt::User>,
    producer: Box<dyn Bridge<RmmtAgent>>,
}

impl BalanceList {
    fn add_user(&mut self, user: rmmt::User) {
        self.users.insert(user.id, user.clone());
        self.balance.push(rmmt::Balance {
            user_id: user.id,
            amount: 0,
        });
        self.balance.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());
    }
}

impl Component for BalanceList {
    type Message = BalanceListMsg;
    type Properties = BalanceListProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut balance = ctx.props().balance.clone();
        balance.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());

        Self {
            balance,
            users: ctx.props().users.clone(),
            producer: RmmtAgent::bridge(ctx.link().callback(BalanceListMsg::RmmtMsg)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BalanceListMsg::RmmtMsg(msg) => match msg {
                RmmtMsg::NewUser(user) => {
                    info!("new user: {:?}", user);
                    self.add_user(user);
                    true
                }
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let max = self.balance
            .iter()
            .map(|b| b.amount)
            .max()
            .unwrap_or(0)
            .to_string();

        html! {
            <table class="table is-fullwidth is-striped is-hoverable">
                <tbody>
                    {
                        self.balance.iter().map(|balance| {
                            html! {
                                <tr>
                                    <td class="is-vcentered">
                                    if balance.amount < 0 {
                                        <div class="progress-wrapper">
                                            <progress class="progress is-large is-danger is-revert" value={ balance.amount.abs().to_string() } max={ max.clone() }>
                                                <Amount amount={ balance.amount } />
                                            </progress>
                                            <p class="progress-value has-text-white"><Amount amount={ balance.amount } /></p>
                                        </div>
                                    }
                                    </td>
                                    <td class="is-vcentered has-text-centered"><UserName users={ self.users.clone() } id={ balance.user_id }/></td>
                                    <td class="is-vcentered">
                                    if balance.amount > 0 {
                                        <div class="progress-wrapper">
                                            <progress class="progress is-large is-success" value={ balance.amount.abs().to_string() } max={ max.clone() }>
                                                <Amount amount={ balance.amount } />
                                            </progress>
                                            <p class="progress-value has-text-white"><Amount amount={ balance.amount } /></p>
                                        </div>
                                    }
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        }
    }
}
