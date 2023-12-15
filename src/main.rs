use yew::prelude::*;
mod algorithms;
mod generator;
mod components;

use crate::generator::generator::generate_array;

#[function_component(Array)]
fn full_array(Cells { cells }: &Cells) -> Html {
    // list of html objects from cells.
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Sorting visualizer" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}