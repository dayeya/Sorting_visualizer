use yew::prelude::*;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Cell {
    pub width: i32,
    pub height: i32,
    pub color: String
}

#[derive(Properties, PartialEq, Clone)]
pub struct Array {
    pub array: Vec<Cell>
}

impl Array {
    pub fn from_vec(array: Vec<i32>) -> Array {
        let converted: Vec<Cell> = array.iter().map(
            |n: &i32| {
                Cell {
                    width: 50,
                    height: *n * 10,
                    color: String::from("black")
                }
            }
        ).collect();

        // return final struct.
        Array {array: converted}
    }
    pub fn wait() -> Option<String> {
        // stops the main thread for a certain amount of time.
        Some(String::from("Hello, world!"))
    }
}

impl IntoIterator for Array {
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}