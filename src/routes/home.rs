use web_sys::HtmlInputElement;

use yew_hooks::prelude::*;
use yew::{
    events::Event,
    function_component, html, use_mut_ref, use_state,
    Callback, TargetCast,
};


/// Home page
///
#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_counter(0);
    let message = use_state(|| "".to_string());
    let message_count = use_mut_ref(|| 0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };

    let onchange = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };
    
    let onclick = Callback::from(move |_| {
        let window = gloo_utils::window();

        if *message_count.borrow_mut() > 3 {
            window.alert_with_message("Message limit reached").unwrap();
        } else {
            *message_count.borrow_mut() += 1;
            window.alert_with_message("Message sent").unwrap();
        }
    });

    let input_text = use_state(|| "my text".to_string());

    html! {
        <div class="app">
            <header class="app-header">
                <a
                    class="app-logo"
                    href="https://yew.rs"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                </a>
                <p>
                    { "Edit " } <code>{ "src/routes/home.rs" }</code> { " and save to reload." }
                </p>
                <a
                    id="learn_yew"
                    class="app-link"
                    href="https://yew.rs"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    { "Learn Yew" }
                </a>
                <p>
                    <button onclick={ondecrease}>{ "Decrease" }</button>
                    { *counter }
                    <button onclick={onincrease}>{ "Increase" }</button>
                </p>

                // <input {onchange} value={(*message).clone()} />

                <p> { (*input_text).clone()  } </p>
                <div>
                    <input {onchange} value={(*message).clone()} />
                    <button {onclick}>{ "Send" }</button>
                </div>
                <p> { (*message).clone()} </p>
            </header>
        </div>
    }
}
