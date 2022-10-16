use yew::prelude::*;
use yew_router::prelude::*;
use material_yew::top_app_bar_fixed::{MatTopAppBarActionItems, MatTopAppBarTitle};
use material_yew::MatTopAppBarFixed;

use crate::routes::AppRoute;
type AppLink = Link<AppRoute>;
/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <MatTopAppBarFixed>
            <MatTopAppBarTitle>
                <div class="app-title">
                    <AppLink to={AppRoute::Home}>{"Create Schema"}</AppLink>
                </div>
            </MatTopAppBarTitle>
            <MatTopAppBarActionItems>
                <div class="app-title">
                    <a href="https://github.com/ppecheux/sql-mermaid">{"Github"}</a>
                </div>
            </MatTopAppBarActionItems>
        </MatTopAppBarFixed>
    }
}
