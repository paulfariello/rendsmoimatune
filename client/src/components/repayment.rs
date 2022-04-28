use crate::components::{user::UserName, utils::Amount};
use rmmt;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepaymentsListProps {
    pub repayments: Vec<rmmt::Repayment>,
    pub users: HashMap<Uuid, rmmt::User>,
    pub limit: Option<usize>,
}

#[function_component(RepaymentsList)]
pub fn repayments_list(
    RepaymentsListProps {
        repayments,
        users,
        limit,
    }: &RepaymentsListProps,
) -> Html {
    let len = repayments.len();

    if len > 0 {
        let map = |repayment: &rmmt::Repayment| {
            html! {
                <tr>
                    <td class="is-vcentered">{ &repayment.date }</td>
                    <td class="is-vcentered"><UserName users={ users.clone() } id={ repayment.payer_id } /></td>
                    <td class="is-vcentered">{ "a remboursé" }</td>
                    <td class="is-vcentered"><Amount amount={ repayment.amount } /></td>
                    <td class="is-vcentered">{ "à" }</td>
                    <td class="is-vcentered"><UserName users={ users.clone() } id={ repayment.beneficiary_id } /></td>
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
                    match limit {
                        Some(limit) => repayments.iter().take(*limit).map(map).collect::<Html>(),
                        None => repayments.iter().map(map).collect::<Html>(),
                    }
                }
                </tbody>
                </table>
                if let Some(limit) = limit {
                    if len > *limit {
                        <a href="">{ format!("Et {} autres…", len - limit) }</a>
                    }
                }
            </>
        }
    } else {
        html! {}
    }
}
