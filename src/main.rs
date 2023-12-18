mod algorithms;
mod generator;
mod components;

use std::boxed::Box;
use yew::prelude::*;
use crate::components::cell::{Cell, Array};
use crate::algorithms::sort_algorithms::insertion_sort;
use crate::generator::array_generator::generate_array;

pub enum Msg {
    Start,
    Swapped, // Used to update the array.
}

pub struct App {
    len: u32,
    min: i32,
    max: i32,
    sorted: bool,
    collection: Array,
    swap_time: i32,
}

impl App {
    fn view_cell(&self, idx: usize, cell: &Cell) -> Html {
        html! {
            <div key={idx} class={classes!("cell")}
                style={
                    format!("width: {}px; height: {}px;", cell.width, cell.height)
                }>
            // Cell data.
            { cell.height / 10 }
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let len: u32 = 50;
        let (min_element, max_element): (i32, i32) = (1, 75);
        let generated_vector: Vec<i32> = generate_array(len, min_element, max_element);
        let arr: Array = Array::from_vec(generated_vector);

        Self {
            len: len,
            min: min_element,
            max: max_element,
            sorted: false,
            collection: arr,
            swap_time: 10,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, action: Self::Message) -> bool {
        match action {
            Msg::Start => {
                insertion_sort(
                    &mut self.collection,
                    ctx.link().callback(Box::new(|_| Msg::Swapped)));
                true
            }
            Msg::Swapped => {
                // rerender the array.
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let array_view = self.collection.clone()
            .into_iter()
            .enumerate().map(|(idx, cell)| self.view_cell(idx, &cell));
        html! {
           <div class={classes!("container")}>
                <div class={classes!("ui_interactions")}>
                    <div class={classes!("header")}>
                        <h1>{"Sorting visualizer"}</h1>
                    </div>
                    <div class={classes!("settings_config")}>
                        <button class={classes!("sort_button")} onclick={ctx.link().callback(Msg::Start)}>{ "Start" }</button>
                    </div>
                </div>
                // Add the array.
                <div class={classes!("array_container")}>
                    { for array_view }
                </div>
                <div class={classes!("links")}>
                    // Insert links.
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
