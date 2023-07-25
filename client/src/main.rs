extern crate wee_alloc;

use bounce::prelude::*;
use bounce::BounceRoot;
use uuid::Uuid;
use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod components;
mod utils;

use components::{
    account::{Account, CreateAccount},
    ctx::AccountProvider,
    expenditure::{EditExpenditure, Expenditures},
    repayment::{EditRepayment, Repayments},
    user::User,
    utils::NavBar,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
    #[at("/account/:account_id/users/:user_id")]
    User { account_id: String, user_id: Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    let cloned_route = route.clone();
    match route {
        Route::Home => html! {
            <>
            <NavBar />
            <CreateAccount />
            </>
        },
        Route::Account { account_id } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <BounceRoot>
                <Suspense fallback={ utils::loading() }>
                    <Account id={ account_id.clone() } />
                </Suspense>
            </BounceRoot>
            </>
        },
        Route::Expenditures { account_id } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <Expenditures />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::CreateExpenditure { account_id } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <EditExpenditure />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::Repayments { account_id } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <Repayments />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::CreateRepayment { account_id } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <EditRepayment />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::EditRepayment {
            account_id,
            repayment_id,
        } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <EditRepayment repayment_id={ repayment_id.clone() } />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::EditExpenditure {
            account_id,
            expenditure_id,
        } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <EditExpenditure expenditure_id={ expenditure_id.clone() } />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::User {
            account_id,
            user_id,
        } => html! {
            <>
            <NavBar account_id={ account_id.clone() } />
            <AccountProvider id={ account_id.clone() }>
                //<Breadcrumb route={ route.clone() } />
                <Suspense fallback={ utils::loading() }>
                    <User user_id={ user_id.clone() } />
                </Suspense>
            </AccountProvider>
            </>
        },
        Route::NotFound => html! {
            <>
            <NavBar />
            <div class="container">
                <h1 class="title is-1">{ "Oups… Cette page n'existe pas" }</h1>
            </div>
            </>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
