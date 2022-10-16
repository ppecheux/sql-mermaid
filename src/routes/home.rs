use web_sys::HtmlInputElement;
use yew::virtual_dom::VNode;
use yew::Html;

use crate::algos::convertor::{sql_mermaid};
use crate::algos::mermaid::Mermaid;
use material_yew::{MatButton, MatTab, MatTabBar, MatTextArea};
use yew::{events::Event, function_component, html, use_state, Callback, TargetCast};

/// Home page
///
///
#[derive(Clone, Copy)]
enum Tabs {
    Schema,
    Code,
}

const INIT_SQL: &'static str = r#"CREATE TABLE "Student" (
    "StudentId" INT NOT NULL,
    "ParentId" INT NOT NULL,
    "Name" VARCHAR(30) NOT NULL,
    "Age" INT NOT NULL,
    "Address" VARCHAR(25) NOT NULL,
    "Phone" VARCHAR(20) NOT NULL,
    CONSTRAINT "PK_Student" PRIMARY KEY ("StudentId")
  );
  
  CREATE TABLE "Parent" (
    "ParentId" INT NOT NULL,
    "StudentId" INT NOT NULL,
    "PartnerId" INT NOT NULL,
    "Name" VARCHAR(30) NOT NULL,
    "Address" VARCHAR(25) NOT NULL,
    "Phone" VARCHAR(20) NOT NULL,
    CONSTRAINT "PK_Parent" PRIMARY KEY ("ParentId")
  );
  
  ALTER TABLE "Student" ADD CONSTRAINT "FK_StudentParentId"
    FOREIGN KEY ("ParentId") REFERENCES "Parent" ("ParentId");
  
  ALTER TABLE "Parent" ADD CONSTRAINT "FK_ParentStudentId"
    FOREIGN KEY ("StudentId") REFERENCES "Student" ("StudentId");
  
  ALTER TABLE "Parent" ADD CONSTRAINT "FK_ParentPartnerId"
    FOREIGN KEY ("PartnerId") REFERENCES "Parent" ("ParentId");
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
        let mermaid = sql_mermaid((*sql).clone().as_str());
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
        <div class="app">
                <div>
                    <div class="flex-container grow-area" id="grow-area" style="display: flex; flex-direction: column; min-height:300px">
                        // <div  class="flex-container"  style=" min-height: 500px; min-width:100%">
                        // <div  tyle="display: flex; flex-direction: row; ">
                    //style="min-height:550px;"
                            // <MatTextArea value={(*sql).clone()} outlined=true helper="create table sql"/>
                            <textarea {onchange} value={(*sql).clone()} style="flex-grow : 1" />
                        // </div>
                        // </div>
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
