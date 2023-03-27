mod fetch_json;

use fetch_json::TargetData;
use gloo_net::http::Request;
//use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let some_value = "sample";
    let option_value = Some("an option");

    let state_data = use_state(|| None);
    {
        let state_data = state_data.clone();

        // fetch data and set it as 'state_data' new value
        use_effect_with_deps(
            move |_| {
                let state_data = state_data.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data: TargetData = Request::get("http://localhost:8080")
                        .send()
                        .await
                        .unwrap()
                        .json() // asumming FetchedDataType is actually Json
                        .await
                        .unwrap();
                    state_data.set(Some(fetched_data));
                });

                || ()
            },
            (),
        ); // end of 'use_effect_with_deps'
    }

    html! {
      <>

        if some_value == "sample" { //* Here, you can see conditional rendering in action
          <h1 class={some_value}>{"Hello World! I matched 'sample'."}</h1>
        } else {
          <h1 class={"invalid"}>{"Hello world! I matched 'an option'."}</h1>
        }

        if let Some(val) = option_value {
          <p>{"And variable 'option_value' has value: "}{val}</p>
        } else {
          <p>{"show no option"}</p>
        }

        <p>{"Here is the fetched_data's first field: "}</p>
        <p>{&state_data.as_ref().unwrap().field_one}</p>

      </>
    }
}

// cargo add gloo-net
// cargo add serde -F derive
// cargo add wasm-bindgen-futures

// * Inside component to hold fetched_data into state
/*
 let fetched_data_state = use_state( || None);

 let on_click_response = {
   let fetched_data_state_clone = fetched_data_state.clone();
   Callback::from(move |target_data: TargetData| {
     fetched_data_state_clone.set(Some(target_data))
   })
 }
*/
// * Still Inside component to hold fetched_data into state
/*
{
    let state_data = state_data.clone();

    // fetch data and set it as 'state_data' new value
    use effect_with_deps( move |_| {
        let state_data = state_data.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let fetched_data: FetchedDataType = Request::get("https://fetch-data-axum-endpoint/just-a-path.anything")
                .send()
                .await
                .unwrap()
                .json() // asumming FetchedDataType is actually Json
                .await
                .unwrap();
            state_data.set(fetched_data);
        });

        || ()

    }, ()); // end of 'use_effect_with_deps'
}
 */

// _____________

/* Properties
/*
  Assume the name of the yew project is 'sample_yew_project'
 */

// * INSIDE a sample module called child_component.rs
// * The component inside this module is called ChildComponent.
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub prop_one: String,
}

// * Your child-component must have a reference of the target Property struct (in this case, Prop)
// * ...as its parameter.
#[function_component(ChildComponent)]
pub fn child_component(prop: &Prop) -> Html {
    html! {
      <div>
        <p>{"I am a Child component"}
        </p>
        <p>{&prop.prop_one}{" received from a component that nested me"}
        </p>
      </div>
    }
}

// *  INSIDE a sample 'lib.rs' module
// *  The component inside this module is called ParentComponent
use yew::prelude::*;

mod child_component;

use child_component::ChildComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <>
        <div>
          <ChildComponent prop_one="prop_one value" />
        </div>
      </>
    }
}

// * INSIDE required module main.rs
// * Here you must have code to render the super parent component of your yew app
// * ...which in this case is the component inside module lib.rs
use sample_yew_project::App;

fn main() {
  yew::Renderer::<App>::new().render();
}

*/
