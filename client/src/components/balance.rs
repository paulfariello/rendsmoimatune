use yew::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use crate::components::{user::UserName, utils::Amount};

#[derive(Properties, PartialEq)]
pub struct BalanceListProps {
    pub balance: Vec<rmmt::Balance>,
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
}

#[function_component(BalanceList)]
pub fn balance_list(BalanceListProps { balance, users }: &BalanceListProps) -> Html {
    let max = balance
        .iter()
        .map(|b| b.amount)
        .max()
        .unwrap_or(0)
        .to_string();
    let mut balance = balance.clone();
    balance.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());

    html! {
        <table class="table is-fullwidth is-striped is-hoverable">
            <tbody>
                {
                    balance.iter().map(|balance| {
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
    }
}
