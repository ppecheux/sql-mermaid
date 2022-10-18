use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
            // wait for yew 0.20
            // <BrowserRouter basename={dotenv!("BASENAME")}>
            <BrowserRouter>
                <Nav />
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        }
}
