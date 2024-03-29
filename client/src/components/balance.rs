use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, warn};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

use crate::components::{
    user::UserName,
    utils::{Amount, Loading},
};

#[derive(Properties, PartialEq)]
pub struct BalanceListProps {
    pub account_id: String,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub balance: Rc<RefCell<rmmt::Balance>>,
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
        let balance = &*ctx.props().balance.borrow();
        let users = &ctx.props().users;

        let max = balance
            .user_balances
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
                            balance.user_balances.iter().map(|balance| {
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
                                        <td class="is-vcentered has-text-centered"><UserName account_id={ ctx.props().account_id.clone() } users={ users.clone() } id={ balance.user_id }/></td>
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
                if balance.account_remaining != 0 {
                    <div class="notification is-info">
                      { "Oups, nous avons perdu " }<Amount amount={ balance.account_remaining.abs() } />{ " dans des arrondis…" }
                    </div>
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BalancingListProps {
    pub account_id: String,
    pub users: Rc<RefCell<HashMap<Uuid, rmmt::User>>>,
    pub balance: Rc<RefCell<rmmt::Balance>>,
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
        let balance = &*ctx.props().balance.borrow();
        let users = &ctx.props().users;

        html! {
            <div class="balancing is-relative block">
                if ctx.props().loading {
                    <div class="loading-overlay">
                        <Loading />
                    </div>
                }
                if balance.balancing.is_empty() {
                    <div class="notification is-success is-light">
                        { "Bien joué, personne de doit rien à personne." }
                    </div>
                } else {
                    <table class="table is-fullwidth is-striped is-hoverable">
                        <thead>
                            <tr>
                                <th class="is-hidden-touch">{ "Payeur" }</th>
                                <th class="is-hidden-desktop">{ "De" }</th>
                                <th class="is-hidden-touch"></th>
                                <th class="is-hidden-touch">{ "Montant" }</th>
                                <th class="is-hidden-desktop"></th>
                                <th class="is-hidden-touch"></th>
                                <th class="is-hidden-touch">{ "Beneficiaire" }</th>
                                <th class="is-hidden-desktop">{ "À" }</th>
                                <th class="is-hidden-touch">{ "Actions" }</th>
                                <th class="is-hidden-desktop"></th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                balance.balancing.iter().map(|balance| {
                                    html! {
                                        <tr>
                                            <td class="is-vcentered has-text-centered"><UserName account_id={ ctx.props().account_id.clone() } users={ users.clone() } id={ balance.payer_id }/></td>
                                            <td class="is-vcentered has-text-centered is-hidden-touch">{ "doit" }</td>
                                            <td class="is-vcentered">
                                                <Amount amount={ balance.amount } />
                                            </td>
                                            <td class="is-vcentered has-text-centered is-hidden-touch">{ "à" }</td>
                                            <td class="is-vcentered has-text-centered"><UserName account_id={ ctx.props().account_id.clone() } users={ users.clone() } id={ balance.beneficiary_id }/></td>
                                            <td>
                                                <Link<Route, rmmt::Balancing> to={Route::CreateRepayment { account_id: ctx.props().account_id.clone() } } query={ Some(balance.clone()) } classes="button is-primary is-hidden-touch">
                                                    <span class="icon">
                                                        <i class="fas fa-credit-card" />
                                                    </span>
                                                    <span>{ "Rembourser" }</span>
                                                </Link<Route, rmmt::Balancing>>
                                                <Link<Route, rmmt::Balancing> to={Route::CreateRepayment { account_id: ctx.props().account_id.clone() } } query={ Some(balance.clone()) } classes="button is-primary is-hidden-desktop">
                                                    <span class="icon">
                                                        <i class="fas fa-credit-card" />
                                                    </span>
                                                </Link<Route, rmmt::Balancing>>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                }
                if !balance.balancing_remaining.is_empty() {
                    <div class="notification is-info content">
                        { "Avec ces histoires d'arrondis il y a des gagnants et des perdants…" }
                        <ul>
                            {
                                balance.balancing_remaining.iter().map(|balance| html!{
                                    <li>
                                        <UserName account_id={ ctx.props().account_id.clone() } users={ users.clone() } id={ balance.user_id } color="white" />
                                        if balance.amount > 0 {
                                            { " à perdu " }
                                        } else {
                                            { " à gagné " }
                                        }
                                        <Amount amount={ balance.amount.abs() } />
                                    </li>
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                }
            </div>
        }
    }
}
