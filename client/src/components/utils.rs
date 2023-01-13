use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct NavBarProps {
    #[prop_or_default]
    pub account_id: Option<String>,
}

pub(crate) enum NavBarMsg {
    Toogle,
    Hide,
}

pub(crate) struct NavBar {
    _listener: LocationHandle,
    menu_visible: bool,
}

impl Component for NavBar {
    type Message = NavBarMsg;
    type Properties = NavBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let listener = ctx
            .link()
            .add_location_listener(ctx.link().callback(|_| NavBarMsg::Hide))
            .unwrap();

        Self {
            _listener: listener,
            menu_visible: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavBarMsg::Toogle => {
                self.menu_visible = !self.menu_visible;
                true
            }
            NavBarMsg::Hide => {
                self.menu_visible = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let click_burger = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            NavBarMsg::Toogle
        });
        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">{ "Rends-moi ma thune" }<small>{ "beta" }</small></a>

                    if ctx.props().account_id.is_some() {
                        <a role="button" class={ classes!("navbar-burger", self.menu_visible.then_some("is-active")) } aria-label="menu" aria-expanded="false" data-target="navbar" onclick={ click_burger } href="">
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    }
                </div>

                if let Some(account_id) = ctx.props().account_id.as_ref() {
                    <div class={ classes!("navbar-menu", self.menu_visible.then_some("is-active")) } id="navbar">
                        <div class="navbar-start">
                            <Link<Route> to={ Route::Account { account_id: account_id.clone() } } classes="navbar-item">
                                { "Compte" }
                            </Link<Route>>
                            <div class="navbar-item has-dropdown is-hoverable">
                                <Link<Route> to={ Route::Expenditures { account_id: account_id.clone() } } classes="navbar-link">
                                    { "Dépenses" }
                                </Link<Route>>
                                <div class="navbar-dropdown">
                                    <Link<Route> to={ Route::CreateExpenditure { account_id: account_id.clone() } } classes="navbar-item">
                                        { "Ajouter" }
                                    </Link<Route>>
                                </div>
                            </div>
                            <div class="navbar-item has-dropdown is-hoverable">
                                <Link<Route> to={ Route::Repayments { account_id: account_id.clone() } } classes="navbar-link">
                                    { "Remboursements" }
                                </Link<Route>>
                                <div class="navbar-dropdown">
                                    <Link<Route> to={ Route::CreateRepayment { account_id: account_id.clone() } } classes="navbar-item">
                                        { "Ajouter" }
                                    </Link<Route>>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            </nav>
        }
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
            <nav class="breadcrumb" aria-label="breadcrumbs">
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
