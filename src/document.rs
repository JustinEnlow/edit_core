//! Defines a `Document` structure, representing a text document with support for text manipulation.        //TODO: this should prob be buffer instead, to express the idea that we are working on a virtual representation of the file, and not the file itself, at least until changes are saved.
//!
//! This module contains constants, types, and functions for manipulating text within a document, including:
//! - Basic document operations like insert, delete, replace, undo, redo    //undo/redo just use insert/delete/replace also, so this may be redundant/unnecessary...
//! - Management of selections and cursor positioning
//! - Handling of different tab styles and file paths

use crate::view::View;
use crate::range::Range;
use crate::selection::{CursorSemantics, Movement, Selection, Direction};
use crate::selections::{Selections, SelectionsError};
use crate::history::{Operation, Change, ChangeSet};
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::path::PathBuf;
use ropey::Rope;



/// Represents errors that can occur when performing operations on a document.
#[derive(Debug)]
pub enum DocumentError{
    NoChangesToUndo,
    NoChangesToRedo,
    SelectionAtDocBounds,
    InvalidInput,
    SelectionsError(SelectionsError)
}
/// Holds the document instance's text, selection data, and other state like undo/redo stacks and clipboard.
pub struct Document{
    pub text: Rope, //the actual text buffer being edited
    pub file_path: Option<PathBuf>,
    pub selections: Selections, //Hashmap<ClientID, Selections>
    pub client_view: View,      //Hashmap<ClientID, View>       //TODO: client should pass view as param where needed, instead of storing this. this ensures a single source of truth (the client)
    pub undo_stack: Vec<ChangeSet>,
    pub redo_stack: Vec<ChangeSet>,
    pub last_saved_text: Rope,
    pub clipboard: String,
}
impl Document{
    ////////////////////////////////////////////////////////////////////// Testing Only ///////////////////////////////////////////////////////////////////////////
    /// Instantiate a new [`Document`]. Only for testing. Potentially also useful for opening a scratch buffer
    #[must_use] pub fn new(cursor_semantics: CursorSemantics) -> Self{
        Self::initialize_fields(None, &Rope::new(), cursor_semantics)
    }
    /// Add [Rope]-based text to an existing instance of [Document]. Only for testing. Potentially also useful for opening a scratch buffer with content from stdin
    #[must_use] pub fn with_text(mut self, text: Rope) -> Self{
        self.text = text.clone();
        self.last_saved_text = text;
        self
    }
    /// Add [Selections] to an existing instance of [Document]. Only for testing. Potentially also useful for opening a buffer with specific selection coordinates
    #[must_use] pub fn with_selections(mut self, selections: Selections) -> Self{
        self.selections = selections;
        self
    }
    /// Add a [View] to an existing instance of [Document]. Only for testing.
    #[must_use] pub fn with_view(mut self, view: View) -> Self{
        self.client_view = view;
        self
    }
    /// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.
    #[must_use] pub fn with_clipboard(mut self, clipboard: String) -> Self{
        self.clipboard = clipboard;
        self
    }
    /// Add [Vec<ChangeSet>] undo stack to an existing instance of [Document]. Only for testing.
    #[must_use] pub fn with_undo_stack(mut self, undo_stack: Vec<ChangeSet>) -> Self{
        self.undo_stack = undo_stack;
        self
    }
    /// Add `last_saved_text` to an existing instance of [Document]. Only for testing.
    #[must_use] pub fn with_last_saved_text(mut self, last_saved_text: Rope) -> Self{
        self.last_saved_text = last_saved_text;
        self
    }
    ////////////////////////////////////////////////////////////////////// Testing Only ///////////////////////////////////////////////////////////////////////////
    
    /// Opens a document from a given file path and loads its content. Supports both block and bar cursor semantics.
    pub fn open(path: &PathBuf, cursor_semantics: CursorSemantics) -> Result<Self, Box<dyn Error>>{
        let text = Rope::from_reader(BufReader::new(File::open(path)?))?;   // pass errors up

        // TODO: make text tab use match settings for USE_HARD_TAB and TAB_WIDTH

        Ok(Self::initialize_fields(Some(path.clone()), &text, cursor_semantics))
    }
    fn initialize_fields(
        file_path: Option<PathBuf>,
        text: &Rope,
        cursor_semantics: CursorSemantics,
    ) -> Self{
        let selections = match cursor_semantics{
            CursorSemantics::Bar => Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, text, cursor_semantics),
            CursorSemantics::Block => Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, text, cursor_semantics)
        };
        Self{
            text: text.clone(),
            file_path,
            selections,
            client_view: View::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_saved_text: text.clone(),
            clipboard: String::new(),
        }
    }

    // TODO: document + test
    #[must_use] pub fn file_name(&self, use_full_file_path: bool) -> Option<String>{
        match &self.file_path{
            Some(path) => {
                if use_full_file_path{
                    Some(path.to_string_lossy().to_string())
                }else{
                    Some(path.file_name().unwrap().to_string_lossy().to_string())
                }
            }
            None => None
        }
    }
    
    /// 1-based number of lines
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::CursorSemantics;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n"); //4     //technically another empty line after last newline
    /// let doc = Document::new(CursorSemantics::Bar).with_text(text.clone());
    /// assert_eq!(doc.len(), 4);
    /// ```
    #[must_use] pub fn len(&self) -> usize{
        self.text.len_lines()
    }

    //TODO: document + test
    #[must_use] pub fn is_modified(&self) -> bool{
        self.text != self.last_saved_text
    }
    
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    pub fn apply_replace(doc_text: &mut Rope, replacement_text: &str, selection: &mut Selection, semantics: CursorSemantics) -> Change{ //TODO: Error if replacement_text is empty(or if selection empty? is this possible?)
        let old_selection = selection.clone();
        let delete_change = Document::apply_delete(doc_text, selection, semantics);
        let replaced_text = if let Operation::Insert{inserted_text} = delete_change.inverse(){inserted_text}else{unreachable!();};  // inverse of delete change should always be insert
        let _ = Document::apply_insert(doc_text, replacement_text, selection, semantics);   //intentionally discard returned Change

        Change::new(Operation::Replace{replacement_text: replacement_text.to_string()}, old_selection, selection.clone(), Operation::Replace{replacement_text: replaced_text})
    }
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    pub fn apply_insert(doc_text: &mut Rope, string: &str, selection: &mut Selection, semantics: CursorSemantics) -> Change{    //TODO: Error if string is empty
        let old_selection = selection.clone();
        doc_text.insert(selection.cursor(doc_text, semantics), string);
        for _ in 0..string.len(){
            //*selection = selection.move_right(doc_text, semantics);
            //if let Ok(new_selection) = selection.move_right(doc_text, semantics){
            if let Ok(new_selection) = crate::utilities::move_cursor_right::selection_impl(selection, doc_text, semantics){
                *selection = new_selection;
            }
        }

        Change::new(Operation::Insert{inserted_text: string.to_string()}, old_selection, selection.clone(), Operation::Delete)
    }
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    pub fn apply_delete(doc_text: &mut Rope, selection: &mut Selection, semantics: CursorSemantics) -> Change{  //TODO: Error if cursor and anchor at end of text
        use std::cmp::Ordering;
        
        let old_selection = selection.clone();
        let original_text = doc_text.clone();

        let (start, end, new_cursor) = match selection.cursor(doc_text, semantics).cmp(&selection.anchor()){
            Ordering::Less => {(selection.head(), selection.anchor(), selection.cursor(doc_text, semantics))}
            Ordering::Greater => {
                match semantics{
                    CursorSemantics::Bar => {(selection.anchor(), selection.head(), selection.anchor())}
                    CursorSemantics::Block => {
                        if selection.cursor(doc_text, semantics) == doc_text.len_chars(){(selection.anchor(), selection.cursor(doc_text, semantics), selection.anchor())}
                        else{(selection.anchor(), selection.head(), selection.anchor())}
                    }
                }
            }
            Ordering::Equal => {
                if selection.cursor(doc_text, semantics) == doc_text.len_chars(){ //do nothing    //or preferrably return error   //could have condition check in calling fn
                    //return Change::new(Operation::Delete, old_selection, selection.clone(), Operation::Insert{inserted_text: "".to_string()});
                    return Change::new(Operation::Delete, old_selection, selection.clone(), Operation::Insert{inserted_text: String::new()});   //change suggested by clippy lint
                }//else{
                    match semantics{
                        CursorSemantics::Bar => {(selection.head(), selection.head().saturating_add(1), selection.anchor())}
                        CursorSemantics::Block => {(selection.anchor(), selection.head(), selection.anchor())}
                    }
                //}
            }
        };

        let change_text = original_text.slice(start..end);
        doc_text.remove(start..end);
        //*selection = selection.put_cursor(new_cursor, &original_text, Movement::Move, semantics, true);
        if let Ok(new_selection) = selection.put_cursor(new_cursor, &original_text, Movement::Move, semantics, true){
            *selection = new_selection;
        }

        Change::new(Operation::Delete, old_selection, selection.clone(), Operation::Insert{inserted_text: change_text.to_string()})
    }
    
    

    //is the below functionality actually desired?...
    //should eventually be moved into utilities
    
    //pub fn extend_selection_document_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
    //    match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_doc_start){
    //        Ok(new_selections) => {self.selections = new_selections;}
    //        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    //    }
    //    Ok(())
    //}
    //pub fn extend_selection_document_end(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
    //    match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_doc_end){
    //        Ok(new_selections) => {self.selections = new_selections;}
    //        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    //    }
    //    Ok(())
    //}
    //pub fn extend_selection_page_up(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
    //    match self.selections.move_cursor_page(&self.text, self.view(), semantics, Selection::extend_page_up){
    //        Ok(new_selections) => {self.selections = new_selections;}
    //        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    //    }
    //    Ok(())
    //}
    //pub fn extend_selection_page_down(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
    //    match self.selections.move_cursor_page(&self.text, self.view(), semantics, Selection::extend_page_down){
    //        Ok(new_selections) => {self.selections = new_selections;}
    //        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    //    }
    //    Ok(())
    //}
}
