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
use std::fs::{self, File};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use ropey::Rope;
use crate::text_util;
use std::cmp::Ordering;


// should these consts be set in the frontend app, and passed in to functions that require them?


/// Specifies the display width of a tab character. This value could be adjusted based on user preferences or configuration, though there are currently no per-language settings.
pub const TAB_WIDTH: usize = 4; //should this be language dependant? on-the-fly configurable?   //TODO: consider what to do with files where the tab width already in use is different than this setting

/// Indicates whether to use hard tabs (e.g., `\t`) or spaces for indentation.
///     - If `USE_HARD_TAB` is `true`, a literal tab character (`\t`) is inserted.
///     - If `USE_HARD_TAB` is `false`, spaces are inserted, with the number of spaces determined by the `TAB_WIDTH` setting.
pub const USE_HARD_TAB: bool = false;   //maybe do enum TabStyle{Hard, Soft, Smart}

/// Determines whether the full file path or just the file name should be displayed when showing the document's name.
pub const USE_FULL_FILE_PATH: bool = false;



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
    text: Rope, //the actual text buffer being edited
    file_path: Option<PathBuf>,
    //modified: bool,
    selections: Selections, //Hashmap<ClientID, Selections>
    client_view: View,      //Hashmap<ClientID, View>       //TODO: client should pass view as param where needed, instead of storing this. this ensures a single source of truth (the client)
    undo_stack: Vec<ChangeSet>,
    redo_stack: Vec<ChangeSet>,
    last_saved_text: Rope,
    clipboard: String,
}
impl Document{
    // if possible, i would like to implement these elsewhere, and have them still be usable in other test locations
    ////////////////////////////////////////////////////////////////////// Testing Only ///////////////////////////////////////////////////////////////////////////
    /// Instantiate a new [`Document`]. Only for testing.                                                                                                        //
    /**/#[must_use] pub fn new(cursor_semantics: CursorSemantics) -> Self{                                                                                       //
    /**/    Self::initialize_fields(None, &Rope::new(), cursor_semantics)                                                                        //
    /**/}                                                                                                                                                        //
    /// Add [Rope]-based text to an existing instance of [Document]. Only for testing.                                                                           //
    /**/#[must_use] pub fn with_text(mut self, text: Rope) -> Self{                                                                                              //
    /**/    self.text = text.clone();                                                                                                                            //
    /**/    self.last_saved_text = text;                                                                                                                         //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    /// Add [Selections] to an existing instance of [Document]. Only for testing.                                                                                //
    /**/#[must_use] pub fn with_selections(mut self, selections: Selections) -> Self{                                                                            //
    /**/    self.selections = selections;                                                                                                                        //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    /// Add a [View] to an existing instance of [Document]. Only for testing.                                                                                    //
    /**/#[must_use] pub fn with_view(mut self, view: View) -> Self{                                                                                              //
    /**/    self.client_view = view;                                                                                                                             //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    /// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.       //
    /**/#[must_use] pub fn with_clipboard(mut self, clipboard: String) -> Self{                                                                                  //
    /**/    self.clipboard = clipboard;                                                                                                                          //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
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
            //CursorSemantics::Bar => Selections::new(vec![Selection::new(0, 0)], 0, text),
            CursorSemantics::Bar => Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, text),
            //CursorSemantics::Block => Selections::new(vec![Selection::new(0, 1)], 0, text),
            CursorSemantics::Block => Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, text)
        };
        Self{
            text: text.clone(),
            file_path,
            //modified: false,
            selections,
            client_view: View::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_saved_text: text.clone(),
            clipboard: String::new(),
        }
    }
    #[must_use] pub fn file_name(&self) -> Option<String>{
        match &self.file_path{
            Some(path) => {
                if USE_FULL_FILE_PATH{
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
    #[must_use] pub fn selections(&self) -> &Selections{
        &self.selections
    }
    pub fn selections_mut(&mut self) -> &mut Selections{
        &mut self.selections
    }
    #[must_use] pub fn text(&self) -> &Rope{
        &self.text
    }
    #[must_use] pub fn view(&self) -> &View{
        &self.client_view
    }
    pub fn view_mut(&mut self) -> &mut View{
        &mut self.client_view
    }
    #[must_use] pub fn clipboard(&self) -> &str{
        &self.clipboard
    }
    #[must_use] pub fn undo_stack(&self) -> Vec<ChangeSet>{ //should this be &Vec?
        self.undo_stack.clone()
    }
    #[must_use] pub fn redo_stack(&self) -> Vec<ChangeSet>{ //should this be &Vec?
        self.redo_stack.clone()
    }
    /// Saves the document's content to its file path.
    pub fn save(&mut self) -> Result<(), Box<dyn Error>>{
        if let Some(path) = &self.file_path{ // does nothing if path is None    //maybe return Err(()) instead?
            self.text.write_to(BufWriter::new(fs::File::create(path)?))?;
            //self.modified = false;
            self.last_saved_text = self.text.clone();
        }
        
        Ok(())
    }
    #[must_use] pub fn is_modified(&self) -> bool{
        //self.modified
        self.text != self.last_saved_text
    }
    
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    fn apply_replace(doc_text: &mut Rope, replacement_text: &str, selection: &mut Selection, semantics: CursorSemantics) -> Change{ //TODO: Error if replacement_text is empty(or if selection empty? is this possible?)
        let old_selection = selection.clone();
        let delete_change = Document::apply_delete(doc_text, selection, semantics);
        let replaced_text = if let Operation::Insert{inserted_text} = delete_change.inverse(){inserted_text}else{unreachable!();};  // inverse of delete change should always be insert
        let _ = Document::apply_insert(doc_text, replacement_text, selection, semantics);   //intentionally discard returned Change

        Change::new(Operation::Replace{replacement_text: replacement_text.to_string()}, old_selection, selection.clone(), Operation::Replace{replacement_text: replaced_text})
    }
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    fn apply_insert(doc_text: &mut Rope, string: &str, selection: &mut Selection, semantics: CursorSemantics) -> Change{    //TODO: Error if string is empty
        let old_selection = selection.clone();
        doc_text.insert(selection.cursor(doc_text, semantics), string);
        for _ in 0..string.len(){
            //*selection = selection.move_right(doc_text, semantics);
            if let Ok(new_selection) = selection.move_right(doc_text, semantics){
                *selection = new_selection;
            }
        }

        Change::new(Operation::Insert{inserted_text: string.to_string()}, old_selection, selection.clone(), Operation::Delete)
    }
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    fn apply_delete(doc_text: &mut Rope, selection: &mut Selection, semantics: CursorSemantics) -> Change{  //TODO: Error if cursor and anchor at end of text
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

    /// Reverts the last set of changes made to the document.
    pub fn undo(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{    //should this be a HistoryError instead?...
        // Check if there is something to undo
        if let Some(change_set) = self.undo_stack.pop(){
            let changes = change_set.changes();
            
            *self.selections_mut() = change_set.clone().selections_after_changes();    //set selections to selections_after_changes to account for any selection movements that may have occurred since edit
            assert!(self.selections.count() == changes.len());

            for (i, change) in changes.iter().enumerate().take(self.selections.count()){
                let selection = self.selections.nth_mut(i);
                match change.operation(){
                    Operation::Insert{inserted_text} => {
                        selection.shift_and_extend(inserted_text.len(), &self.text, semantics);
                        let _ = Document::apply_delete(&mut self.text, selection, semantics);
                        self.selections.shift_subsequent_selections_backward(i, inserted_text.len());
                    }
                    Operation::Delete => {
                        if let Operation::Insert{inserted_text} = change.inverse(){
                            let _ = Document::apply_insert(&mut self.text, &inserted_text, selection, semantics);   //apply inverse operation
                            self.selections.shift_subsequent_selections_forward(i, inserted_text.len());
                        }
                    }
                    Operation::Replace{replacement_text} => {
                        let inserted_text = replacement_text;
                        if let Operation::Replace{replacement_text} = change.inverse(){
                            selection.shift_and_extend(inserted_text.len(), &self.text, semantics);
                            let _ = Document::apply_replace(&mut self.text, &replacement_text, selection, semantics);
                            match inserted_text.len().cmp(&replacement_text.len()){    //old selected text vs new text
                                Ordering::Greater => {self.selections.shift_subsequent_selections_backward(i, inserted_text.len().saturating_sub(replacement_text.len()));}
                                Ordering::Less => {self.selections.shift_subsequent_selections_forward(i, replacement_text.len().saturating_sub(inserted_text.len()));}
                                Ordering::Equal => {}   // no change to subsequent selections
                            }
                        }
                    }
                }
            }
            // selections should be the same as they were before changes were made, because we are restoring that previous state
            *self.selections_mut() = change_set.selections_before_changes();

            // Push inverted changes onto redo stack
            self.redo_stack.push(change_set);

            Ok(())
        }else{Err(DocumentError::NoChangesToUndo)}
    }

    /// Re-applies the last undone changes to the document.
    // Make sure to clear the redo stack in every edit fn. new actions invalidate the redo history
    pub fn redo(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{    //should this be HistoryError instead?...
        // Check if there is something to redo
        if let Some(change_set) = self.redo_stack.pop(){
            let changes = change_set.changes();

            *self.selections_mut() = change_set.clone().selections_before_changes();    //set selections to selections_before_changes to account for any selection movements that may have occurred since undo
            assert!(self.selections.count() == changes.len());   //num selections should match num changes

            for (i, change) in changes.iter().enumerate().take(self.selections.count()){
                let selection = self.selections.nth_mut(i);
                match change.operation(){
                    Operation::Insert{inserted_text} => {
                        let _ = Document::apply_insert(&mut self.text, &inserted_text, selection, semantics);
                        self.selections.shift_subsequent_selections_forward(i, inserted_text.len());
                    }
                    Operation::Delete => {
                        *selection = change.selection_before_change();
                        let change = Document::apply_delete(&mut self.text, selection, semantics);
                        if let Operation::Insert{inserted_text} = change.inverse(){
                            self.selections.shift_subsequent_selections_backward(i, inserted_text.len());
                        }
                    }
                    Operation::Replace{replacement_text} => {
                        let inserted_text = replacement_text;
                        let change = Document::apply_replace(&mut self.text, &inserted_text, selection, semantics);
                        if let Operation::Replace{replacement_text} = change.inverse(){   //destructure to get currently selected text
                            match replacement_text.len().cmp(&inserted_text.len()){    //old selected text vs new text
                                Ordering::Greater => {self.selections.shift_subsequent_selections_backward(i, replacement_text.len().saturating_sub(inserted_text.len()));}
                                Ordering::Less => {self.selections.shift_subsequent_selections_forward(i, inserted_text.len().saturating_sub(replacement_text.len()));}
                                Ordering::Equal => {}   // no change to subsequent selections
                            }
                        }
                    }
                }
            }
            assert!(self.selections == change_set.clone().selections_after_changes());

            // Push changes back onto the undo stack
            self.undo_stack.push(change_set);

            Ok(())
        }else{Err(DocumentError::NoChangesToRedo)}
    }

    /// Inserts provided string into text at each selection.
    pub fn insert_string(&mut self, string: &str, semantics: CursorSemantics) -> Result<(), DocumentError>{
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

        if string.is_empty(){return Err(DocumentError::InvalidInput);}

        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            let change = match string{
                //"\n" => {}    //handle behavior specific to pressing "enter". auto-indent, etc... //TODO: create tests for newline behavior...
                "\t" => {   //handle behavior specific to pressing "tab".
                    if USE_HARD_TAB{
                        if selection.is_extended(semantics){self.handle_insert_replace(i, semantics, "\t")}
                        else{self.handle_insert("\t", i, semantics)}
                    }
                    else{
                        let tab_distance = text_util::distance_to_next_multiple_of_tab_width(selection, &self.text, semantics);
                        let modified_tab_width = if tab_distance > 0 && tab_distance < TAB_WIDTH{tab_distance}else{TAB_WIDTH};
                        let soft_tab = " ".repeat(modified_tab_width);

                        if selection.is_extended(semantics){self.handle_insert_replace(i, semantics, &soft_tab)}
                        else{self.handle_insert(&soft_tab, i, semantics)}
                    }
                }
                //handle any other inserted string
                _ => {
                    if selection.is_extended(semantics){self.handle_insert_replace(i, semantics, string)}
                    else{self.handle_insert(string, i, semantics)}
                }
            };

            changes.push(change);
        }

        // push change set to undo stack
        self.undo_stack.push(ChangeSet::new(changes, selections_before_changes, self.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        Ok(())
    }
    fn handle_insert_replace(&mut self, current_selection_index: usize, semantics: CursorSemantics, new_text: &str) -> Change{
        let selection = self.selections.nth_mut(current_selection_index);
        let change = Document::apply_replace(&mut self.text, new_text, selection, semantics);
        if let Operation::Replace{replacement_text} = change.inverse(){
            match replacement_text.len().cmp(&new_text.len()){    //old selected text vs new text
                Ordering::Greater => {self.selections.shift_subsequent_selections_backward(current_selection_index, replacement_text.len().saturating_sub(new_text.len()));}
                Ordering::Less => {self.selections.shift_subsequent_selections_forward(current_selection_index, new_text.len().saturating_sub(replacement_text.len()));}
                Ordering::Equal => {}   // no change to subsequent selections
            }
        }
        change
    }
    fn handle_insert(&mut self, string: &str, current_selection_index: usize, semantics: CursorSemantics) -> Change{
        let selection = self.selections.nth_mut(current_selection_index);
        let change = Document::apply_insert(&mut self.text, string, selection, semantics);
        self.selections.shift_subsequent_selections_forward(current_selection_index, string.len());
        change
    }

    // Can delete and backspace be combined? pub fn delete(&mut self, direction: Direction, semantics: CursorSemantics)
    // if selection.is_extended() || direction == Direction::Forward, apply delete, else use backspace code

    /// Deletes text inside each [`Selection`] in [`Selections`], or if [`Selection`] not extended, the next character, and pushes changes to undo stack.
    pub fn delete(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

        // if any selection errors, don't allow deletion for any other...
        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            if !selection.is_extended(semantics) && selection.cursor(&self.text, semantics) >= self.text.len_chars(){return Err(DocumentError::SelectionAtDocBounds);}
        }

        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            let change = Document::apply_delete(&mut self.text, selection, semantics);
            if let Operation::Insert{inserted_text} = change.inverse(){
                self.selections.shift_subsequent_selections_backward(i, inserted_text.len());
            }
            changes.push(change);
        }

        // push change set to undo stack
        self.undo_stack.push(ChangeSet::new(changes, selections_before_changes, self.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        Ok(())
    }

    /// Deletes the previous character, or deletes selection if extended.
    /// #### Invariants:
    /// - will not delete past start of doc
    /// - at start of line, appends current line to end of previous line
    /// - removes previous soft tab, if `TAB_WIDTH` spaces are before cursor
    /// - deletes selection if selection extended
    pub fn backspace(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

        // if any selection errors, don't allow deletion for any other...
        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            if !selection.is_extended(semantics) && selection.cursor(&self.text, semantics) == 0{return Err(DocumentError::SelectionAtDocBounds);}
        }

        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            if selection.is_extended(semantics){
                let change = Document::apply_delete(&mut self.text, selection, semantics);
                if let Operation::Insert{inserted_text} = change.inverse(){
                    self.selections.shift_subsequent_selections_backward(i, inserted_text.len());
                }
                changes.push(change);
            }else{
                let offset_from_line_start = text_util::offset_from_line_start(selection.cursor(&self.text, semantics), &self.text);
                let line = self.text.line(self.text.char_to_line(selection.cursor(&self.text, semantics)));
                let is_deletable_soft_tab = !USE_HARD_TAB && offset_from_line_start >= TAB_WIDTH
                // handles case where user adds a space after a tab, and wants to delete only the space
                && offset_from_line_start % TAB_WIDTH == 0
                // if previous 4 chars are spaces, delete 4. otherwise, use default behavior
                && text_util::slice_is_all_spaces(line.slice(offset_from_line_start.saturating_sub(TAB_WIDTH)..offset_from_line_start));

                if is_deletable_soft_tab{
                    selection.shift_and_extend(TAB_WIDTH, &self.text, semantics);
                    changes.push(Document::apply_delete(&mut self.text, selection, semantics));
                    self.selections.shift_subsequent_selections_backward(i, TAB_WIDTH);
                }
                //else if selection.cursor(semantics) > 0{
                else{
                    //*selection = selection.move_left(&self.text, semantics);
                    if let Ok(new_selection) = selection.move_left(&self.text, semantics){
                        *selection = new_selection;
                    }   //TODO: handle error    //first for loop guarantees no selection is at doc bounds, so this should be ok to ignore...
                    changes.push(Document::apply_delete(&mut self.text, selection, semantics));
                    self.selections.shift_subsequent_selections_backward(i, 1);
                }
            }
        }

        // push changes to undo stack
        self.undo_stack.push(ChangeSet::new(changes, selections_before_changes, self.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        Ok(())
    }

    /// Cut single selection.
    /// Copies text to clipboard and removes selected text from document.
    /// Ensure single selection when calling this function.
    pub fn cut(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        if self.selections.count() > 1{return Err(DocumentError::SelectionsError(SelectionsError::MultipleSelections));}

        let selection = self.selections.primary_mut();
        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.range.start..selection.range.end).to_string();
        self.delete(semantics)  //notice this is returning the result from delete
    }

    /// Copy single selection to clipboard.
    /// Ensure single selection when calling this function.
    pub fn copy(&mut self) -> Result<(), DocumentError>{
        if self.selections.count() > 1{return Err(DocumentError::SelectionsError(SelectionsError::MultipleSelections));}
        
        let selection = self.selections.primary().clone();
        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.range.start..selection.range.end).to_string();

        Ok(())
    }

    /// Insert clipboard contents at cursor position(s).
    pub fn paste(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        self.insert_string(&self.clipboard.clone(), semantics)
    }

    //TODO: impl fn move_selected_text_up       //swap selected text with line above
    //TODO: impl fn move_selected_text_down     //swap selected text with line below

    //TODO: make pub fn align_selected_text_vertically
    //TODO: make pub fn rotate_selected_text

    //TODO: make pub fn add_surrounding_pair
    ///
    /// ```
    /// use ropey::Rope;
    /// use edit_core::document::Document;
    /// use edit_core::range::Range;
    /// use edit_core::selection::{Selection, CursorSemantics, Direction};
    /// use edit_core::selections::Selections;
    /// 
    /// let text = Rope::from("some\nshit\n");
    /// let semantics = CursorSemantics::Block;
    /// let mut doc = Document::new(semantics)
    ///     .with_text(text.clone())
    ///     .with_selections(
    ///         Selections::new(
    ///             vec![
    ///                 Selection::new(Range::new(0, 2/*1*/), Direction::Forward),
    ///                 Selection::new(Range::new(5, 7/*6*/), Direction::Forward)
    ///             ], 
    ///             0, 
    ///             &text
    ///         )
    ///     );
    /// let _ = doc.add_surrounding_pair('{', '}');
    ///// assert_eq!("{s}ome\n{s}hit\n", doc.text());
    /// assert_eq!("{so}me\n{sh}it\n", doc.text());
    ///// assert_eq!(
    /////     &Selections::new(
    /////         vec![
    /////             Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1),
    /////             Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 1)
    /////         ], 
    /////         0, 
    /////         &text
    /////     ),
    /////     doc.selections()
    ///// );
    ///// assert!(doc.is_modified());
    /// ```
    pub fn add_surrounding_pair(&mut self, leading_char: char, trailing_char: char) -> Result<(), DocumentError>{
        //how can goal behavior be accomplished?...
        //we could just replace each selection with its text contents + leading and trailing char added
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();
        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            let mut contents = selection.contents_as_string(&self.text);
            contents.insert(0, leading_char);
            contents.push_str(&trailing_char.to_string());
            let change = Document::apply_replace(&mut self.text, &contents, selection, CursorSemantics::Block);
            changes.push(change);
            self.selections.shift_subsequent_selections_forward(i, 2);    //can't borrow as mutable
        }
        // push change set to undo stack
        self.undo_stack.push(ChangeSet::new(changes, selections_before_changes, self.selections.clone()));
        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();
        Ok(())
    }

    //TODO: impl movement fns
    //pub fn flip_selections_direction(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
    //    match self.selections.move_cursor_non_overlapping(&self.text, semantics, Selection::flip_direction){
    //        Ok(new_selections) => {self.selections = new_selections;}
    //        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    //    }
    //    Ok(())
    //}
    pub fn move_to_line_number(&mut self, line_number: usize, semantics: CursorSemantics) -> Result<(), DocumentError>{
        //assert!(line_number > 0);
        //let line_number = line_number.saturating_sub(1);    //convert to zero based //should this conversion really be happening on the back end?
        if line_number >= self.text.len_lines(){return Err(DocumentError::InvalidInput);}
        
        if let Ok(()) = self.clear_non_primary_selections(){};
        match self.selections.primary().set_from_line_number(line_number, &self.text, Movement::Move, semantics){
            Ok(new_selection) => {*self.selections_mut().primary_mut() = new_selection;}
            Err(_) => {return Err(DocumentError::InvalidInput);}    //should be same state error
        }
        Ok(())
    }
    pub fn move_cursor_up(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_up){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_down(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_down){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_left(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_left){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_right(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_right){
        //match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, crate::selection::movement::move_right){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_word_boundary_forward(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_right_word_boundary){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_word_boundary_backward(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_left_word_boundary){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_line_end(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_line_text_end){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_line_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_line_start){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_line_text_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_line_text_start){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_home(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::move_home){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_document_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_clearing_non_primary(&self.text, semantics, Selection::move_doc_start){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_document_end(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_clearing_non_primary(&self.text, semantics, Selection::move_doc_end){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    //TODO: maybe functions that use self.view() should take an area(height + width) instead?...
    pub fn move_cursor_page_up(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_page(&self.text, self.view(), semantics, Selection::move_page_up){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn move_cursor_page_down(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_page(&self.text, self.view(), semantics, Selection::move_page_down){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    
    
    
    pub fn extend_selection_up(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_up){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_down(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_down){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_left(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_left){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_right(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_right){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_word_boundary_backward(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_left_word_boundary){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_word_boundary_forward(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_right_word_boundary){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_line_end(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_line_text_end){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_line_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_line_start){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_line_text_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_line_text_start){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_home(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_home){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_document_start(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_doc_start){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_document_end(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::extend_doc_end){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_page_up(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_page(&self.text, self.view(), semantics, Selection::extend_page_up){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn extend_selection_page_down(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_page(&self.text, self.view(), semantics, Selection::extend_page_down){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn select_line(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_potentially_overlapping(&self.text, semantics, Selection::select_line){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }

    pub fn select_all(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_clearing_non_primary(&self.text, semantics, Selection::select_all){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }

    pub fn collapse_selections(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.move_cursor_non_overlapping(&self.text, semantics, Selection::collapse){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }

    pub fn clear_non_primary_selections(&mut self) -> Result<(), DocumentError>{
        match self.selections.clear_non_primary_selections(){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn add_selection_above(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.add_selection_above(&self.text, semantics){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn add_selection_below(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.add_selection_below(&self.text, semantics){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn remove_primary_selection(&mut self) -> Result<(), DocumentError>{
        match self.selections.remove_primary_selection(){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn increment_primary_selection(&mut self) -> Result<(), DocumentError>{
        match self.selections.increment_primary_selection(){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn decrement_primary_selection(&mut self) -> Result<(), DocumentError>{
        match self.selections.decrement_primary_selection(){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
        }
        Ok(())
    }
    pub fn surround(&mut self) -> Result<(), DocumentError>{
        match self.selections.surround(&self.text){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e));}
        }
        Ok(())
    }
    pub fn nearest_surrounding_pair(&mut self, semantics: CursorSemantics) -> Result<(), DocumentError>{
        match self.selections.nearest_surrounding_pair(&self.text, semantics){
            Ok(new_selections) => {self.selections = new_selections;}
            Err(e) => {return Err(DocumentError::SelectionsError(e));}
        }
        Ok(())
    }
}
