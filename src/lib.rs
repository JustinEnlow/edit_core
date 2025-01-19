// prevent linter warnings for these scenarios  //this should prob be set up in its own clippy.toml config file in the crate root
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::assign_op_pattern)]    //allow x = x + y, instead of x += y

//use std::path::PathBuf;



pub mod editor;
pub mod document;
pub mod selection;
#[cfg(test)] mod selection_tests;
pub mod selection2d;
#[cfg(test)] mod selection2d_tests;
pub mod selections;
#[cfg(test)] mod selections_tests;
pub mod view;
#[cfg(test)] mod view_tests;
pub mod text_util;
#[cfg(test)] mod text_util_tests;
pub mod id;
pub mod history;



#[derive(Debug, Default, Clone/*, Copy*/)]
pub struct Position{
    x: usize,
    y: usize,
}
impl Position{
    pub fn new(x: usize, y: usize) -> Self{
        Self{x, y}
    }
    pub fn x(&self) -> usize{
        self.x
    }
    pub fn set_x(&mut self, val: usize){
        self.x = val;
    }
    pub fn y(&self) -> usize{
        self.y
    }
    pub fn set_y(&mut self, val: usize){
        self.y = val;
    }
}
impl PartialEq for Position{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position{}
