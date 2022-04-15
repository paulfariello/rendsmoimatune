use yew::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use rmmt;

#[derive(Properties, PartialEq)]
pub struct UserProps {
    pub users: Rc<HashMap<Uuid, rmmt::User>>,
    pub id: Uuid,
}

#[function_component(UserName)]
pub fn user_name(UserProps { users, id }: &UserProps) -> Html {
    html! {
        { &users.get(&id).unwrap().name }
    }
}
