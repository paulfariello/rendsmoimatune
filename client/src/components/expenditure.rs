use yew::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use rmmt;
use crate::components::{user::UserName, utils::Amount};

#[derive(Properties, PartialEq)]
pub struct ExpendituresListProps {
    pub expenditures: Vec<rmmt::Expenditure>,
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
    pub limit: Option<usize>,
}

#[function_component(ExpendituresList)]
pub fn expenditures_list(
    ExpendituresListProps {
        expenditures,
        users,
        limit,
    }: &ExpendituresListProps,
) -> Html {
    let len = expenditures.len();
    let map = |expenditure: &rmmt::Expenditure| {
        html! {
            <tr>
                <td class="is-vcentered">{ &expenditure.date }</td>
                <td class="is-vcentered">{ &expenditure.name }</td>
                <td class="is-vcentered"><Amount amount={ expenditure.amount } /></td>
                <td class="is-vcentered"><UserName users={ users.clone() } id={ expenditure.payer_id }/></td>
                <td class="is-vcentered">{ "todo" }</td>
                <td class="is-vcentered">
                <a aria-label="Éditer" class="button is-info" href="">
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
                        <th>{ "Nom" }</th>
                        <th>{ "Montant" }</th>
                        <th>{ "Payeur" }</th>
                        <th>{ "Participants" }</th>
                        <th>{ "Actions" }</th>
                    </tr>
                </thead>
            <tbody>
            {
                match limit {
                    None => expenditures.iter().map(map).collect::<Html>(),
                    Some(limit) => expenditures.iter().take(*limit).map(map).collect::<Html>(),
                }
            }
            </tbody>
            </table>
            if let Some(limit) = limit {
                <a href="">{ format!("Et {} autres…", len - limit) }</a>
            }
        </>
    }
}
