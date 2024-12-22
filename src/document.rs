use crate::view::View;
use crate::selection::{CursorSemantics, Movement, Selection, Selections, SelectionsError};
use crate::history::{Operation, Change, ChangeSet};
use std::fs::{self, File};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use ropey::Rope;
use crate::text_util;
use std::cmp::Ordering;

// tab keypress inserts the number of spaces specified in TAB_WIDTH into the focused document
pub const TAB_WIDTH: usize = 4; //should this be language dependant? on-the-fly configurable?
// whether to use hard or soft tabs
pub const USE_HARD_TAB: bool = false;
// whether to use full file path or just file name
pub const USE_FULL_FILE_PATH: bool = false;



#[derive(Debug)]
pub enum DocumentError{
    NoChangesToUndo,
    NoChangesToRedo,
}
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
    // if possible, i would like to implement these elsewhere, and have them still be usable in other test locations
    ////////////////////////////////////////////////////////////////////// Testing Only ///////////////////////////////////////////////////////////////////////////
    /// Instantiate a new [`Document`]. Only for testing.                                                                                                        //
    /**/pub fn new(cursor_semantics: CursorSemantics) -> Self{                                                                                                   //
    /**/    Self::initialize_fields(None, Rope::new(), cursor_semantics)                                                                         //
    /**/}                                                                                                                                                        //
    /// Add [Rope]-based text to an existing instance of [Document]. Only for testing.                                                                           //
    /**/pub fn with_text(mut self, text: Rope) -> Self{                                                                                                          //
    /**/    self.text = text.clone();                                                                                                                            //
    /**/    self.last_saved_text = text;                                                                                                                         //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    /// Add [Selections] to an existing instance of [Document]. Only for testing.                                                                                //
    /**/pub fn with_selections(mut self, selections: Selections) -> Self{                                                                                        //
    /**/    self.selections = selections;                                                                                                                        //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    /// Add a [View] to an existing instance of [Document]. Only for testing.                                                                                    //
    /**/pub fn with_view(mut self, view: View) -> Self{                                                                                                          //
    /**/    self.client_view = view;                                                                                                                             //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    /// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.       //
    /**/pub fn with_clipboard(mut self, clipboard: String) -> Self{                                                                                              //
    /**/    self.clipboard = clipboard;                                                                                                                          //
    /**/    self                                                                                                                                                 //
    /**/}                                                                                                                                                        //
    ////////////////////////////////////////////////////////////////////// Testing Only ///////////////////////////////////////////////////////////////////////////
    
    pub fn open(path: &PathBuf, cursor_semantics: CursorSemantics) -> Result<Self, Box<dyn Error>>{
        let text = Rope::from_reader(BufReader::new(File::open(path)?))?;   // pass errors up

        // TODO: make text tab use match settings for USE_HARD_TAB and TAB_WIDTH

        Ok(Self::initialize_fields(Some(path.clone()), text, cursor_semantics))
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
    pub fn undo_stack(&self) -> Vec<ChangeSet>{ //should this be &Vec?
        self.undo_stack.clone()
    }
    pub fn redo_stack(&self) -> Vec<ChangeSet>{ //should this be &Vec?
        self.redo_stack.clone()
    }
    pub fn save(&mut self) -> Result<(), Box<dyn Error>>{
        if let Some(path) = &self.file_path{ // does nothing if path is None    //maybe return Err(()) instead?
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
        doc_text.insert(selection.cursor(semantics), string);
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
        let old_selection = selection.clone();
        let original_text = doc_text.clone();

        use std::cmp::Ordering;
        let (start, end, new_cursor) = match selection.cursor(semantics).cmp(&selection.anchor()){
            Ordering::Less => {(selection.head(), selection.anchor(), selection.cursor(semantics))}
            Ordering::Greater => {
                match semantics{
                    CursorSemantics::Bar => {(selection.anchor(), selection.head(), selection.anchor())}
                    CursorSemantics::Block => {
                        if selection.cursor(semantics) == doc_text.len_chars(){(selection.anchor(), selection.cursor(semantics), selection.anchor())}
                        else{(selection.anchor(), selection.head(), selection.anchor())}
                    }
                }
            }
            Ordering::Equal => {
                if selection.cursor(semantics) == doc_text.len_chars(){ //do nothing    //or preferrably return error   //could have condition check in calling fn
                    return Change::new(Operation::Delete, old_selection, selection.clone(), Operation::Insert{inserted_text: "".to_string()});
                }else{
                    match semantics{
                        CursorSemantics::Bar => {(selection.head(), selection.head().saturating_add(1), selection.anchor())}
                        CursorSemantics::Block => {(selection.anchor(), selection.head(), selection.anchor())}
                    }
                }
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

    /// Undoes the most recent change made to the document, restoring the previous state.
    #[allow(clippy::result_unit_err)]
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

    /// Redoes the most recent Undo made to the document, restoring the previous state.
    /// Make sure to clear the redo stack in every edit fn. new actions invalidate the redo history
    #[allow(clippy::result_unit_err)]
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
    pub fn insert_string(&mut self, string: &str, semantics: CursorSemantics){  //-> Result<(), ()>{    //error if empty string
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

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
                        let tab_distance = text_util::distance_to_next_multiple_of_tab_width(selection.clone(), &self.text, semantics);
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
    pub fn delete(&mut self, semantics: CursorSemantics){   //-> Result<(), ()>{    //error if delete would result in same state(like at end of doc)
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

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
    }

    /// Deletes the previous character, or deletes selection if extended.
    /// #### Invariants:
    /// - will not delete past start of doc
    /// - at start of line, appends current line to end of previous line
    /// - removes previous soft tab, if TAB_WIDTH spaces are before cursor
    /// - deletes selection if selection extended
    #[allow(clippy::collapsible_else_if)]
    pub fn backspace(&mut self, semantics: CursorSemantics){    //-> Result<(), ()>{    //if backspace will result in same state, return err(like at start of doc)
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            if selection.is_extended(semantics){
                let change = Document::apply_delete(&mut self.text, selection, semantics);
                if let Operation::Insert{inserted_text} = change.inverse(){
                    self.selections.shift_subsequent_selections_backward(i, inserted_text.len());
                }
                changes.push(change);
            }else{
                let offset_from_line_start = text_util::offset_from_line_start(selection.cursor(semantics), &self.text);
                let line = self.text.line(self.text.char_to_line(selection.cursor(semantics)));
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
                else if selection.cursor(semantics) > 0{
                    //*selection = selection.move_left(&self.text, semantics);
                    if let Ok(new_selection) = selection.move_left(&self.text, semantics){
                        *selection = new_selection;
                    }
                    changes.push(Document::apply_delete(&mut self.text, selection, semantics));
                    self.selections.shift_subsequent_selections_backward(i, 1);
                }//else{return Err(());}    //cannot backspace at beginning of text
            }
        }

        // push changes to undo stack
        self.undo_stack.push(ChangeSet::new(changes, selections_before_changes, self.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();
    }

    /// Cut single selection.
    /// Copies text to clipboard and removes selected text from document.
    /// Ensure single selection when calling this function.
    #[allow(clippy::result_unit_err)]
    pub fn cut(&mut self, semantics: CursorSemantics) -> Result<(), SelectionsError>{
        if self.selections.count() > 1{return Err(SelectionsError::MultipleSelections)}

        let selection = self.selections.primary_mut();
        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();
        self.delete(semantics);

        Ok(())
    }

    /// Copy single selection to clipboard.
    /// Ensure single selection when calling this function.
    #[allow(clippy::result_unit_err)]
    pub fn copy(&mut self) -> Result<(), SelectionsError>{
        if self.selections.count() > 1{return Err(SelectionsError::MultipleSelections)}
        
        let selection = self.selections.primary().clone();
        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();

        Ok(())
    }

    /// Insert clipboard contents at cursor position(s).
    pub fn paste(&mut self, semantics: CursorSemantics){    //-> Result<(), ()>{    //error if clipboard string is empty string
        self.insert_string(&self.clipboard.clone(), semantics);
    }

    // frontend application should save previous selections for case where user exits search mode
    pub fn search(&mut self, input: &str, semantics: CursorSemantics){
        let mut selections = Vec::new();
        let mut start = 0;
        
        while let Some(index_of_match) = text_util::kmp_search(&self.text.to_string()[start..], input){
        //while let Some(index_of_match) = text_util::naive_search(&self.text.to_string()[start..], input){
            let index_of_match_end = index_of_match + input.len();
            selections.push(Selection::new(index_of_match + start, index_of_match_end + start));
            start = index_of_match_end + start;
            if start >= self.text.len_chars(){
                break;
            }
        }

        if !selections.is_empty(){
            self.selections = Selections::new(selections, 0, &self.text);
        }
    }
}
