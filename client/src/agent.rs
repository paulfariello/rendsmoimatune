use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
use rmmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum RmmtMsg {
    NewUser(rmmt::User),
}

pub(crate) struct RmmtAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for RmmtAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = RmmtMsg;
    type Output = RmmtMsg;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
