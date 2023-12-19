use yew::prelude::*;
use wasm_bindgen::prelude::*;
use std::ops::{Index, IndexMut};
use std::thread;
use std::time::Duration;

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Cell {
    pub height: i32,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Array {
    pub array: Vec<Cell>
}

impl Array {
    pub fn from_vec(array: Vec<i32>) -> Array {
        let converted: Vec<Cell> = array.iter().map(|n: &i32| {
            Cell {
                height: *n,
            }
        }).collect();

        // return final struct.
        Array {array: converted}
    }

    pub fn swap(&mut self, k: usize, v: usize) {
        self.array.swap(k, v)
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }
}

impl Index<usize> for Array {
    type Output = i32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.array[idx].height
    }
}

impl IndexMut<usize> for Array {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.array[idx].height
    }
}

impl IntoIterator for Array {
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}