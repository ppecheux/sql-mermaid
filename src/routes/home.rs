use web_sys::HtmlInputElement;

use crate::algos::mermaid;
use crate::algos::mermaid::Mermaid;
use crate::algos::{convertor::sql_mermaid, mermaid::Props};
use material_yew::{MatButton, MatTab, MatTabBar, MatTextArea};
use yew::{events::Event, function_component, html, use_mut_ref, use_state, Callback, TargetCast};
use yew_hooks::prelude::*;
/// Home page
///
/// 
#[derive(Clone, Copy)]
enum Tabs {
    Schema,
    Code,
}

#[function_component(Home)]
pub fn home() -> Html {
    let message = use_state(|| "".to_string());
    let tab = use_state(|| Tabs::Schema);

    let onchange = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
            // graph.set(Props{code : input.value()});
            log::info!("Update: {:?}", "ok");
            log::info!("Update: {:?}", sql_mermaid((*message).clone().as_str()));
        })
    };

    let on_activated = {
        let tab = tab.clone();
        Callback::from(move |index| match index {
            0 => tab.set(Tabs::Schema),
            1 => tab.set(Tabs::Code),
            num => unreachable!("{}", num),
        })
    };

    let schema_or_code = match *tab {
        Tabs::Schema => html! { <Mermaid code={ sql_mermaid((*message).clone().as_str()) } /> },
        Tabs::Code => html! { <p> { sql_mermaid((*message).clone().as_str()) } </p> },
    };

    html! {
        <div class="app">
            
                <div>
                    <div {onchange} >
                    <MatTextArea value={(*message).clone()} />
                    </div>
                    <div >
                    <MatButton label="send" />
                    </div>
                </div>
            <MatTabBar onactivated={on_activated}>
                <MatTab icon="schema" />
                <MatTab icon="code" />
            </MatTabBar>
            {schema_or_code}
        </div>
    }
}
