use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, warn};
use uuid::Uuid;
use yew::prelude::*;

use crate::components::utils::Loading;
use crate::components::{user::UserName, utils::Amount};

#[derive(Properties, PartialEq)]
pub struct BalanceListProps {
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub balances: Rc<RefCell<(Vec<rmmt::Balance>, i64, Vec<rmmt::Balancing>)>>,
    pub loading: bool,
}

pub struct BalanceList;

impl Component for BalanceList {
    type Message = ();
    type Properties = BalanceListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (balances, remaining, _) = &*ctx.props().balances.borrow();
        let users = &ctx.props().users;

        let max = balances
            .iter()
            .map(|b| b.amount)
            .max()
            .unwrap_or(0)
            .to_string();

        html! {
            <div class="balance is-relative block">
                if ctx.props().loading {
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
                                        <td class="is-vcentered has-text-centered"><UserName users={ users.clone() } id={ balance.user_id }/></td>
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
                if *remaining != 0 {
                    <div class="notification is-info">
                      { "Oups, nous avons perdu " }<Amount amount={ remaining.abs() } />{ " dans des arrondis…" }
                    </div>
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BalancingListProps {
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub balances: Rc<RefCell<(Vec<rmmt::Balance>, i64, Vec<rmmt::Balancing>)>>,
    pub loading: bool,
}

pub struct BalancingList;

impl Component for BalancingList {
    type Message = ();
    type Properties = BalancingListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (_, _, balancing) = &*ctx.props().balances.borrow();
        let users = &ctx.props().users;

        html! {
            <div class="balancing is-relative block">
                if ctx.props().loading {
                    <div class="loading-overlay">
                        <Loading />
                    </div>
                }
                if balancing.is_empty() {
                    <div class="notification is-success">
                        { "Bien joué, personne de doit rien à personne." }
                    </div>
                } else {
                    <table class="table is-fullwidth is-striped is-hoverable">
                        <tbody>
                            {
                                balancing.iter().map(|balance| {
                                    html! {
                                        <tr>
                                            <td class="is-vcentered has-text-centered"><UserName users={ users.clone() } id={ balance.payer_id }/></td>
                                            <td class="is-vcentered has-text-centered">{ "doit" }</td>
                                            <td class="is-vcentered">
                                                <Amount amount={ balance.amount } />
                                            </td>
                                            <td class="is-vcentered has-text-centered">{ "à" }</td>
                                            <td class="is-vcentered has-text-centered"><UserName users={ users.clone() } id={ balance.beneficiary_id }/></td>
                                            <td>
                                                <button class="button is-primary">
                                                    <span class="icon">
                                                        <i class="fa fa-credit-card" />
                                                    </span>
                                                    <span>{ "Rembourser" }</span>
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                }
            </div>
        }
    }
}
