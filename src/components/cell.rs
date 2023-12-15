use yew::prelude::*;

pub struct Cell {
    pub width: u32,
    pub height: u32,
    pub color: String
}

impl Cell {
    pub fn new(w: u32, h:u32, c: String) -> Cell {
        return Cell {
            width: 10,
            height: h,
            color: c
        }
    }
}

pub struct Cells {
    pub cells: Vec<Cell>
}

impl Cells {
    pub fn from_vec() -> Cells {
        todo!()
    }
}