use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let welcome = "Welcome to Doppelkopf!";

    html! {
        <div>
            <p>{ welcome }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
