//TODO: edits should be handled in reverse_selection_order. this ensures that edits at selection position
//in rope do not effect subsequent selection positions
// e.g. "ab[]cd[]efg" insert char x
    //if not reversed, would result in "abx[]c[]xdefg" because second selection position remains at position 4 in rope
    // if reversed, would result in "abx[]cdx[]efg" because previous selection positions arent effected by later insertions
//we also need to ensure selections are sorted by position/index on the rope. and overlapping selections
//are combined into a single selection

use crate::view::View;
use crate::selection::{CursorSemantics, Movement, Selection, Selections};
use crate::history::{Change, ChangeSet, Operation};
use std::fs::{self, File};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use ropey::Rope;
use crate::text_util;

// tab keypress inserts the number of spaces specified in TAB_WIDTH into the focused document
pub const TAB_WIDTH: usize = 4; //should this be language dependant? on-the-fly configurable?
// whether to use full file path or just file name
pub const USE_FULL_FILE_PATH: bool = false;



pub struct Document{
    text: Rope,
    file_path: Option<PathBuf>,
    modified: bool,
    selections: Selections, //Hashmap<ClientID, Selections>
    client_view: View,      //Hashmap<ClientID, View>
    undo_stack: Vec<ChangeSet>,
    redo_stack: Vec<ChangeSet>,
    last_saved_text: Rope,
    clipboard: String,
}
impl Document{
    pub fn open(path: &PathBuf, cursor_semantics: CursorSemantics) -> Result<Self, Box<dyn Error>>{
        let text = Rope::from_reader(BufReader::new(File::open(path)?))?;

        Ok(Self::initialize_fields(Some(path.clone()), text, cursor_semantics))
    }
    pub fn new(cursor_semantics: CursorSemantics) -> Self{
        Self::initialize_fields(None, Rope::new(), cursor_semantics)
    }
    fn initialize_fields(
        file_path: Option<PathBuf>,
        text: Rope,
        cursor_semantics: CursorSemantics,
    ) -> Self{
        let selections = match cursor_semantics{
            CursorSemantics::Bar => Selections::new(vec![Selection::new(0, 0)], 0, &text),
            CursorSemantics::Block => Selections::new(vec![Selection::new(0, 1)], 0, &text),
        };
        Self{
            text: text.clone(),
            file_path,
            modified: false,
            selections,
            client_view: View::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_saved_text: text.clone(),
            clipboard: String::new(),
        }
    }
    /// Add [Rope]-based text to an existing instance of [Document]. Only for testing.
    pub fn with_text(mut self, text: Rope) -> Self{
        self.text = text.clone();
        self.last_saved_text = text;
        self
    }
    /// Add [Selections] to an existing instance of [Document]. Only for testing.
    pub fn with_selections(mut self, selections: Selections) -> Self{
        self.selections = selections;
        self
    }
    /// Add a [View] to an existing instance of [Document]. Only for testing.
    pub fn with_view(mut self, view: View) -> Self{
        self.client_view = view;
        self
    }
    /// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.
    pub fn with_clipboard(mut self, clipboard: String) -> Self{
        self.clipboard = clipboard;
        self
    }
    pub fn file_name(&self) -> Option<String>{
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
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize{
        self.text.len_lines()
    }
    pub fn selections(&self) -> &Selections{
        &self.selections
    }
    pub fn selections_mut(&mut self) -> &mut Selections{
        &mut self.selections
    }
    pub fn text(&self) -> &Rope{
        &self.text
    }
    pub fn view(&self) -> &View{
        &self.client_view
    }
    pub fn view_mut(&mut self) -> &mut View{
        &mut self.client_view
    }
    pub fn clipboard(&self) -> &str{
        &self.clipboard
    }
    pub fn undo_stack(&self) -> Vec<ChangeSet>{
        self.undo_stack.clone()
    }
    pub fn redo_stack(&self) -> Vec<ChangeSet>{
        self.redo_stack.clone()
    }
    pub fn save(&mut self) -> Result<(), Box<dyn Error>>{
        if let Some(path) = &self.file_path{ // does nothing if path is None
            self.text.write_to(BufWriter::new(fs::File::create(path)?))?;
            
            self.modified = false;
            self.last_saved_text = self.text.clone();
        }
        
        Ok(())
    }
    pub fn is_modified(&self) -> bool{
        //self.modified
        self.text != self.last_saved_text
    }

    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// # use edit_core::history::{Change, ChangeSet, Operation};
    /// 
    ///// let text = Rope::from("idk\nshit\n");
    ///// let mut document = Document::new(CursorSemantics::Bar).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(4, 4)], 0, &text));
    ///// document.insert_string("some\n", CursorSemantics::Bar);
    ///// assert_eq!("idk\nsome\nshit\n", document.text().clone());
    ///// assert_eq!(vec![ChangeSet::new(vec![Change::new(Operation::Insert, "some\n".to_string(), Selection::new(4, 4), Selection::with_stored_line_position(9, 9, 0))])], document.undo_stack());
    ///// document.undo(CursorSemantics::Bar);
    ///// assert_eq!(vec![ChangeSet::new(vec![Change::new(Operation::Insert, "some\n".to_string(), Selection::new(4, 4), Selection::with_stored_line_position(9, 9, 0))])], document.redo_stack());
    ///// //assert_eq!(vec![ChangeSet::new(vec![Change::new(Operation::Delete, "some\n".to_string(), Selection::new(9, 9), Selection::new(4, 4))])], document.undo_stack());
    ///// //assert_eq!(vec![ChangeSet::new(vec![Change::new(Operation::Delete, "some\n".to_string(), Selection::new(4, 9), Selection::new(4, 4))])], document.undo_stack());
    ///// assert_eq!("idk\nshit\n", document.text().clone());
    /// 
    /// // basic usage with successful undo
    /// //let text = Rope::from("some\nshit\n");
    /// //let mut doc = Document::new(CursorSemantics::Bar).with_text(text.clone());
    /// let mut doc = Document::new(CursorSemantics::Bar).with_text(Rope::from("some\nshit\n"));
    /// doc.insert_string("idk\n", CursorSemantics::Bar);
    /// assert_eq!("idk\nsome\nshit\n", doc.text());
    /// assert!(doc.is_modified());
    /// doc.undo(CursorSemantics::Bar);
    /// assert_eq!("some\nshit\n", doc.text());
    /// assert!(!doc.is_modified());
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut doc = Document::new(CursorSemantics::Bar).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(4, 9)], 0, &text));
    /// doc.delete(CursorSemantics::Bar);
    /// assert_eq!("idk\nshit\n", doc.text());
    /// assert!(doc.is_modified());
    /// doc.undo(CursorSemantics::Bar);
    /// assert_eq!("idk\nsome\nshit\n", doc.text());
    /// assert!(!doc.is_modified());
    /// 
    /// // attempting to undo with no changes
    /// let mut doc = Document::new(CursorSemantics::Bar);
    /// assert!(doc.undo(CursorSemantics::Bar).is_err());
    /// ```
    pub fn undo(&mut self, semantics: CursorSemantics) -> Result<(), ()>{
        // Check if there is something to undo
        if let Some(prev_changes) = self.undo_stack.pop(){
            let new_changes = prev_changes.invert();

            // perform inverse operation, restoring previous state
            let mut new_selections = Vec::new();
            for change in new_changes{
                match change.operation(){
                    Operation::Insert => {
                        let (new_text, new_change) = Document::insert_string_single_selection(&change.old_selection(), &self.text, &change.text(), semantics);
                        self.text = new_text;
                        new_selections.push(new_change.new_selection());
                    }
                    Operation::Delete => {
                        let (new_text, new_change) = Document::delete_single_selection(&change.old_selection(), &self.text, semantics);
                        self.text = new_text;
                        new_selections.push(new_change.new_selection());
                    }
                }
            }
            let selections = Selections::new(new_selections, self.selections.primary_selection_index(), &self.text);
            *self.selections_mut() = selections;
            
            // Push popped change set onto redo stack
            self.redo_stack.push(prev_changes);

            // update document modified status
            //self.modified = self.text != self.last_saved_text;
            
            Ok(())
        }else{
            Err(())
        }
    }

    /// Re-applies a ChangeSet from the redo stack.
    /// Make sure to clear the redo stack in every edit fn. new actions invalidate the redo history
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// # use edit_core::history::{Change, ChangeSet, Operation};
    /// 
    /// // basic usage with successful redo
    /// let mut doc = Document::new(CursorSemantics::Bar).with_text(Rope::from("some\nshit\n"));
    /// doc.insert_string("idk\n", CursorSemantics::Bar);
    /// assert_eq!("idk\nsome\nshit\n", doc.text());
    /// assert!(doc.is_modified());
    /// doc.undo(CursorSemantics::Bar);
    /// assert_eq!("some\nshit\n", doc.text());
    /// assert!(!doc.is_modified());
    /// doc.redo(CursorSemantics::Bar);
    /// assert_eq!("idk\nsome\nshit\n", doc.text());
    /// assert!(doc.is_modified());
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut doc = Document::new(CursorSemantics::Bar).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(4, 9)], 0, &text));
    /// doc.delete(CursorSemantics::Bar);
    /// assert_eq!("idk\nshit\n", doc.text());
    /// assert!(doc.is_modified());
    /// doc.undo(CursorSemantics::Bar);
    /// assert_eq!("idk\nsome\nshit\n", doc.text());
    /// assert!(!doc.is_modified());
    /// doc.redo(CursorSemantics::Bar);
    /// assert_eq!("idk\nshit\n", doc.text());
    /// assert!(doc.is_modified());
    /// 
    /// // attempting to redo with no changes
    /// let mut doc = Document::new(CursorSemantics::Bar);
    /// assert!(doc.redo(CursorSemantics::Bar).is_err());
    /// ```
    pub fn redo(&mut self, semantics: CursorSemantics) -> Result<(), ()>{
        // Check if there is something to redo
        if let Some(changes) = self.redo_stack.pop(){
            let mut new_selections = Vec::new();
            for change in changes.clone(){
                match change.operation(){
                    Operation::Insert => {
                        let (new_text, new_change) = Document::insert_string_single_selection(&change.old_selection(), &self.text, &change.text(), semantics);
                        self.text = new_text;
                        new_selections.push(new_change.new_selection());
                    }
                    Operation::Delete => {
                        let (new_text, new_change) = Document::delete_single_selection(&change.old_selection(), &self.text, semantics);
                        self.text = new_text;
                        new_selections.push(new_change.new_selection());
                    }
                }
            }
            let selections = Selections::new(new_selections, self.selections.primary_selection_index(), &self.text);
            *self.selections_mut() = selections;

            // Push changes back onto the undo stack
            self.undo_stack.push(changes);

            // update document modified status
            //self.modified = self.text != self.last_saved_text;

            Ok(())
        }else{
            Err(())
        }
    }

    /// Inserts provided string into text for a single selection.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// # use edit_core::history::{Change, Operation};
    /// 
    /// let text = Rope::from("idk\nshit\n");
    /// let selection = Selection::new(4, 4);
    /// assert_eq!(
    ///     (
    ///         Rope::from("idk\nsome\nshit\n"), 
    ///         Change::new(Operation::Insert, "some\n".to_string(), Selection::new(4, 4), Selection::with_stored_line_position(9, 9, 0))
    ///     ), 
    ///     Document::insert_string_single_selection(&selection, &text, "some\n", CursorSemantics::Bar)
    /// );
    /// ```
    pub fn insert_string_single_selection(selection: &Selection, text: &Rope, string: &str, semantics: CursorSemantics) -> (Rope, Change){
        //if string.is_empty(){
        //    return error;
        //}
        let mut new_selection = selection.clone();
        let mut new_text = text.clone();

        // Delete the current selection if extended
        if selection.is_extended(semantics){
            let change;
            (new_text, change) = Document::delete_single_selection(&selection, text, semantics);
            new_selection = change.new_selection();
        }

        // Insert the new string at the cursor position
        let cursor_pos = new_selection.cursor(semantics);
        new_text.insert(cursor_pos, string);

        // Move the selection cursor right by the length of the inserted string
        for _ in 0..string.len(){
            new_selection = new_selection.move_right(&new_text, semantics);
        }

        (new_text, Change::new(Operation::Insert, string.to_string(), selection.clone(), new_selection))
    }

    /// Removes text from document inside a single selection if extended, or the next character otherwise.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// # use edit_core::history::{Change, Operation};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let selection = Selection::new(4, 8);
    /// assert_eq!((Rope::from("idk\n\nshit\n"), Change::new(Operation::Delete, "some".to_string(), Selection::new(4, 8), Selection::with_stored_line_position(4, 4, 0))), Document::delete_single_selection(&selection, &text, CursorSemantics::Bar));
    /// ```
    pub fn delete_single_selection(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> (Rope, Change){ //can ignore Change if called from function that doesn't need to push to undo stack
        let old_selection = selection.clone();
        let mut new_selection = selection.clone();
        let mut new_text = text.clone();
        let rope = Rope::new();
        let mut change_text = rope.slice(..);

        use std::cmp::Ordering;
        match selection.cursor(semantics).cmp(&selection.anchor()){
            Ordering::Less => { //cursor < anchor
                //i<dk|\nsome\nshit\n   //i|>\nsome\nshit\n
                //i<dk|\nsome\nshit\n   //i|:\n>some\nshit\n
                change_text = text.slice(selection.head()..selection.anchor());
                new_text.remove(selection.head()..selection.anchor());
                new_selection = selection.put_cursor(selection.cursor(semantics), text, Movement::Move, semantics, true);
            }
            Ordering::Greater => {  //cursor > anchor
                match semantics{
                    CursorSemantics::Bar => {
                        //|id>k\nsome\nshit\n   //|>k\nsome\nshit\n
                        //|idk\nsome\nshit\n>   //|>
                        change_text = text.slice(selection.anchor()..selection.head());
                        new_text.remove(selection.anchor()..selection.head());
                        new_selection = selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
                    }
                    CursorSemantics::Block => {
                        //|idk\nsome\nshit\n: > //|: >
                        if selection.cursor(semantics) == text.len_chars(){
                            change_text = text.slice(selection.anchor()..selection.cursor(semantics));
                            new_text.remove(selection.anchor()..selection.cursor(semantics));
                        }
                        //|i:d>k\nsome\nshit\n  //|:k>\nsome\nshit\n
                        else{
                            change_text = text.slice(selection.anchor()..selection.head());
                            new_text.remove(selection.anchor()..selection.head());
                        }
                        new_selection = selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
                    }
                }
            }
            Ordering::Equal => {    //cursor == anchor
                //idk\nsome\nshit\n|>   //idk\nsome\nshit\n|>
                //idk\nsome\nshit\n|: > //idk\nsome\nshit\n|: >
                if selection.cursor(semantics) == text.len_chars(){}    //do nothing    //or preferrably return error
                else{
                    match semantics{
                        CursorSemantics::Bar => {
                            //|>idk\nsome\nshit\n   //|>dk\nsome\nshit\n
                            change_text = text.slice(selection.head()..selection.head().saturating_add(1));
                            new_text.remove(selection.head()..selection.head().saturating_add(1));
                        }
                        CursorSemantics::Block => {
                            //|:i>dk\nsome\nshit\n  //|:d>k\nsome\nshit\n
                            change_text = text.slice(selection.anchor()..selection.head());
                            new_text.remove(selection.anchor()..selection.head());
                        }
                    }
                    new_selection = selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
                }
            }
        }

        let change = Change::new(Operation::Delete, change_text.to_string(), old_selection, new_selection);
        (new_text, change)
    }

    /// Inserts provided string into text at each selection.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selections, Selection, CursorSemantics};
    /// # use edit_core::history::{ChangeSet, Change, Operation};
    /// 
    /// fn test(expected: Rope, selections: Vec<Selection>, expected_changes: Vec<ChangeSet>, string: &str, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nshit\n");
    ///     let mut document = Document::new(semantics).with_text(Rope::from(text.clone())).with_selections(Selections::new(selections, 0, &text));
    ///     document.insert_string(string, semantics);
    ///     println!("expected: {:#?}\ngot: {:#?}\nexpected_changes: {:#?}\ngot: {:#?}", expected, document.text().clone(), expected_changes, document.undo_stack());
    ///     document.text().clone() == expected &&
    ///     document.undo_stack() == expected_changes
    /// }
    /// assert!(
    ///     test(
    ///         Rope::from("idk\nsome\nshit\nsome\n"), 
    ///         vec![
    ///             Selection::new(4, 4), 
    ///             Selection::new(9, 9)
    ///         ], 
    ///         vec![ChangeSet::new(vec![
    ///             Change::new(Operation::Insert, "some\n".to_string(), Selection::new(9, 9), Selection::with_stored_line_position(14, 14, 0)), 
    ///             Change::new(Operation::Insert, "some\n".to_string(), Selection::new(4, 4), Selection::with_stored_line_position(9, 9, 0))
    ///         ])], 
    ///         "some\n", 
    ///         CursorSemantics::Bar
    ///     )
    /// );
    /// ```
    pub fn insert_string(&mut self, string: &str, semantics: CursorSemantics){
        let mut changes = Vec::new();

        //if string == "\n"{}   // handle behavior specific to pressing "enter". auto-indent, etc...
        /*else */if string == "\t"{ // handle behavior specific to pressing "tab".
            for selection in self.selections.iter_mut().rev(){
                let tab_distance = text_util::distance_to_next_multiple_of_tab_width(selection.clone(), &self.text, semantics);
                let modified_tab_width = if tab_distance > 0 && tab_distance < TAB_WIDTH{
                    tab_distance
                }else{
                    TAB_WIDTH
                };
                let mut soft_tab = String::new();
                //for _ in 0..modified_tab_width{   //deprecating: this produces a change for each space character inserted, instead of a change inserting them all at once
                //    let (new_text, change) = Document::insert_string_single_selection(selection, &self.text, &' '.to_string(), semantics);
                //    self.text = new_text;
                //    *selection = change.new_selection();
                //    changes.push(change);
                //}

                for _ in 0..modified_tab_width{
                    soft_tab.push(' ');
                }
                let (new_text, change) = Document::insert_string_single_selection(selection, &self.text, &soft_tab, semantics);
                self.text = new_text;
                *selection = change.new_selection();
                changes.push(change);
            }
        }
        else{
            // insert string at each selection
            for selection in self.selections.iter_mut().rev(){
                let (new_text, change) = Document::insert_string_single_selection(selection, &self.text, string, semantics);
                self.text = new_text;
                *selection = change.new_selection();
                changes.push(change);
            }
        }

        // push change set to undo stack
        self.undo_stack.push(ChangeSet::new(changes));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        // update document modified status
        //self.modified = self.text != self.last_saved_text;
    }

    // TODO: test multiple selections. only testing one right now...
    /// Deletes text inside each [`Selection`] in [`Selections`], or if [`Selection`] not extended, the next character, and pushes changes to undo stack.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// # use edit_core::history::{Change, ChangeSet, Operation};
    /// 
    /// fn test(name: &str, selection: Selection, expected_selection: Selection, expected_text: Rope, expected_undo_stack: Vec<ChangeSet>, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    ///     let changes = doc.delete(semantics);
    ///     println!("{:#?}\n{:#?}\nexpected_text {:#?}\ngot: {:#?}\nexpected_selection: {:#?}\ngot: {:#?}\nexpected_undo_stack: {:#?}\ngot: {:#?}\n", name, semantics, expected_text, doc.text().clone(), expected_selection, doc.selections().primary().clone(), expected_undo_stack, doc.undo_stack());
    ///     doc.text().clone() == expected_text &&
    ///     doc.selections().primary().clone() == expected_selection &&
    ///     doc.undo_stack() == expected_undo_stack
    /// }
    /// 
    /// // will not delete past end of doc
    /// assert!(test("test1", Selection::new(14, 14), Selection::new(14, 14), Rope::from("idk\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "".to_string(), Selection::new(14, 14), Selection::new(14, 14))])], CursorSemantics::Bar));
    /// assert!(test("test1", Selection::new(14, 15), Selection::new(14, 15), Rope::from("idk\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "".to_string(), Selection::new(14, 15), Selection::new(14, 15))])], CursorSemantics::Block)); //idk\nsome\nshit\n|: >
    /// 
    /// // no selection
    /// assert!(test("test2", Selection::new(0, 0), Selection::with_stored_line_position(0, 0, 0), Rope::from("dk\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "i".to_string(), Selection::new(0, 0), Selection::with_stored_line_position(0, 0, 0))])], CursorSemantics::Bar));
    /// assert!(test("test2", Selection::new(0, 1), Selection::with_stored_line_position(0, 1, 0), Rope::from("dk\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "i".to_string(), Selection::new(0, 1), Selection::with_stored_line_position(0, 1, 0))])], CursorSemantics::Block));    //|:i>dk\nsome\nshit\n
    /// 
    /// // with selection head > anchor
    /// assert!(test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 0, 0), Rope::from("k\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "id".to_string(), Selection::new(0, 2), Selection::with_stored_line_position(0, 0, 0))])], CursorSemantics::Bar));
    /// assert!(test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 1, 0), Rope::from("k\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "id".to_string(), Selection::new(0, 2), Selection::with_stored_line_position(0, 1, 0))])], CursorSemantics::Block)); //|i:d>k\nsome\nshit\n
    /// 
    /// // with selection head < anchor
    /// assert!(test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 1, 1), Rope::from("i\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "dk".to_string(), Selection::new(3, 1), Selection::with_stored_line_position(1, 1, 1))])], CursorSemantics::Bar));
    /// assert!(test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 2, 1), Rope::from("i\nsome\nshit\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "dk".to_string(), Selection::new(3, 1), Selection::with_stored_line_position(1, 2, 1))])], CursorSemantics::Block));    //i|d:k>\nsome\nshit\n
    /// 
    /// // with whole text selected
    /// assert!(test("test5", Selection::new(0, 13), Selection::with_stored_line_position(0, 0, 0), Rope::from("\n"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "idk\nsome\nshit".to_string(), Selection::new(0, 13), Selection::with_stored_line_position(0, 0, 0))])], CursorSemantics::Bar));  //just verifying...
    /// assert!(test("test5", Selection::new(0, 14), Selection::with_stored_line_position(0, 0, 0), Rope::from(""), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "idk\nsome\nshit\n".to_string(), Selection::new(0, 14), Selection::with_stored_line_position(0, 0, 0))])], CursorSemantics::Bar));
    /// assert!(test("test5", Selection::new(0, 15), Selection::with_stored_line_position(0, 1, 0), Rope::from(""), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "idk\nsome\nshit\n".to_string(), Selection::new(0, 15), Selection::with_stored_line_position(0, 1, 0))])], CursorSemantics::Block));  //|idk\nsome\nshit\n: >
    /// 
    /// // at 1 less doc end
    /// assert!(test("test6", Selection::new(13, 13), Selection::with_stored_line_position(13, 13, 4), Rope::from("idk\nsome\nshit"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "\n".to_string(), Selection::new(13, 13), Selection::with_stored_line_position(13, 13, 4))])], CursorSemantics::Bar));
    /// assert!(test("test6", Selection::new(13, 14), Selection::with_stored_line_position(13, 14, 4), Rope::from("idk\nsome\nshit"), vec![ChangeSet::new(vec![Change::new(Operation::Delete, "\n".to_string(), Selection::new(13, 14), Selection::with_stored_line_position(13, 14, 4))])], CursorSemantics::Block));  //idk\nsome\nshit|:\n> //idk\nsome\nshit|: >
    /// ```
    pub fn delete(&mut self, semantics: CursorSemantics){
        let mut changes = Vec::new();

        for selection in self.selections.iter_mut().rev(){
            let (new_text, change) = Document::delete_single_selection(&selection, &self.text, semantics);
            self.text = new_text;
            *selection = change.new_selection();
            changes.push(change);
        }

        // push change set to undo stack
        self.undo_stack.push(ChangeSet::new(changes));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        // update document modified status
        //self.modified = self.text != self.last_saved_text;
    }

    /// Deletes the previous character, or deletes selection if extended.
    /// #### Invariants:
    /// - will not delete past start of doc
    /// - at start of line, appends current line to end of previous line
    /// - removes previous soft tab, if TAB_WIDTH spaces are before cursor
    /// - deletes selection if selection extended
    /// # Example
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::{Document, TAB_WIDTH};
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// 
    /// fn test(name: &str, selection: Selection, expected: Rope, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    ///     doc.backspace(semantics);
    ///     println!("{:#?}\n{:#?}\nexpected: {:#?}\ngot: {:#?}\n", name, semantics, expected, doc.text().clone());
    ///     doc.text().clone() == expected
    /// }
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // does nothing at doc start
    /// assert!(test("test0", Selection::new(0, 0), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test0", Selection::new(0, 1), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block));
    /// 
    /// // without selection deletes previous char
    /// assert!(test("test1", Selection::new(1, 1), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test1", Selection::new(1, 2), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));   //i|:d>k\nsome\nshit\n
    /// 
    /// // backspace at start of line appends current line to end of previous line
    /// assert!(test("test2", Selection::new(4, 4), Rope::from("idksome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test2", Selection::new(4, 5), Rope::from("idksome\nshit\n"), CursorSemantics::Block));
    /// 
    /// // with selection and head > anchor
    /// assert!(test("test3", Selection::new(0, 2), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test3", Selection::new(0, 2), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block));
    /// 
    /// // with selection and head < anchor
    /// assert!(test("test4", Selection::new(2, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test4", Selection::new(2, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block));
    /// 
    /// // at text end
    /// assert!(test("test5", Selection::new(14, 14), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
    /// assert!(test("test5", Selection::new(14, 15), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit\n|: > //idk\nsome\nshit|: >
    /// 
    /// // backspace removes previous tab
    /// let mut spaces = String::new();
    /// for x in 0..TAB_WIDTH{
    ///     spaces.push(' ');
    /// }
    /// let text = Rope::from(format!("{}idk\nsome\nshit\n", spaces));
    /// let semantics = CursorSemantics::Block; //test Bar too
    /// let selection = Selection::new(TAB_WIDTH, match semantics{CursorSemantics::Bar => TAB_WIDTH, CursorSemantics::Block => TAB_WIDTH.saturating_add(1)});
    /// let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    /// doc.backspace(semantics);
    /// assert!(doc.text().clone() == Rope::from("idk\nsome\nshit\n"));
    /// assert!(doc.selections().primary().clone() == Selection::with_stored_line_position(0, match semantics{CursorSemantics::Bar => 0, CursorSemantics::Block => 1}, 0));
    /// ```
    #[allow(clippy::collapsible_else_if)]
    pub fn backspace(&mut self, semantics: CursorSemantics){
        let mut changes = Vec::new();

        for selection in self.selections.iter_mut().rev(){
            let cursor_line_position = selection.cursor(semantics).saturating_sub(self.text.line_to_char(self.text.char_to_line(selection.cursor(semantics))));

            let is_deletable_soft_tab = cursor_line_position >= TAB_WIDTH
            // handles case where user adds a space after a tab, and wants to delete only the space
            && cursor_line_position % TAB_WIDTH == 0
            // if previous 4 chars are spaces, delete 4. otherwise, use default behavior
            && text_util::slice_is_all_spaces(
                self.text.line(
                    self.text.char_to_line(selection.cursor(semantics))
                ).as_str().unwrap(),
                cursor_line_position - TAB_WIDTH,
                cursor_line_position
            );
            
            if selection.is_extended(semantics){
                let (new_text, change) = Document::delete_single_selection(&selection, &self.text, semantics);
                self.text = new_text;
                *selection = change.new_selection();
                changes.push(change);
            }else{
                if is_deletable_soft_tab{
                    //for _ in 0..TAB_WIDTH{    //deprecating: this produces a change for each space character deleted, instead of a change deleting them all at once
                    //    *selection = selection.move_left(&self.text, semantics);
                    //    let (new_text, change) = Document::delete_single_selection(&selection, &self.text, semantics);
                    //    self.text = new_text;
                    //    *selection = change.new_selection();
                    //    changes.push(change);
                    //}

                    // move cursor to start of soft_tab
                    for _ in 0..TAB_WIDTH{
                        *selection = selection.move_left(&self.text, semantics);
                    }
                    // extend selection to encompass soft_tab
                    match semantics{
                        CursorSemantics::Bar => {
                            for _ in 0..TAB_WIDTH{
                                *selection = selection.extend_right(&self.text, semantics);
                            }
                        }
                        CursorSemantics::Block => {
                            for _ in 0..TAB_WIDTH.saturating_sub(1){
                                *selection = selection.extend_right(&self.text, semantics);
                            }
                        }
                    }
                    // delete soft tab
                    let (new_text, change) = Document::delete_single_selection(&selection, &self.text, semantics);
                    self.text = new_text;
                    *selection = change.new_selection();
                    changes.push(change);
                }
                else if selection.cursor(semantics) > 0{
                    *selection = selection.move_left(&self.text, semantics);
                    let (new_text, change) = Document::delete_single_selection(&selection, &self.text, semantics);
                    self.text = new_text;
                    *selection = change.new_selection();
                    changes.push(change);
                }
            }
        }

        // push changes to undo stack
        self.undo_stack.push(ChangeSet::new(changes));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        // update document modified status
        //self.modified = self.text != self.last_saved_text;
    }

    /// Cut single selection.
    /// Copies text to clipboard and removes selected text from document.
    /// Ensure single selection when calling this function.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// 
    /// fn test(selection: Selection, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    ///     doc.cut(semantics);
    ///     println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
    ///     doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection
    /// 
    ///     //TODO: ensure clipboard text is correct as well
    /// }
    /// 
    /// assert!(test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
    /// assert!(test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
    /// assert!(test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    /// assert!(test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    /// ```
    pub fn cut(&mut self, semantics: CursorSemantics){  //-> Result<(), Error>  if multiple selections
        //assert!(self.selections.count() == 1);    // if multiple selections, trigger warning  //prob to be done in client code
        
        let selection = self.selections.primary_mut();

        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();

        // Remove the selected text from the document
        let (new_text, change) = Document::delete_single_selection(&selection, &self.text, semantics);
        self.text = new_text;
        *selection = change.new_selection();

        // push changes to undo stack
        self.undo_stack.push(ChangeSet::new(vec![change]));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        // update document modified status
        //self.modified = self.text != self.last_saved_text;
    }

    /// Copy single selection to clipboard.
    /// Ensure single selection when calling this function.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// 
    /// fn test(selection: Selection, expected: &str, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    ///     doc.copy();
    ///     println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.clipboard());
    ///     doc.clipboard() == expected
    /// }
    /// 
    /// assert!(test(Selection::new(4, 9), "some\n", CursorSemantics::Bar));
    /// assert!(test(Selection::new(4, 9), "some\n", CursorSemantics::Block));    //idk\n|some:\n>shit\n
    /// ```
    pub fn copy(&mut self){ //-> Result<(), Error>  if multiple selections
        //assert!(self.selections.count() == 1);    // if multiple selections, trigger warning  //prob to be done in client code
        
        let selection = self.selections.primary().clone();

        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();
    }

    /// Insert clipboard contents at cursor position(s).
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// 
    /// fn test(selection: Selection, string: &str, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text)).with_clipboard(string.to_string());
    ///     doc.paste(semantics);
    ///     println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
    ///     doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection
    /// }
    /// 
    /// assert!(test(Selection::new(9, 9), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 15, 0), CursorSemantics::Bar));
    /// assert!(test(Selection::new(9, 10), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 16, 0), CursorSemantics::Block));
    /// ```
    pub fn paste(&mut self, semantics: CursorSemantics){
        let mut changes = Vec::new();

        // insert clipboard text into the document
        for selection in self.selections.iter_mut().rev(){
            let (new_text, change) = Document::insert_string_single_selection(selection, &self.text, &self.clipboard, semantics);
            self.text = new_text;
            *selection = change.new_selection();
            changes.push(change);
        }

        // push changes to undo stack
        self.undo_stack.push(ChangeSet::new(changes));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();

        // update document modified status
        //self.modified = self.text != self.last_saved_text;
    }
}
