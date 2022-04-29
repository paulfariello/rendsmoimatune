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
    utils::Amount,
};

#[derive(Properties, PartialEq)]
pub struct ExpendituresListProps {
    pub limit: Option<usize>,
}

pub enum ExpendituresListMsg {
    AccountMsg(AccountMsg),
}

pub struct ExpendituresList {
    expenditures: Rc<RefCell<Option<Vec<rmmt::Expenditure>>>>,
    users: Rc<RefCell<Option<HashMap<Uuid, rmmt::User>>>>,
    _account_bridge: Box<dyn Bridge<AccountAgent>>,
}

impl Component for ExpendituresList {
    type Message = ExpendituresListMsg;
    type Properties = ExpendituresListProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            expenditures: Rc::new(RefCell::new(None)),
            users: Rc::new(RefCell::new(None)),
            _account_bridge: AccountAgent::bridge(
                ctx.link().callback(ExpendituresListMsg::AccountMsg),
            ),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExpendituresListMsg::AccountMsg(msg) => match msg {
                AccountMsg::UpdateUsers(users) => {
                    self.users = users;
                    true
                }
                AccountMsg::UpdateExpenditures(expenditures) => {
                    self.expenditures = expenditures;
                    true
                }
                _ => false,
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(expenditures) = &*self.expenditures.borrow() {
            let len = expenditures.len();

            if len > 0 {
                let map = |expenditure: &rmmt::Expenditure| {
                    html! {
                        <tr>
                            <td class="is-vcentered">{ &expenditure.date }</td>
                            <td class="is-vcentered">{ &expenditure.name }</td>
                            <td class="is-vcentered"><Amount amount={ expenditure.amount } /></td>
                            <td class="is-vcentered"><UserName users={ self.users.clone() } id={ expenditure.payer_id }/></td>
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
                    </>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }
}
