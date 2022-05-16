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
            <div class="container">
                <Account id={ account_id.clone() } />
            </div>
        },
        Route::Expenditures { account_id } => html! {
            <div class="container">
                <Expenditures account_id={ account_id.clone() } />
            </div>
        },
        Route::CreateExpenditure { account_id } => html! {
            <div class="container">
                <EditExpenditure account_id={ account_id.clone() } />
            </div>
        },
        Route::Repayments { account_id } => html! {
            <div class="container">
                <Repayments account_id={ account_id.clone() } />
            </div>
        },
        Route::CreateRepayment { account_id } => html! {
            <div class="container">
                <EditRepayment account_id={ account_id.clone() } />
            </div>
        },
        Route::EditRepayment {
            account_id,
            repayment_id,
        } => html! {
            <div class="container">
                <EditRepayment account_id={ account_id.clone() } repayment_id={ repayment_id.clone() } />
            </div>
        },
        Route::EditExpenditure {
            account_id,
            expenditure_id,
        } => html! {
            <div class="container">
                <EditExpenditure account_id={ account_id.clone() } expenditure_id={ expenditure_id.clone() } />
            </div>
        },
        Route::NotFound => html! {
            <div class="container">
                <h1 class="title is-1">{ "Oups… Cette page n'existe pas" }</h1>
            </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <TopBar/>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
