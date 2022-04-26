use std::collections::HashMap;
use std::rc::Rc;

use gloo_net::http::Request;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rmmt::{self, prelude::*};
use uuid::Uuid;
use yew::prelude::*;

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

#[derive(PartialEq, Properties)]
pub struct CreateUserProps {
    pub account_id: String,
}

pub enum CreateUserMsg {
    Submit,
    Created { user: rmmt::User },
}

pub struct CreateUser {
    creating: bool,
    input_name: NodeRef,
}

impl CreateUser {
    fn create_user(&mut self, ctx: &Context<Self>) {
        self.creating = true;

        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        let name = input_name.value();

        let account_id: UniqId = ctx.props().account_id.clone().try_into().unwrap();
        let user = rmmt::NewUser {
            account_id: account_id.into(),
            name,
        };
        let url = format!("/api/account/{}/users", ctx.props().account_id);
        ctx.link().send_future(async move {
            let created_user: rmmt::User = Request::post(&url)
                .json(&user)
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            CreateUserMsg::Created { user: created_user }
        });
    }

    fn clear(&mut self) {
        self.creating = false;
        let input_name = self.input_name.cast::<web_sys::HtmlInputElement>().unwrap();
        input_name.set_value("");
    }
}

impl Component for CreateUser {
    type Message = CreateUserMsg;
    type Properties = CreateUserProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            creating: false,
            input_name: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateUserMsg::Submit => {
                if self.creating {
                    false
                } else {
                    self.create_user(ctx);
                    true
                }
            }
            CreateUserMsg::Created { user } => {
                info!("Created user: {}", user.name);
                self.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            CreateUserMsg::Submit
        });

        html! {
            <form {onsubmit}>
                <div class="field has-addons">
                    <div class={classes!("control", self.creating.then(|| "is-loading"))}>
                        <input ref={self.input_name.clone()} type="text" class="input is-primary" name="name" required=true />
                    </div>
                    <div class="control">
                        <button type="submit" class={classes!("button", "is-primary", self.creating.then(|| "is-loading"))}>
                            <span class="icon">
                                <i class="fa fa-user-plus" />
                            </span>
                            <span>{ "Ajouter" }</span>
                        </button>
                    </div>
                </div>
            </form>
        }
    }
}
