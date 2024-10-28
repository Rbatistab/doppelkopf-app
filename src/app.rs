use std::fmt::Display;
use yew::prelude::*;
use yew::props;
use yew::html::IntoPropValue;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <h1>{ "Hello Game!" }</h1>
        </>
    }
}
