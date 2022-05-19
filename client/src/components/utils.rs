use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">{ "Rends-moi ma thune" }<small>{ "beta" }</small></a>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" href="">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navbarBasicExample" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="/">{ "Home" }</a>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct AmountProps {
    pub amount: i64,
}

#[function_component(Amount)]
pub fn amount(AmountProps { amount }: &AmountProps) -> Html {
    html! {
        <>
        { *amount as f64 / 100f64 }{ " €" }
        </>
    }
}

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading block"></div>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct BreadcrumpProps {
    pub route: Route,
}

#[function_component(Breadcrumb)]
pub(crate) fn breadcrumb(BreadcrumpProps { route }: &BreadcrumpProps) -> Html {
    let components = match route {
        Route::Account { account_id } => Some(vec![(
            "fa-bank",
            "Compte",
            Route::Account {
                account_id: account_id.clone(),
            },
            true,
        )]),
        Route::Expenditures { account_id } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-credit-card",
                "Dépenses",
                Route::Expenditures {
                    account_id: account_id.clone(),
                },
                true,
            ),
        ]),
        Route::CreateExpenditure { account_id } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-credit-card",
                "Dépenses",
                Route::Expenditures {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-plus",
                "Nouvelle dépense",
                Route::CreateExpenditure {
                    account_id: account_id.clone(),
                },
                true,
            ),
        ]),
        Route::Repayments { account_id } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-exchange",
                "Remboursements",
                Route::Repayments {
                    account_id: account_id.clone(),
                },
                true,
            ),
        ]),
        Route::CreateRepayment { account_id } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-exchange",
                "Remboursements",
                Route::Repayments {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-plus",
                "Nouveau remboursement",
                Route::CreateRepayment {
                    account_id: account_id.clone(),
                },
                true,
            ),
        ]),
        Route::EditRepayment {
            account_id,
            repayment_id,
        } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-exchange",
                "Remboursements",
                Route::Repayments {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-pen",
                "Éditer remboursement",
                Route::EditRepayment {
                    account_id: account_id.clone(),
                    repayment_id: repayment_id.clone(),
                },
                true,
            ),
        ]),
        Route::EditExpenditure {
            account_id,
            expenditure_id,
        } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-credit-card",
                "Dépenses",
                Route::Expenditures {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-pen",
                "Éditer dépense",
                Route::EditExpenditure {
                    account_id: account_id.clone(),
                    expenditure_id: expenditure_id.clone(),
                },
                true,
            ),
        ]),
        Route::User {
            account_id,
            user_id,
        } => Some(vec![
            (
                "fa-bank",
                "Compte",
                Route::Account {
                    account_id: account_id.clone(),
                },
                false,
            ),
            (
                "fa-user",
                "Utilisateur",
                Route::User {
                    account_id: account_id.clone(),
                    user_id: user_id.clone(),
                },
                true,
            ),
        ]),
        _ => None,
    };

    if let Some(components) = components {
        html! {
            <nav class="breadcrumb mt-4" aria-label="breadcrumbs">
              <ul>
                  {
                      components.into_iter().map(|component| html! {
                        <li class={ classes!(component.3.then_some("is-active")) }>
                          <Link<Route> to={ component.2 } classes="has-text-primary">
                            <span class="icon is-small">
                              <i class={ classes!("fas", component.0) } aria-hidden="true"></i>
                            </span>
                            <span>{ component.1 }</span>
                          </Link<Route>>
                        </li>
                      }).collect::<Html>()
                  }
              </ul>
            </nav>
        }
    } else {
        html! {}
    }
}
