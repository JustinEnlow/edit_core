// prevent linter warnings for these scenarios  //this should prob be set up in its own clippy.toml config file in the crate root
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::assign_op_pattern)]    //allow x = x + y, instead of x += y

//use std::path::PathBuf;



pub mod editor;
pub mod document;
pub mod range;
#[cfg(test)] mod range_tests;
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



#[derive(Debug, Default, Clone)]
pub struct Position{
    pub x: usize,
    pub y: usize,
}
impl Position{
    pub fn new(x: usize, y: usize) -> Self{
        Self{x, y}
    }
}
impl PartialEq for Position{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position{}
