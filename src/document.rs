//TODO: edits should be handled in reverse_selection_order. this ensures that edits at selection position
//in rope do not effect subsequent selection positions
// e.g. "ab[]cd[]efg" insert char x
    //if not reversed, would result in "abx[]c[]xdefg" because second selection position remains at position 4 in rope
    // if reversed, would result in "abx[]cdx[]efg" because previous selection positions arent effected by later insertions
//we also need to ensure selections are sorted by position/index on the rope. and overlapping selections
//are combined into a single selection

use crate::view::View;
use crate::selection::{Selection, Selections, CursorSemantics, Movement};
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
    //undo_stack: Vec<Operation>,   Operation{Insert, Delete}
    //redo_stack: Vec<Operation>
    last_saved_text: Rope,
    clipboard: String,
}
impl Document{
    pub fn open(path: &PathBuf, cursor_semantics: CursorSemantics) -> Result<Self, Box<dyn Error>>{
        let text = Rope::from_reader(BufReader::new(File::open(&path)?))?;
    
        Ok(Self{
            text: text.clone(),
            file_path: Some(path.clone()),
            modified: false,
            selections: match cursor_semantics{
                CursorSemantics::Bar => Selections::new(vec![Selection::new(0, 0)], 0, &text.clone()),
                CursorSemantics::Block => Selections::new(vec![Selection::new(0, 1)], 0, &text.clone())
            },
            client_view: View::default(),
            last_saved_text: text.clone(),
            clipboard: String::new(),
        })
    }
    pub fn new(cursor_semantics: CursorSemantics) -> Self{
        Self{
            text: Rope::new(),
            file_path: None,
            modified: false,
            selections: match cursor_semantics{
                CursorSemantics::Bar => Selections::new(vec![Selection::new(0, 0)], 0, &Rope::new()),
                CursorSemantics::Block => Selections::new(vec![Selection::new(0, 1)], 0, &Rope::new())
            },
            client_view: View::default(),
            last_saved_text: Rope::new(),
            clipboard: String::new(),
        }
    }
    /// Add [Rope]-based text to an existing instance of [Document]. Only for testing.
    pub fn with_text(&self, text: Rope) -> Self{
        Self{
            text: text.clone(),
            file_path: self.file_path.clone(),
            modified: self.modified,
            selections: self.selections.clone(),
            client_view: self.client_view.clone(),
            last_saved_text: text.clone(),
            clipboard: self.clipboard.clone(),
        }
    }
    /// Add [Selections] to an existing instance of [Document]. Only for testing.
    pub fn with_selections(&self, selections: Selections) -> Self{
        Self{
            text: self.text.clone(),
            file_path: self.file_path.clone(),
            modified: self.modified,
            selections,
            client_view: self.client_view.clone(),
            last_saved_text: self.last_saved_text.clone(),
            clipboard: self.clipboard.clone(),
        }
    }
    /// Add a [View] to an existing instance of [Document]. Only for testing.
    pub fn with_view(&self, view: View) -> Self{
        Self{
            text: self.text.clone(),
            file_path: self.file_path.clone(),
            modified: self.modified,
            selections: self.selections.clone(),
            client_view: view,
            last_saved_text: self.last_saved_text.clone(),
            clipboard: self.clipboard.clone(),
        }
    }
    /// Add [String]-based text to an existing instance of [Document]. Clipboard is scoped to the editor only, not the system clipboard. Only for testing.
    pub fn with_clipboard(&self, clipboard: String) -> Self{
        Self{
            text: self.text.clone(),
            file_path: self.file_path.clone(),
            modified: self.modified,
            selections: self.selections.clone(),
            client_view: self.client_view.clone(),
            last_saved_text: self.last_saved_text.clone(),
            clipboard,
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
    pub fn save(&mut self) -> Result<(), Box<dyn Error>>{
        if let Some(path) = &self.file_path{ // does nothing if path is None
            self.text.write_to(BufWriter::new(fs::File::create(path)?))?;
            
            self.modified = false;
            self.last_saved_text = self.text.clone();
        }
        
        Ok(())
    }
    pub fn is_modified(&self) -> bool{
        self.modified
    }

    /// Creates a new line in the document.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::CursorSemantics;
    /// 
    /// fn test(expected: Rope, semantics: CursorSemantics) -> bool{
    ///     let mut doc = Document::new(semantics).with_text(Rope::from("idk"));
    ///     doc.enter(semantics);
    ///     println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.text().clone());
    ///     doc.text().clone() == expected
    /// }
    /// 
    /// assert!(test(Rope::from("\nidk"), CursorSemantics::Bar));
    /// assert!(test(Rope::from("\nidk"), CursorSemantics::Block));
    /// 
    /// //TODO: test auto indent...
    /// ```
    pub fn enter(&mut self, semantics: CursorSemantics){
        //self.insert_char('\n', semantics);
        for selection in self.selections.iter_mut().rev(){
            (*selection, self.text) = Document::enter_at_cursor(selection.clone(), &self.text, semantics);
        }

        self.modified = !(self.text == self.last_saved_text);
    }
    fn enter_at_cursor(mut selection: Selection, text: &Rope, semantics: CursorSemantics) -> (Selection, Rope){
        //determine indentation level

        // insert newline
        let new_text;
        (selection, new_text) = Document::insert_char_at_cursor(selection.clone(), text, '\n', semantics);

        // if auto indent, insert proper indentation characters

        (selection, new_text)
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
    /// assert!(test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    /// ```
    pub fn cut(&mut self, semantics: CursorSemantics){  //-> Result<(), Error>  if multiple selections
        // if multiple selections, trigger warning  //prob to be done in client code
        //assert!(single selection)
        let selection = self.selections.primary_mut();
        self.clipboard = self.text.slice(selection.start()..selection.end()).to_string();
        // remove from text
        (self.text, *selection) = Document::delete_at_cursor(selection.clone(), &self.text.clone(), semantics);
        // set cursor to selection start
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
        // if multiple selections, trigger warning  //prob to be done in client code
        //assert!(single selection)
        let selection = self.selections.primary().clone();
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
        for selection in self.selections.iter_mut().rev(){
            (*selection, self.text) = Document::insert_string_at_cursor(
                selection.clone(),
                &self.text,
                &self.clipboard,
                semantics
            )
        }
    }
    fn insert_string_at_cursor(mut selection: Selection, text: &Rope, string: &str, semantics: CursorSemantics) -> (Selection, Rope){
        let mut new_text = text.clone();
        if selection.is_extended(semantics){
            (new_text, selection) = Document::delete_at_cursor(selection.clone(), text, semantics);
        }
        new_text.insert(selection.cursor(semantics), string);
        for _ in 0..string.len(){
            selection = selection.move_right(&new_text, semantics);
        }

        (selection, new_text)
    }

    /// Inserts specified char, replacing selected text if selection extended.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// 
    /// fn test(selection: Selection, char: char, expected: Rope, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    ///     doc.insert_char(char, semantics);
    ///     println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.text().clone());
    ///     doc.text().clone() == expected
    /// }
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // normal use. selection not extended
    /// assert!(test(Selection::new(0, 0), 'x', Rope::from("xidk\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test(Selection::new(0, 1), 'x', Rope::from("xidk\nsome\nshit\n"), CursorSemantics::Block));
    /// 
    /// // with selection extended
    /// assert!(test(Selection::new(0, 1), 'x', Rope::from("xdk\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test(Selection::new(0, 2), 'x', Rope::from("xk\nsome\nshit\n"), CursorSemantics::Block));   //|i:d>k\nsome\nshit\n
    /// ```
    pub fn insert_char(&mut self, c: char, semantics: CursorSemantics){
        //TODO: if use auto-pairs and inserted char has a mapped auto-pair character, insert that char as well
        for cursor in self.selections.iter_mut().rev(){
            (*cursor, self.text) = Document::insert_char_at_cursor(
                cursor.clone(), 
                &self.text, 
                c,
                semantics
            );
        }

        self.modified = !(self.text == self.last_saved_text);
    }
    fn insert_char_at_cursor(mut selection: Selection, text: &Rope, char: char, semantics: CursorSemantics) -> (Selection, Rope){
        let mut new_text = text.clone();
        if selection.is_extended(semantics){
            (new_text, selection) = Document::delete_at_cursor(selection.clone(), text, semantics);
        }
        new_text.insert_char(selection.cursor(semantics), char);
        selection.move_right(&new_text, semantics);

        (selection, new_text)
    }

    /// Inserts [TAB_WIDTH] spaces.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::{Document, TAB_WIDTH};
    /// # use edit_core::selection::CursorSemantics;
    /// 
    /// fn test(expected: Rope, semantics: CursorSemantics) -> bool{
    ///     let mut doc = Document::new(semantics).with_text(Rope::from("idk\nsome\nshit\n"));
    ///     doc.tab(semantics);
    ///     println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.text().clone());
    ///     doc.text().clone() == expected
    /// }
    /// 
    /// let mut spaces = String::new();
    /// for x in 0..TAB_WIDTH{
    ///     spaces.push(' ');
    /// }
    /// assert!(test(Rope::from(format!("{}idk\nsome\nshit\n", spaces)), CursorSemantics::Bar));
    /// assert!(test(Rope::from(format!("{}idk\nsome\nshit\n", spaces)), CursorSemantics::Block));  // i think text_util::distance_to_next_multiple_of_tab_width needs to be updated to use selection.cursor() and CursorSemantics
    /// ```
    pub fn tab(&mut self, semantics: CursorSemantics){
        for selection in self.selections.iter_mut().rev(){
            let tab_distance = text_util::distance_to_next_multiple_of_tab_width(selection.clone(), &self.text, semantics);
            let modified_tab_width = if tab_distance > 0 && tab_distance < TAB_WIDTH{
                tab_distance
            }else{
                TAB_WIDTH
            };
            for _ in 0..modified_tab_width{
                (*selection, self.text) = Document::insert_char_at_cursor(
                    selection.clone(), 
                    &self.text, 
                    ' ',
                    semantics
                );
            }
        }

        self.modified = !(self.text == self.last_saved_text);
    }

    /// Deletes selection, or if no selection, the next character.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    /// 
    /// fn test(name: &str, selection: Selection, expected_selection: Selection, expected_text: Rope, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    ///     doc.delete(semantics);
    ///     println!("{:#?}\n{:#?}\nexpected_text {:#?}\ngot: {:#?}\nexpected_selection: {:#?}\ngot: {:#?}\n", name, semantics, expected_text, doc.text().clone(), expected_selection, doc.selections().primary().clone());
    ///     doc.text().clone() == expected_text &&
    ///     doc.selections().primary().clone() == expected_selection
    /// }
    /// 
    /// // will not delete past end of doc
    /// assert!(test("test1", Selection::new(14, 14), Selection::new(14, 14), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test1", Selection::new(14, 15), Selection::new(14, 15), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block)); //idk\nsome\nshit\n|: >
    /// 
    /// // no selection
    /// assert!(test("test2", Selection::new(0, 0), Selection::with_stored_line_position(0, 0, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test2", Selection::new(0, 1), Selection::with_stored_line_position(0, 1, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));    //|:i>dk\nsome\nshit\n
    /// 
    /// // with selection head > anchor
    /// assert!(test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 0, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 1, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block)); //|i:d>k\nsome\nshit\n
    /// 
    /// // with selection head < anchor
    /// assert!(test("test4", Selection::new(1, 3), Selection::with_stored_line_position(1, 1, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Bar));
    /// assert!(test("test4", Selection::new(1, 3), Selection::with_stored_line_position(1, 2, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Block));    //i|d:k>\nsome\nshit\n
    /// //idk... //assert!(test("test3", Selection::new(13, 0), Selection::with_stored_line_position(0, 1, 0), Rope::from("\n"), CursorSemantics::Block)); //<idk\nsome\nshit|\n
    /// 
    /// // with whole text selected
    /// assert!(test("test5", Selection::new(0, 13), Selection::with_stored_line_position(0, 0, 0), Rope::from("\n"), CursorSemantics::Bar));  //just verifying...
    /// assert!(test("test5", Selection::new(0, 14), Selection::with_stored_line_position(0, 0, 0), Rope::from(""), CursorSemantics::Bar));
    /// assert!(test("test5", Selection::new(0, 15), Selection::with_stored_line_position(0, 1, 0), Rope::from(""), CursorSemantics::Block));  //|idk\nsome\nshit\n: >
    /// 
    /// // at 1 less doc end
    /// assert!(test("test6", Selection::new(13, 13), Selection::with_stored_line_position(13, 13, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
    /// assert!(test("test6", Selection::new(13, 14), Selection::with_stored_line_position(13, 14, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit|:\n> //idk\nsome\nshit|: >
    /// ```
    pub fn delete(&mut self, semantics: CursorSemantics){
        for selection in self.selections.iter_mut().rev(){
            (self.text, *selection) = Document::delete_at_cursor(selection.clone(), &self.text, semantics);
        }

        self.modified = !(self.text == self.last_saved_text);
    }
    fn delete_at_cursor(mut selection: Selection, text: &Rope, semantics: CursorSemantics) -> (Rope, Selection){
        let mut new_text = text.clone();

        use std::cmp::Ordering;
        match selection.cursor(semantics).cmp(&selection.anchor()){
            Ordering::Less => { //cursor < anchor
                //i<dk|\nsome\nshit\n   //i|>\nsome\nshit\n
                //i<dk|\nsome\nshit\n   //i|:\n>some\nshit\n
                new_text.remove(selection.head()..selection.anchor());
                selection = selection.put_cursor(selection.cursor(semantics), text, Movement::Move, semantics, true);
            }
            Ordering::Greater => {  //cursor > anchor
                match semantics{
                    CursorSemantics::Bar => {
                        //|id>k\nsome\nshit\n   //|>k\nsome\nshit\n
                        //|idk\nsome\nshit\n>   //|>
                        new_text.remove(selection.anchor()..selection.head());
                        selection = selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
                    }
                    CursorSemantics::Block => {
                        //|idk\nsome\nshit\n: > //|: >
                        if selection.cursor(semantics) == text.len_chars(){
                            new_text.remove(selection.anchor()..selection.cursor(semantics));
                        }
                        //|i:d>k\nsome\nshit\n  //|:k>\nsome\nshit\n
                        else{
                            new_text.remove(selection.anchor()..selection.head());
                        }
                        selection = selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
                    }
                }
            }
            Ordering::Equal => {    //cursor == anchor
                //idk\nsome\nshit\n|>   //idk\nsome\nshit\n|>
                //idk\nsome\nshit\n|: > //idk\nsome\nshit\n|: >
                if selection.cursor(semantics) == text.len_chars(){}    //do nothing
                else{
                    match semantics{
                        CursorSemantics::Bar => {
                            //|>idk\nsome\nshit\n   //|>dk\nsome\nshit\n
                            new_text.remove(selection.head()..selection.head().saturating_add(1));
                        }
                        CursorSemantics::Block => {
                            //|:i>dk\nsome\nshit\n  //|:d>k\nsome\nshit\n
                            new_text.remove(selection.anchor()..selection.head());
                        }
                    }
                    selection = selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
                }
            }
        }
        
//        match semantics{
//            CursorSemantics::Bar => {
//                match selection.head().cmp(&selection.anchor()){
//                    //i<dk|\nsome\nshit\n   //i|>\nsome\nshit\n
//                    std::cmp::Ordering::Less => {
//                        new_text.remove(selection.head()..selection.anchor());
//                        selection.put_cursor(selection.head(), text, Movement::Move, semantics, true);
//                    }
//                    //|id>k\nsome\nshit\n   //|>k\nsome\nshit\n
//                    //|idk\nsome\nshit\n>   //|>
//                    std::cmp::Ordering::Greater => {
//                        new_text.remove(selection.anchor()..selection.head());
//                        selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
//                    }
//                    std::cmp::Ordering::Equal => {
//                        //idk\nsome\nshit\n|>   //idk\nsome\nshit\n|>
//                        if selection.head() == text.len_chars(){}
//                        //|>idk\nsome\nshit\n   //|>dk\nsome\nshit\n
//                        else{
//                            new_text.remove(selection.head()..selection.head().saturating_add(1));
//                            selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
//                        }
//                    }
//                }
//            }
//            CursorSemantics::Block => {
//                match selection.cursor(semantics).cmp(&selection.anchor()){
//                    //i<dk|\nsome\nshit\n   //i\nsome\nshit\n
//                    std::cmp::Ordering::Less => {
//                        new_text.remove(selection.head()..selection.anchor());
//                        selection.put_cursor(selection.cursor(semantics), text, Movement::Move, semantics, true);
//                    }
//                    std::cmp::Ordering::Greater => {
//                        //|idk\nsome\nshit\n: > //|: >
//                        if selection.cursor(semantics) == text.len_chars(){
//                            new_text.remove(selection.anchor()..selection.cursor(semantics));
//                            //selection.put_cursor(selection.cursor(semantics), text, Movement::Move, semantics, true);
//                            selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
//                        }
//                        //|i:d>k\nsome\nshit\n  //|:k>\nsome\nshit\n
//                        else{
//                            new_text.remove(selection.anchor()..selection.head());
//                            selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
//                        }
//                    }
//                    std::cmp::Ordering::Equal => {
//                        //idk\nsome\nshit\n|: > //idk\nsome\nshit\n|: >
//                        if selection.cursor(semantics) == text.len_chars(){}
//                        //|:i>dk\nsome\nshit\n  //|:d>k\nsome\nshit\n
//                        else{
//                            new_text.remove(selection.anchor()..selection.head());
//                            selection.put_cursor(selection.anchor(), text, Movement::Move, semantics, true);
//                        }
//                    }
//                }
//            }
//        }

        (new_text, selection)
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
    pub fn backspace(&mut self, semantics: CursorSemantics){
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
                (self.text, *selection) = Document::delete_at_cursor(selection.clone(), &self.text, semantics);
            }else{
                if is_deletable_soft_tab{
                    for _ in 0..TAB_WIDTH{
                        *selection = selection.move_left(&self.text, semantics);
                        (self.text, *selection) = Document::delete_at_cursor(
                            selection.clone(), 
                            &self.text,
                            semantics
                        );
                    }
                }
                else if selection.cursor(semantics) > 0{
                    *selection = selection.move_left(&self.text, semantics);
                    (self.text, *selection) = Document::delete_at_cursor(
                        selection.clone(), 
                        &self.text,
                        semantics
                    );
                }
            }
        }

        self.modified = !(self.text == self.last_saved_text);
    }
}
