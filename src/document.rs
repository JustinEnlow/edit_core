use crate::view::View;
use crate::selection::{CursorSemantics, Movement, Selection, Selections};
use crate::history::{Operation, Change, ChangeSet};
use std::fs::{self, File};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use ropey::Rope;
use crate::text_util;

// tab keypress inserts the number of spaces specified in TAB_WIDTH into the focused document
pub const TAB_WIDTH: usize = 4; //should this be language dependant? on-the-fly configurable?
// whether to use hard or soft tabs
pub const USE_HARD_TAB: bool = false;
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
    ///// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.
    //pub fn with_clipboard(mut self, clipboard: String) -> Self{
    //    self.clipboard = clipboard;
    //    self
    //}
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
    
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    fn apply_replace(doc_text: &mut Rope, replacement_text: &str, selection: &mut Selection, semantics: CursorSemantics) -> Change{
        let old_selection = selection.clone();
        let mut replaced_text = String::new();
        let delete_change = Document::apply_delete(doc_text, selection, semantics);
        if let Operation::Insert{inserted_text} = delete_change.inverse(){
            replaced_text = inserted_text;
        }
        let _ = Document::apply_insert(doc_text, replacement_text, selection, semantics);   //intentionally discard returned Change

        Change::new(Operation::Replace{replacement_text: replacement_text.to_string()}, old_selection, selection.clone(), Operation::Replace{replacement_text: replaced_text})
    }
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    fn apply_insert(doc_text: &mut Rope, string: &str, selection: &mut Selection, semantics: CursorSemantics) -> Change{
        let old_selection = selection.clone();
        doc_text.insert(selection.cursor(semantics), string);
        for _ in 0..string.len(){
            *selection = selection.move_right(doc_text, semantics);
        }

        Change::new(Operation::Insert{inserted_text: string.to_string()}, old_selection, selection.clone(), Operation::Delete)
    }
    // TODO: test. should test rope is edited correctly and selection is moved correctly, not necessarily the returned change. behavior, not impl
    fn apply_delete(doc_text: &mut Rope, selection: &mut Selection, semantics: CursorSemantics) -> Change{
        let old_selection = selection.clone();
        let original_text = doc_text.clone();
        let rope = Rope::new();
        let mut change_text = rope.slice(..);

        use std::cmp::Ordering;
        match selection.cursor(semantics).cmp(&selection.anchor()){
            Ordering::Less => { //cursor < anchor
                change_text = original_text.slice(selection.head()..selection.anchor());
                doc_text.remove(selection.head()..selection.anchor());
                *selection = selection.put_cursor(selection.cursor(semantics), &original_text, Movement::Move, semantics, true);
            }
            Ordering::Greater => {  //cursor > anchor
                match semantics{
                    CursorSemantics::Bar => {
                        change_text = original_text.slice(selection.anchor()..selection.head());
                        doc_text.remove(selection.anchor()..selection.head());
                        *selection = selection.put_cursor(selection.anchor(), &original_text, Movement::Move, semantics, true);
                    }
                    CursorSemantics::Block => {
                        if selection.cursor(semantics) == doc_text.len_chars(){
                            change_text = original_text.slice(selection.anchor()..selection.cursor(semantics));
                            doc_text.remove(selection.anchor()..selection.cursor(semantics));
                        }
                        else{
                            change_text = original_text.slice(selection.anchor()..selection.head());
                            doc_text.remove(selection.anchor()..selection.head());
                        }
                        *selection = selection.put_cursor(selection.anchor(), &original_text, Movement::Move, semantics, true);
                    }
                }
            }
            Ordering::Equal => {    //cursor == anchor
                if selection.cursor(semantics) == doc_text.len_chars(){}    //do nothing    //or preferrably return error
                else{
                    match semantics{
                        CursorSemantics::Bar => {
                            change_text = original_text.slice(selection.head()..selection.head().saturating_add(1));
                            doc_text.remove(selection.head()..selection.head().saturating_add(1));
                        }
                        CursorSemantics::Block => {
                            change_text = original_text.slice(selection.anchor()..selection.head());
                            doc_text.remove(selection.anchor()..selection.head());
                        }
                    }
                    *selection = selection.put_cursor(selection.anchor(), &original_text, Movement::Move, semantics, true);
                }
            }
        }

        Change::new(Operation::Delete, old_selection, selection.clone(), Operation::Insert{inserted_text: change_text.to_string()})
    }

    fn adjust_subsequent_selections_after_insert(current_selection_index: usize, selections: &mut Selections, inserted_text_len: usize){
        for subsequent_selection_index in current_selection_index.saturating_add(1)..selections.count(){
            let subsequent_selection = selections.nth_mut(subsequent_selection_index);
            *subsequent_selection = Selection::new(subsequent_selection.anchor().saturating_add(inserted_text_len), subsequent_selection.head().saturating_add(inserted_text_len));
        }
    }
    fn adjust_subsequent_selections_after_delete(current_selection_index: usize, selections: &mut Selections, deleted_text_len: usize){
        for subsequent_selection_index in current_selection_index.saturating_add(1)..selections.count(){
            let subsequent_selection = selections.nth_mut(subsequent_selection_index);
            *subsequent_selection = Selection::new(subsequent_selection.anchor().saturating_sub(deleted_text_len), subsequent_selection.head().saturating_sub(deleted_text_len));
        }
    }
    fn adjust_subsequent_selections_after_replace(current_selection_index: usize, selections: &mut Selections, replacement_text_len: usize, original_text_len: usize){
        use std::cmp::Ordering;
        match original_text_len.cmp(&replacement_text_len){    //old selected text vs new text
            Ordering::Greater => {
                let difference = original_text_len.saturating_sub(replacement_text_len);
                for subsequent_selection_index in current_selection_index.saturating_add(1)..selections.count(){
                    let selection = selections.nth_mut(subsequent_selection_index);
                    *selection = Selection::new(selection.anchor().saturating_sub(difference), selection.head().saturating_sub(difference));
                }
            }
            Ordering::Less => {
                let difference = replacement_text_len.saturating_sub(original_text_len);
                for subsequent_selection_index in current_selection_index.saturating_add(1)..selections.count(){
                    let selection = selections.nth_mut(subsequent_selection_index);
                    *selection = Selection::new(selection.anchor().saturating_add(difference), selection.head().saturating_add(difference));
                }
            }
            Ordering::Equal => {}   // no change to subsequent selections
        }
    }

    fn shift_and_extend_selection(doc_text: &Rope, text_len: usize, selection: &mut Selection, semantics: CursorSemantics){
        for _ in 0..text_len{
            *selection = selection.move_left(doc_text, semantics);
        }
        if text_len > 1{
            match semantics{
                CursorSemantics::Bar => {
                    for _ in 0..text_len{
                        *selection = selection.extend_right(doc_text, semantics);
                    }
                }
                CursorSemantics::Block => {
                    for _ in 0..text_len.saturating_sub(1){
                        *selection = selection.extend_right(doc_text, semantics);
                    }
                }
            }
        }
    }

    /// Undoes the most recent change made to the document, restoring the previous state.
    pub fn undo(&mut self, semantics: CursorSemantics) -> Result<(), ()>{
        // Check if there is something to undo
        if let Some(changes) = self.undo_stack.pop(){
            let changes_as_vec = changes.changes();
            
            *self.selections_mut() = changes.clone().selections_after_changes();    //set selections to selections_after_changes to account for any selection movements that may have occurred since edit
            assert!(self.selections.count() == changes_as_vec.len());

            for i in 0..self.selections.count(){
                let change = changes_as_vec[i].clone();
                let selection = self.selections.nth_mut(i);
                let inverse_operation = change.inverse();
                match inverse_operation{
                    Operation::Insert{inserted_text} => {
                        let _ = Document::apply_insert(&mut self.text, &inserted_text, selection, semantics);   //apply inverse operation
                        Document::adjust_subsequent_selections_after_insert(i, &mut self.selections, inserted_text.len());
                    }
                    Operation::Delete => {
                        if let Operation::Insert{inserted_text} = change.operation(){   //need destructuring to get inserted_text from change
                            Document::shift_and_extend_selection(&self.text, inserted_text.len(), selection, semantics);
                            let _ = Document::apply_delete(&mut self.text, selection, semantics);
                            Document::adjust_subsequent_selections_after_delete(i, &mut self.selections, inserted_text.len());
                        }
                    }
                    Operation::Replace{replacement_text} => {
                        let undo_text = replacement_text;    //this is the text we want to go back to
                        if let Operation::Replace{replacement_text} = change.operation(){   //need destructuring to get replacement_text from change
                            Document::shift_and_extend_selection(&self.text, replacement_text.len(), selection, semantics);
                            let _ = Document::apply_replace(&mut self.text, &undo_text, selection, semantics);
                            Document::adjust_subsequent_selections_after_replace(i, &mut self.selections, undo_text.len(), replacement_text.len());
                        }
                    }
                }
            }

            // selections should be the same as they were before changes were made, because we are restoring that previous state
            *self.selections_mut() = changes.selections_before_changes();

            // Push inverted changes onto redo stack
            self.redo_stack.push(changes);

            Ok(())
        }else{
            Err(())
        }
    }

    /// Redoes the most recent Undo made to the document, restoring the previous state.
    /// Make sure to clear the redo stack in every edit fn. new actions invalidate the redo history
    pub fn redo(&mut self, semantics: CursorSemantics) -> Result<(), ()>{
        // Check if there is something to redo
        if let Some(changes) = self.redo_stack.pop(){
            let changes_as_vec = changes.changes();

            *self.selections_mut() = changes.clone().selections_before_changes();    //set selections to selections_before_changes to account for any selection movements that may have occurred since undo
            assert!(self.selections.count() == changes_as_vec.len());   //num selections should match num changes
            
            for i in 0..self.selections.count(){
                let change = changes_as_vec[i].clone();
                let selection = self.selections.nth_mut(i);

                // reapply change
                match change.operation(){
                    Operation::Insert{inserted_text} => {
                        let _ = Document::apply_insert(&mut self.text, &inserted_text, selection, semantics);
                        Document::adjust_subsequent_selections_after_insert(i, &mut self.selections, inserted_text.len());
                    }
                    Operation::Delete => {  //not working for backspace...
                        *selection = change.selection_before_change();  //this somehow makes backspace work...
                        let change = Document::apply_delete(&mut self.text, selection, semantics);
                        if let Operation::Insert{inserted_text} = change.inverse(){
                            Document::adjust_subsequent_selections_after_delete(i, &mut self.selections, inserted_text.len());
                        }
                    }
                    Operation::Replace{replacement_text} => {
                        let change = Document::apply_replace(&mut self.text, &replacement_text, selection, semantics);
                        let redo_text = replacement_text;
                        if let Operation::Replace{replacement_text} = change.inverse(){   //destructure to get currently selected text
                            let current_text = replacement_text;
                            Document::adjust_subsequent_selections_after_replace(i, &mut self.selections, redo_text.len(), current_text.len());
                        }
                    }
                }
            }

            //assert!(self.selections == changes.clone().selections_after_changes());

            // Push changes back onto the undo stack
            self.undo_stack.push(changes);

            Ok(())
        }else{
            Err(())
        }
    }

    /// Inserts provided string into text at each selection.
    pub fn insert_string(&mut self, string: &str, semantics: CursorSemantics){
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

        // handle behavior specific to pressing "enter". auto-indent, etc...
        //if string == "\n"{}
        // handle behavior specific to pressing "tab".
        /*else */if string == "\t"{
            for i in 0..self.selections.count(){
                let selection = self.selections.nth_mut(i);
                if selection.is_extended(semantics){
                    // handle tab insert with selection extended
                }
                else{
                    if USE_HARD_TAB{
                        let change = Document::apply_insert(&mut self.text, string, selection, semantics);
                        Document::adjust_subsequent_selections_after_insert(i, &mut self.selections, string.len());
                        changes.push(change);
                    }else{
                        let tab_distance = text_util::distance_to_next_multiple_of_tab_width(selection.clone(), &self.text, semantics);
                        let modified_tab_width = if tab_distance > 0 && tab_distance < TAB_WIDTH{tab_distance}else{TAB_WIDTH};

                        let soft_tab = " ".repeat(modified_tab_width);
                        
                        let change = Document::apply_insert(&mut self.text, &soft_tab, selection, semantics);
                        Document::adjust_subsequent_selections_after_insert(i, &mut self.selections, soft_tab.len());
                        changes.push(change);
                    }
                }
            }
        }
        // handle any other inserted string
        else{
            for i in 0..self.selections.count(){
                let selection = self.selections.nth_mut(i);
                if selection.is_extended(semantics){
                    let change = Document::apply_replace(&mut self.text, string, selection, semantics);
                    if let Operation::Replace{replacement_text} = change.inverse(){
                        Document::adjust_subsequent_selections_after_replace(i, &mut self.selections, string.len(), replacement_text.len());
                    }
                    changes.push(change);
                }
                else{
                    let change = Document::apply_insert(&mut self.text, string, selection, semantics);
                    Document::adjust_subsequent_selections_after_insert(i, &mut self.selections, string.len());
                    changes.push(change);
                }
                
            }
        }

        // push change set to undo stack
        self.undo_stack.push(ChangeSet::new(changes, selections_before_changes, self.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();
    }

    /// Deletes text inside each [`Selection`] in [`Selections`], or if [`Selection`] not extended, the next character, and pushes changes to undo stack.
    pub fn delete(&mut self, semantics: CursorSemantics){
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();

        for i in 0..self.selections.count(){
            let selection = self.selections.nth_mut(i);
            let change = Document::apply_delete(&mut self.text, selection, semantics);
            if let Operation::Insert{inserted_text} = change.inverse(){
                Document::adjust_subsequent_selections_after_delete(i, &mut self.selections, inserted_text.len());
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
    pub fn backspace(&mut self, semantics: CursorSemantics){
        let selections_before_changes = self.selections.clone();
        let mut changes = Vec::new();
        let mut sum = 0;

        for selection in self.selections.iter_mut(){
            *selection = Selection::new(selection.anchor().saturating_sub(sum), selection.head().saturating_sub(sum));
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
                changes.push(Document::apply_delete(&mut self.text, selection, semantics));
            }else{
                if is_deletable_soft_tab{
                    //// move cursor to start of soft_tab and extend selection to encompass soft_tab
                    Document::shift_and_extend_selection(&self.text, TAB_WIDTH, selection, semantics);
                    // delete soft tab
                    changes.push(Document::apply_delete(&mut self.text, selection, semantics));
                }
                else if selection.cursor(semantics) > 0{
                    *selection = selection.move_left(&self.text, semantics);
                    changes.push(Document::apply_delete(&mut self.text, selection, semantics));
                }
            }

            if let Some(change) = changes.last(){
                if let Operation::Insert{inserted_text} = change.inverse(){
                    sum += inserted_text.len();
                }
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
    pub fn cut(&mut self, semantics: CursorSemantics){  //-> Result<(), Error>  if multiple selections
        //assert!(self.selections.count() == 1);    // if multiple selections, trigger warning  //prob to be done in client code
        let selections_before_changes = self.selections.clone();
        let selection = self.selections.primary_mut();

        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();

        // Remove the selected text from the document
        let change = Document::apply_delete(&mut self.text, selection, semantics);

        // push changes to undo stack
        self.undo_stack.push(ChangeSet::new(vec![change], selections_before_changes, self.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        self.redo_stack.clear();
    }

    /// Copy single selection to clipboard.
    /// Ensure single selection when calling this function.
    pub fn copy(&mut self){ //-> Result<(), Error>  if multiple selections
        //assert!(self.selections.count() == 1);    // if multiple selections, trigger warning  //prob to be done in client code
        
        let selection = self.selections.primary().clone();

        // Copy the selected text to the clipboard
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();
    }

    /// Insert clipboard contents at cursor position(s).
    pub fn paste(&mut self, semantics: CursorSemantics){
        self.insert_string(&self.clipboard.clone(), semantics);
    }
}




#[cfg(test)]
mod tests{
    use ropey::Rope;
    use crate::document::Document;
    use crate::selection::{Selection, Selections, CursorSemantics};
    //use crate::view::View;
    use crate::history::{Change, ChangeSet, Operation};

    impl Document{
        ///// Add [Rope]-based text to an existing instance of [Document]. Only for testing.
        //pub fn with_text(mut self, text: Rope) -> Self{
        //    self.text = text.clone();
        //    self.last_saved_text = text;
        //    self
        //}
        ///// Add [Selections] to an existing instance of [Document]. Only for testing.
        //pub fn with_selections(mut self, selections: Selections) -> Self{
        //    self.selections = selections;
        //    self
        //}
        ///// Add a [View] to an existing instance of [Document]. Only for testing.
        //pub fn with_view(mut self, view: View) -> Self{
        //    self.client_view = view;
        //    self
        //}
        /// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.
        pub fn with_clipboard(mut self, clipboard: String) -> Self{
            self.clipboard = clipboard;
            self
        }
        pub fn with_undo_stack(mut self, undo_stack: Vec<ChangeSet>) -> Self{
            self.undo_stack = undo_stack;
            self
        }
        pub fn with_redo_stack(mut self, redo_stack: Vec<ChangeSet>) -> Self{
            self.redo_stack = redo_stack;
            self
        }
        pub fn with_last_saved_text(mut self, last_saved_text: Rope) -> Self{
            self.last_saved_text = last_saved_text;
            self
        }
    }

    /*
    TODO:
        do all tests with block and bar cursor semantics

        test insert with hard tab
        test insert with soft tab

        cut/undo/redo
        copy
        paste/undo/redo
    */
    
    //#[test]
    //fn undo_insert_single_char_with_multi_selection(){
    //    let last_saved_text = Rope::from("some\nshit\n");
    //    let text = Rope::from("xsome\nxshit\n");
    //
    //    let semantics = CursorSemantics::Block;
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text))
    //        .with_undo_stack(
    //            vec![
    //                ChangeSet::new(
    //                    vec![
    //                        Change::new(operation, old_selection, new_selection, inverse_operation), 
    //                        Change::new(operation, old_selection, new_selection, inverse_operation)
    //                    ], 
    //                    Selections::new(
    //                        vec![
    //                            Selection::new(0, 1), 
    //                            Selection::new(5, 6)
    //                        ], 
    //                        0, 
    //                        &text
    //                    ), 
    //                    Selections::new(
    //                        vec![
    //                            Selection::with_stored_line_position(1, 2, 1), 
    //                            Selection::with_stored_line_position(7, 8, 1)
    //                        ], 
    //                        0, 
    //                        &text
    //                    )
    //                )
    //            ]
    //        )
    //        .with_last_saved_text(last_saved_text);
    //    let _ = doc.undo(semantics);
    //    assert_eq!("some\nshit\n", doc.text());
    //    assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text), doc.selections());
    //    assert!(!doc.is_modified());
    //}

    #[test]
    fn insert_single_char_with_multi_selection(){
        let text = Rope::from("some\nshit\n");
        
        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("some\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("some\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn insert_multi_char_with_multi_selection(){
        let text = Rope::from("some\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
        doc.insert_string("idk\n", semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 0), Selection::with_stored_line_position(13, 14, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("some\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 0), Selection::with_stored_line_position(13, 14, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
        doc.insert_string("idk\n", semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 0), Selection::with_stored_line_position(13, 13, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("some\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 0), Selection::with_stored_line_position(13, 13, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_forward_single_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(9, 10)], 0, &text));
        doc.delete(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(9, 10)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(9, 9)], 0, &text));
        doc.delete(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 0), Selection::new(9, 9)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_forward_multi_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.delete(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.delete(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_backward_single_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(1, 2), Selection::new(10, 11)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(1, 2), Selection::new(10, 11)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(1, 1), Selection::new(10, 10)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(1, 1), Selection::new(10, 10)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_backward_multi_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn replace_same_len_with_multi_selection(){
        // redo replace (multi selection with replacement string same len as selected)
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("wut", semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 4, 3), Selection::with_stored_line_position(12, 13, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 4, 3), Selection::with_stored_line_position(12, 13, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("wut", semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 3, 3), Selection::with_stored_line_position(12, 12, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 3, 3), Selection::with_stored_line_position(12, 12, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn replace_more_chars_with_multi_selection(){
        // redo replace (multi selection with replacement string more chars than selected)
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("shit", semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 4), Selection::with_stored_line_position(14, 15, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 4), Selection::with_stored_line_position(14, 15, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("shit", semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 4), Selection::with_stored_line_position(14, 14, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 4), Selection::with_stored_line_position(14, 14, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn replace_less_chars_with_multi_selection(){
        // redo replace (multi selection with replacement string less chars than selected)
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("x\nsome\nxt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(8, 9, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("x\nsome\nxt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(8, 9, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn undo_with_nothing_on_stack_errors(){
        let mut doc = Document::new(CursorSemantics::Bar);
        assert!(doc.undo(CursorSemantics::Bar).is_err());
    }

    #[test]
    fn redo_with_nothing_on_stack_errors(){
        let mut doc = Document::new(CursorSemantics::Bar);
        assert!(doc.redo(CursorSemantics::Bar).is_err());
    }

    ////////////////////////////////////////////////////////////////////// Insert ///////////////////////////////////////////////////////////////////////////
    #[test]
    fn idk_insert_single_char_with_multi_selection(){
        let text = Rope::from("some\nshit\n");
        
        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }
    ////////////////////////////////////////////////////////////////////// Insert ///////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////// Delete ///////////////////////////////////////////////////////////////////////////
    fn delete_test(name: &str, selection: Selection, expected_selection: Selection, expected_text: Rope, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        let _ = doc.delete(semantics);
        println!("{:#?}\n{:#?}\nexpected_text {:#?}\ngot: {:#?}\nexpected_selection: {:#?}\ngot: {:#?}\n", name, semantics, expected_text, doc.text().clone(), expected_selection, doc.selections().primary().clone());
        doc.text().clone() == expected_text &&
        doc.selections().primary().clone() == expected_selection
    }
    
    #[test]
    fn delete_will_not_delete_past_end_of_doc(){
        assert!(delete_test("test1", Selection::new(14, 14), Selection::new(14, 14), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test1", Selection::new(14, 15), Selection::new(14, 15), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block)); //idk\nsome\nshit\n|: >
    }
    #[test]
    fn delete_with_no_selection(){
        assert!(delete_test("test2", Selection::new(0, 0), Selection::with_stored_line_position(0, 0, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test2", Selection::new(0, 1), Selection::with_stored_line_position(0, 1, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));    //|:i>dk\nsome\nshit\n
    }
    #[test]
    fn delete_with_selection_head_greater_than_anchor(){
        assert!(delete_test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 0, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 1, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block)); //|i:d>k\nsome\nshit\n
    }
    #[test]
    fn delete_with_selection_head_less_than_anchor(){
        assert!(delete_test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 1, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 2, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Block));    //i|d:k>\nsome\nshit\n
    }
    #[test]
    fn delete_with_whole_text_selected(){
        assert!(delete_test("test5", Selection::new(0, 13), Selection::with_stored_line_position(0, 0, 0), Rope::from("\n"), CursorSemantics::Bar));  //just verifying...
        assert!(delete_test("test5", Selection::new(0, 14), Selection::with_stored_line_position(0, 0, 0), Rope::from(""), CursorSemantics::Bar));
        assert!(delete_test("test5", Selection::new(0, 15), Selection::with_stored_line_position(0, 1, 0), Rope::from(""), CursorSemantics::Block));  //|idk\nsome\nshit\n: >
    }
    #[test]
    fn delete_at_1_less_doc_end(){
        assert!(delete_test("test6", Selection::new(13, 13), Selection::with_stored_line_position(13, 13, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
        assert!(delete_test("test6", Selection::new(13, 14), Selection::with_stored_line_position(13, 14, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit|:\n> //idk\nsome\nshit|: >
    }
    ////////////////////////////////////////////////////////////////////// Delete ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////// Backspace ///////////////////////////////////////////////////////////////////////////
    fn backspace_test(name: &str, selection: Selection, expected: Rope, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        doc.backspace(semantics);
        println!("{:#?}\n{:#?}\nexpected: {:#?}\ngot: {:#?}\n", name, semantics, expected, doc.text().clone());
        doc.text().clone() == expected
    }
    
    #[test]
    fn backspace_does_nothing_at_doc_start(){
        assert!(backspace_test("test0", Selection::new(0, 0), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(backspace_test("test0", Selection::new(0, 1), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block));
    }
    #[test]
    fn backspace_without_selection_deletes_previous_char(){
        assert!(backspace_test("test1", Selection::new(1, 1), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(backspace_test("test1", Selection::new(1, 2), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));   //i|:d>k\nsome\nshit\n
    }
    #[test]
    fn backspace_at_line_start_appends_current_line_to_end_of_previous_line(){
        assert!(backspace_test("test2", Selection::new(4, 4), Rope::from("idksome\nshit\n"), CursorSemantics::Bar));
        assert!(backspace_test("test2", Selection::new(4, 5), Rope::from("idksome\nshit\n"), CursorSemantics::Block));
    }
    #[test]
    fn backspace_with_selection_head_greater_than_anchor(){
        assert!(backspace_test("test3", Selection::new(0, 2), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(backspace_test("test3", Selection::new(0, 2), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block));
    }
    #[test]
    fn backspace_with_selection_head_less_than_anchor(){
        assert!(backspace_test("test4", Selection::new(2, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(backspace_test("test4", Selection::new(2, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block));
    }
    #[test]
    fn backspace_at_text_end(){
        assert!(backspace_test("test5", Selection::new(14, 14), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
        assert!(backspace_test("test5", Selection::new(14, 15), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit\n|: > //idk\nsome\nshit|: >
    }
    #[test]
    fn backspace_removes_previous_tab(){
        use crate::document::TAB_WIDTH;
        let mut spaces = String::new();
        for _ in 0..TAB_WIDTH{
            spaces.push(' ');
        }
        let text = Rope::from(format!("{}idk\nsome\nshit\n", spaces));
        let semantics = CursorSemantics::Block; //test Bar too
        let selection = Selection::new(TAB_WIDTH, match semantics{CursorSemantics::Bar => TAB_WIDTH, CursorSemantics::Block => TAB_WIDTH.saturating_add(1)});
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        doc.backspace(semantics);
        assert!(doc.text().clone() == Rope::from("idk\nsome\nshit\n"));
        assert!(doc.selections().primary().clone() == Selection::with_stored_line_position(0, match semantics{CursorSemantics::Bar => 0, CursorSemantics::Block => 1}, 0));
    }
    ////////////////////////////////////////////////////////////////////// Backspace ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////// Cut ///////////////////////////////////////////////////////////////////////////
    //# use ropey::Rope;
    //# use edit_core::document::Document;
    //# use edit_core::selection::{Selection, Selections, CursorSemantics};
    
    fn cut_test(selection: Selection, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        doc.cut(semantics);
        println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
        doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection
    
        //TODO: ensure clipboard text is correct as well
    }
    
    #[test]
    fn cut_with_selection_anchor_less_than_head(){
        assert!(cut_test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
        assert!(cut_test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    }

    #[test]
    fn cut_with_selection_anchor_greater_than_head(){
        assert!(cut_test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
        assert!(cut_test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    }
    ////////////////////////////////////////////////////////////////////// Cut ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////// Copy ///////////////////////////////////////////////////////////////////////////
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    
    fn copy_test(selection: Selection, expected: &str, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        doc.copy();
        println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.clipboard());
        doc.clipboard() == expected
    }
    
    #[test]
    fn copy_with_selection_anchor_less_than_head(){
        assert!(copy_test(Selection::new(4, 9), "some\n", CursorSemantics::Bar));
        assert!(copy_test(Selection::new(4, 9), "some\n", CursorSemantics::Block));    //idk\n|some:\n>shit\n
    }
    ////////////////////////////////////////////////////////////////////// Copy ///////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////// Paste ///////////////////////////////////////////////////////////////////////////
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    
    fn paste_test(selection: Selection, string: &str, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text)).with_clipboard(string.to_string());
        doc.paste(semantics);
        println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
        doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection
    }
    
    #[test]
    fn paste(){
        assert!(paste_test(Selection::new(9, 9), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 15, 0), CursorSemantics::Bar));
        assert!(paste_test(Selection::new(9, 10), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 16, 0), CursorSemantics::Block));
    }
    ////////////////////////////////////////////////////////////////////// Paste ///////////////////////////////////////////////////////////////////////////
}
