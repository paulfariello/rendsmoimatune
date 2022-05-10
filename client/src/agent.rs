use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt;
use uuid::Uuid;
use yew_agent::{Agent, AgentLink, Context as AgentContext, HandlerId};

#[derive(Debug, Clone)]
pub enum AccountMsg {
    LoadAccount(String),
    UpdateAccount(Rc<RefCell<rmmt::Account>>),
    LoadExpenditure{ account_id: String, expenditure_id: Uuid },
    UpdateExpenditure(rmmt::Expenditure),
    LoadRepayment{ account_id: String, repayment_id: Uuid },
    UpdateRepayment(rmmt::Repayment),
    ChangedUsers,
    UpdateUsers(Rc<RefCell<HashMap<Uuid, rmmt::User>>>),
    UpdateBalance(Rc<RefCell<rmmt::Balance>>),
    ChangedExpenditures,
    UpdateExpenditures(Rc<RefCell<HashMap<Uuid, rmmt::Expenditure>>>),
    ChangedRepayments,
    UpdateRepayments(Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>),
}

pub struct AccountAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    account_id: Option<String>,
    account: Option<Rc<RefCell<rmmt::Account>>>,
    users: Option<Rc<RefCell<HashMap<Uuid, rmmt::User>>>>,
    balance: Option<Rc<RefCell<rmmt::Balance>>>,
    expenditures: Option<Rc<RefCell<HashMap<Uuid, rmmt::Expenditure>>>>,
    repayments: Option<Rc<RefCell<HashMap<Uuid, rmmt::Repayment>>>>,
}

impl AccountAgent {
    fn fetch_account(&mut self, account_id: String) {
        info!("Fetching account: {}", account_id);
        self.account_id = Some(account_id);
        let account_id = self.account_id.clone().unwrap();
        self.link.send_future(async move {
            let account: rmmt::Account = Request::get(&format!("/api/account/{}", account_id))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            AccountMsg::UpdateAccount(Rc::new(RefCell::new(account)))
        })
    }

    fn fetch_users(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching users for account: {}", account_id);
                let account_id = account_id.clone();
                self.link.send_future(async move {
                    let users: Vec<rmmt::User> =
                        Request::get(&format!("/api/account/{}/users", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!("Fetched {} users for account: {}", users.len(), account_id);
                    let users = users.into_iter().map(|u| (u.id.clone(), u)).collect();
                    AccountMsg::UpdateUsers(Rc::new(RefCell::new(users)))
                });
            }
            None => error!("Cannot fetch users without account id"),
        }
    }

    fn fetch_balance(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching balance for account: {}", account_id);
                let account_id = account_id.clone();
                self.link.send_future(async move {
                    let mut balance: rmmt::Balance =
                        Request::get(&format!("/api/account/{}/balance", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!("Fetched balance for account: {}", account_id);
                    balance
                        .user_balances
                        .sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());
                    balance
                        .balancing
                        .sort_by(|a, b| a.payer_id.partial_cmp(&b.payer_id).unwrap());
                    AccountMsg::UpdateBalance(Rc::new(RefCell::new(balance)))
                });
            }
            None => error!("Cannot fetch balance without account id"),
        }
    }

    fn fetch_expenditures(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching expenditures for account: {}", account_id);
                let account_id = account_id.clone();
                self.link.send_future(async move {
                    let expenditures: Vec<rmmt::Expenditure> =
                        Request::get(&format!("/api/account/{}/expenditures", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!(
                        "Fetched {} expenditures for account: {}",
                        expenditures.len(),
                        account_id
                    );
                    let expenditures = expenditures.into_iter().map(|e| (e.id.clone(), e)).collect();
                    AccountMsg::UpdateExpenditures(Rc::new(RefCell::new(expenditures)))
                });
            }
            None => error!("Cannot fetch expenditures without account id"),
        }
    }

    fn fetch_repayments(&mut self) {
        match self.account_id.as_ref() {
            Some(account_id) => {
                info!("Fetching repayments for account: {}", account_id);
                let account_id = account_id.clone();
                self.link.send_future(async move {
                    let repayments: Vec<rmmt::Repayment> =
                        Request::get(&format!("/api/account/{}/repayments", account_id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    info!(
                        "Fetched {} repayments for account: {}",
                        repayments.len(),
                        account_id
                    );
                    let repayments = repayments.into_iter().map(|r| (r.id.clone(), r)).collect();
                    AccountMsg::UpdateRepayments(Rc::new(RefCell::new(repayments)))
                });
            }
            None => error!("Cannot fetch repayments without account id"),
        }
    }

    fn broadcast(&self, msg: <Self as Agent>::Output) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }
}

