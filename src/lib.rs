//! This library implements the core, non-graphical logic for a simple text editor on Linux.
//!
//! This crate provides the essential building blocks for text manipulation, file handling, and
//! other editor functionality. It is designed to be used as the backend for a text editor,
//! and can be integrated with various graphical frontends or used in command-line applications.
//!
//! ### Features
//! - **Selection Manipulation**: Provides operations such as selection extension, addition, and removal, 
//!     as well as operations for cursor movement. The `Selections` structure provides an API to handle 
//!     these operations, with suport for handling multiple selections simultaneously.
//! 
//! - **Text Manipulation**: Allows basic editing operations such as inserting, deleting, and replacing text.
//!     The `Document` structure provides an API to handle these operations, with support for handling
//!     multiple edits simultaneously.
//!
//! - **Configuration**: The editor supports customizable behavior. You can configure whether
//!     tabs should be hard tabs (`\t`) or soft tabs (spaces) and set the width of soft tabs using the
//!     `TAB_WIDTH` constant, or whether to use Bar or Block cursor semantics.
//!
//! - **File I/O**: The library includes functionality for loading and saving text files.
//!
//! ### Example Usage
//! Below is an example of how to use the core functionality of this library:
//! ```rust
//! use edit_core::{Document, DocumentError, CursorSemantics};
//!
//! // Open a document from a file
//! let mut doc = Document::open("example.txt", CursorSemantics::Bar).expect("Failed to open file");
//!
//! // Insert text into the document at the current cursor position
//! doc.insert_string("Hello, World!", CursorSemantics::Bar).expect("Failed to insert text");
//!
//! // Save the document back to the file
//! doc.save().expect("Failed to save file");
//!
//! // Perform an undo operation
//! doc.undo(CursorSemantics::Bar).expect("Failed to undo");
//!
//! // Perform a redo operation
//! doc.redo(CursorSemantics::Bar).expect("Failed to redo");
//! ```
//!
//! ### Future Enhancements
//!     - [ ] idk...



// prevent linter warnings for these scenarios  //this should prob be set up in its own clippy.toml config file in the crate root
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::assign_op_pattern)]    //allow x = x + y, instead of x += y
#![allow(clippy::match_same_arms)]  //idk, double check if we want this one...
#![allow(clippy::missing_errors_doc)]   //idk, double check if we want this one...
#![allow(clippy::missing_panics_doc)]   //idk, double check if we want this one...



pub mod id;
pub mod editor;
pub mod history;
pub mod document;
#[cfg(test)] mod document_tests;
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



#[derive(Debug, Default, Clone)]    //TODO: move position into its own module
pub struct Position{
    pub x: usize,
    pub y: usize,
}
impl Position{
    #[must_use] pub fn new(x: usize, y: usize) -> Self{
        Self{x, y}
    }
}
impl PartialEq for Position{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position{}
