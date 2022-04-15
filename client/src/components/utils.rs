use yew::prelude::*;

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
    pub amount: i32,
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
        <div class="loading">
            { "Loading..." }
        </div>
    }
}
