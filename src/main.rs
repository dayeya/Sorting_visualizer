use yew::prelude::*;
mod algorithm;
mod generator;

use crate::generator::generator::generate_array;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Sorting visualizer" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}