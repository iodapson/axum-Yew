use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Deserialize)]
pub struct TargetData {
    pub field_one: String,
    pub field_two: i32,
    pub field_three: String,
}

#[function_component(FetchJson)]
pub fn fetch_json(target_data: &TargetData) -> Html {
    html! {
        <>
            <h2>{ "I fetched some data" }</h2>
            <p>{"field_one:"}{&target_data.field_one}</p>
            <p>{"field_two:"}{&target_data.field_two}</p>
            <p>{"field_three:"}{&target_data.field_three}</p>
        </>
    }
}
