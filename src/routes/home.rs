use web_sys::HtmlInputElement;
use yew::{Html, use_state};

use crate::algos::convertor::{sql_s_mermaid};
use crate::algos::mermaid::Mermaid;
use material_yew::{MatButton, MatTab, MatTabBar};
use yew::{events::Event, function_component, html, Callback, TargetCast};

/// Home page
///
///
#[derive(Clone, Copy)]
enum Tabs {
    Schema,
    Code,
}

const INIT_SQL: &'static str = r#"CREATE TABLE CUSTOMERS(
    ID   INT              NOT NULL,
    NAME VARCHAR (20)     NOT NULL,
    AGE  INT              NOT NULL,
    ADDRESS  CHAR (25) ,
    SALARY   DECIMAL (18, 2),       
    PRIMARY KEY (ID)
 );
 CREATE TABLE ORDERS (
    ID          INT        NOT NULL,
    DATE        DATETIME, 
    CUSTOMER_ID INT references CUSTOMERS(ID),
    AMOUNT     double,
    PRIMARY KEY (ID)
 );
 "#;

#[function_component(Home)]
pub fn home() -> Html {
    let sql = use_state(|| INIT_SQL.to_string());
    let tab = use_state(|| Tabs::Schema);

    let onchange = {
        let sql = sql.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            sql.set(input.value());
            // log::info!("Update: {:?}", "ok");
            // log::info!("Update: {:?}", sql_mermaid((*sql).clone().as_str()));
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

    let schema_or_code = {
        let mermaid = sql_s_mermaid((*sql).clone().as_str());
        match *tab {
            Tabs::Schema => html! { <Mermaid code={ mermaid } /> },
            Tabs::Code => html! {
                <div >
                   <p style="margin-left: auto; margin-right: auto; text-align:left; max-width: 300px; align:center"> { mermaid.lines().map(
                        |row| html! { <> {row} <br/> </>}
                    ).collect::<Html>()}
                    </p>
               </div>
            },
        }
    };

    html! {
        <body>
        <div class="app">
                <div>
                    <div class="flex-container grow-area" id="grow-area" style="display: flex; flex-direction: column; min-height:300px">
                            <textarea {onchange} value={(*sql).clone()} style="flex-grow : 1" />
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
        </body>
    }
}
