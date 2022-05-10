use uuid::Uuid;
use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

mod agent;
mod components;

use components::{
    account::{Account, CreateAccount},
    expenditure::{EditExpenditure, Expenditures},
    repayment::{EditRepayment, Repayments},
    utils::TopBar,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/account/:account_id")]
    Account { account_id: String },
    #[at("/account/:account_id/expenditures")]
    Expenditures { account_id: String },
    #[at("/account/:account_id/create_expenditure")]
    CreateExpenditure { account_id: String },
    #[at("/account/:account_id/repayments")]
    Repayments { account_id: String },
    #[at("/account/:account_id/create_repayment")]
    CreateRepayment { account_id: String },
    #[at("/account/:account_id/repayments/:repayment_id/edit")]
    EditRepayment {
        account_id: String,
        repayment_id: Uuid,
    },
    #[at("/account/:account_id/expenditures/:expenditure_id/edit")]
    EditExpenditure {
        account_id: String,
        expenditure_id: Uuid,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <CreateAccount /> },
        Route::Account { account_id } => html! {
            <Account id={ account_id.clone() } />
        },
        Route::Expenditures { account_id } => html! {
            <Expenditures account_id={ account_id.clone() } />
        },
        Route::CreateExpenditure { account_id } => html! {
            <EditExpenditure account_id={ account_id.clone() } />
        },
        Route::Repayments { account_id } => html! {
            <Repayments account_id={ account_id.clone() } />
        },
        Route::CreateRepayment { account_id } => html! {
            <EditRepayment account_id={ account_id.clone() } />
        },
        Route::EditRepayment {
            account_id,
            repayment_id,
        } => html! {
            <EditRepayment account_id={ account_id.clone() } repayment_id={ repayment_id.clone() } />
        },
        Route::EditExpenditure {
            account_id,
            expenditure_id,
        } => html! {
            <EditExpenditure account_id={ account_id.clone() } expenditure_id={ expenditure_id.clone() } />
        },
        Route::NotFound => {
            html! { <h1 class="title is-1">{ "Oups… Cette page n'existe pas" }</h1> }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <body>
            <TopBar/>
            <div class="container">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
