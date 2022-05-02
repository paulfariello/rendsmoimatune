use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, warn};
use uuid::Uuid;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::components::account::{AccountAgent, AccountMsg};
use crate::components::utils::Loading;
use crate::components::{user::UserName, utils::Amount};

#[derive(Properties, PartialEq)]
pub struct BalanceListProps {}

pub enum BalanceListMsg {
    AccountMsg(AccountMsg),
}

pub struct BalanceList {
    loading: bool,
    balances: Rc<RefCell<Option<Vec<rmmt::Balance>>>>,
    users: Rc<RefCell<Option<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for BalanceList {
    type Message = BalanceListMsg;
    type Properties = BalanceListProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            loading: false,
            balances: Rc::new(RefCell::new(None)),
            users: Rc::new(RefCell::new(None)),
            _account_bridge: AccountAgent::bridge(ctx.link().callback(BalanceListMsg::AccountMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BalanceListMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateUsers(users) => {
                    self.users = users;
                    true
                }
                AccountMsg::UpdateBalances(balances) => {
                    self.loading = false;
                    self.balances = balances;
                    true
                }
                AccountMsg::FetchUsers => {
                    self.loading = true;
                    true
                }
                _ => false,
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(balances) = &*self.balances.borrow() {
            let max = balances
                .iter()
                .map(|b| b.amount)
                .max()
                .unwrap_or(0)
                .to_string();

            html! {
                <div class="balance is-relative block">
                    if self.loading {
                        <div class="loading-overlay">
                            <Loading />
                        </div>
                    }
                    <table class="table is-fullwidth is-striped is-hoverable">
                        <tbody>
                            {
                                balances.iter().map(|balance| {
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
                </div>
            }
        } else {
            html! { <Loading /> }
        }
    }
}
