use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;

use about::About;
use home::Home;

/// App routes
// wait yew 0.20 to remove pages base name
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
    // remove when yew 0.20
    #[at("/sql-mermaid/")]
    AlsoHome,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::AlsoHome => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