impl Agent for AccountAgent {
    type Reach = AgentContext<Self>;
    type Message = AccountMsg;
    type Input = AccountMsg;
    type Output = AccountMsg;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            account_id: None,
            account: None,
            users: None,
            balance: None,
            expenditures: None,
            repayments: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match &msg {
            AccountMsg::UpdateAccount(account) => self.account = Some(account.clone()),
            AccountMsg::UpdateUsers(users) => self.users = Some(users.clone()),
            AccountMsg::UpdateBalance(balance) => self.balance = Some(balance.clone()),
            AccountMsg::UpdateExpenditures(expenditures) => {
                self.expenditures = Some(expenditures.clone())
            }
            AccountMsg::UpdateRepayments(repayments) => self.repayments = Some(repayments.clone()),
            _ => {}
        }
        self.broadcast(msg);
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        debug!("Handle account msg: {:?}", msg);
        let broadcast = match &msg {
            AccountMsg::LoadAccount(id) => {
                if Some(id) != self.account_id.as_ref() {
                    self.fetch_account(id.clone());
                    self.fetch_users();
                    self.fetch_balance();
                    self.fetch_expenditures();
                    self.fetch_repayments();
                }
                false
            }
            AccountMsg::LoadExpenditure{ account_id, expenditure_id } => {
                if Some(account_id) == self.account_id.as_ref() {
                    if let Some(expenditures) = self.expenditures.as_ref() {
                        match expenditures.borrow().get(expenditure_id) {
                            Some(expenditure) => self.link.respond(id, AccountMsg::UpdateExpenditure(expenditure.clone())),
                            None => {}
                        }
                    }
                } else {
                    error!("Invalid account_id: {} != {:?}", account_id, self.account_id);
                }
                false
            }
            AccountMsg::LoadRepayment{ account_id, repayment_id } => {
                if Some(account_id) == self.account_id.as_ref() {
                    if let Some(repayments) = self.repayments.as_ref() {
                        match repayments.borrow().get(repayment_id) {
                            Some(repayment) => self.link.respond(id, AccountMsg::UpdateRepayment(repayment.clone())),
                            None => {}
                        }
                    }
                } else {
                    error!("Invalid account_id: {} != {:?}", account_id, self.account_id);
                }
                false
            }
            AccountMsg::ChangedUsers => {
                self.fetch_users();
                self.fetch_balance();
                true
            }
            AccountMsg::ChangedExpenditures => {
                self.fetch_expenditures();
                self.fetch_balance();
                true
            }
            AccountMsg::ChangedRepayments => {
                self.fetch_repayments();
                self.fetch_balance();
                true
            }
            _ => true,
        };

        if broadcast {
            self.broadcast(msg);
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
        if let Some(account) = self.account.clone() {
            self.link.respond(id, AccountMsg::UpdateAccount(account));
        }
        if let Some(users) = self.users.clone() {
            self.link.respond(id, AccountMsg::UpdateUsers(users));
        }
        if let Some(balance) = self.balance.clone() {
            self.link.respond(id, AccountMsg::UpdateBalance(balance));
        }
        if let Some(expenditures) = self.expenditures.clone() {
            self.link
                .respond(id, AccountMsg::UpdateExpenditures(expenditures));
        }
        if let Some(repayments) = self.repayments.clone() {
            self.link
                .respond(id, AccountMsg::UpdateRepayments(repayments));
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
