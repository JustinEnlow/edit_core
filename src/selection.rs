// follow documentation style from https://std-dev-guide.rust-lang.org/development/how-to-write-documentation.html
use ropey::Rope;
use crate::{
    text_util, view::View, Position
};



#[derive(Clone, Copy, Debug)]
pub enum CursorSemantics{
    Bar,    //difference between anchor and head is 0
    Block   //difference between anchor and head is 1 grapheme
}
#[derive(PartialEq, Debug)]
pub enum Direction{
    Forward,
    Backward,
}
#[derive(PartialEq)]
pub enum Movement{
    Extend,
    Move,
}
#[derive(Debug, PartialEq)]
pub enum SelectionError{        //or should each fallible fn have its own fn specific Error? this would prevent the calling fn from having to match unused variants in the fallible fn...
    ResultsInSameState,
    NoOverlap,
    InvalidInput,   //as in put_cursor  //to > text.len_chars()
}
/// 1 dimensional representation of a single selection(between anchor and head) within a text rope.
/// a cursor is a selection with an anchor/head difference of 0 or 1(depending on cursor semantics)
/// Should ensure head/anchor are always within text bounds
#[derive(PartialEq, Clone, Debug)]
pub struct Selection{
    anchor: usize,  // the stationary portion of a selection.
    head: usize,    // the mobile portion of a selection. this is the portion a user can move to extend selection
    stored_line_position: Option<usize>,    // the offset from the start of the line self.head is on
}
impl Selection{
    /////////////////////////////////////////////////////////// Only for Testing ////////////////////////////////////////////////////////////////////
    /// Returns a new instance of [`Selection`] with a specified `stored_line_position`.                                                           //
    /**/pub fn with_stored_line_position(anchor: usize, head: usize, stored_line_position: usize) -> Self{                                         //
    /**/    Self{anchor, head, stored_line_position: Some(stored_line_position)}                                                                   //
    /**/}                                                                                                                                          //
    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    /// Returns a new instance of [`Selection`].
    #[must_use]
    pub fn new(anchor: usize, head: usize) -> Self{ // could init with cursor semantics: (anchor: usize, cursor: usize, semantics: CursorSemantics)
        Self{anchor, head, stored_line_position: None}
    }
    
    /// Returns the char index of [`Selection`] anchor.
    #[must_use]
    pub fn anchor(&self) -> usize{self.anchor}
    
    /// Returns the char index of [`Selection`] head.
    #[must_use]
    pub fn head(&self) -> usize{self.head}

    /// Returns the char index of the start of the [`Selection`] from left to right.
    #[must_use]
    pub fn start(&self) -> usize{std::cmp::min(self.anchor, self.head)}
    
    /// Returns the char index of the end of the [`Selection`] from left to right.
    #[must_use]
    pub fn end(&self) -> usize{std::cmp::max(self.anchor, self.head)}

    /// Returns `true` if [`Selection`] len > 0 with bar cursor semantics, or 
    /// [`Selection`] len > 1 with block cursor semantics, or else returns `false`.
    #[must_use]
    pub fn is_extended(&self, semantics: CursorSemantics) -> bool{
        match semantics{
            CursorSemantics::Bar => self.end().saturating_sub(self.start()) > 0,
            CursorSemantics::Block => self.end().saturating_sub(self.start()) > 1  //if selection is greater than one grapheme //currently uses char count though...
        }
    }

    // TODO: impl tests
    /// Returns a bool indicating whether the selection spans multiple lines.
    pub fn spans_multiple_lines(&self, text: &Rope, semantics: CursorSemantics) -> bool{
        text.char_to_line(self.anchor) != text.char_to_line(self.cursor(semantics))
    }

    ///// Returns the direction of [`Selection`].
    //#[must_use]
    //pub fn direction(&self, semantics: CursorSemantics) -> Direction{
    //    //assert!(self.cursor(semantics) <= text.len_chars());  we would need a & to text
    //    //assert!(self.anchor <= text.len_chars());
    //    if self.cursor(semantics) < self.anchor{Direction::Backward}
    //    else{Direction::Forward}
    //}

    ///// Sets [`Selection`] direction to specified direction.
    //#[must_use]
    //pub fn set_direction(&self, direction: Direction, text: &Rope, semantics: CursorSemantics) -> Self{
    //    assert!(self.start() <= self.end());    //i think this is already guaranteed
    //    assert!(text.len_lines() > 0);
    //    
    //    let (anchor, head) = match direction {
    //        Direction::Forward => (self.start(), self.end()),
    //        Direction::Backward => (self.end(), self.start()),
    //    };
    //
    //    let mut selection = Selection::new(anchor, head);
    //    selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(semantics), text));
    //
    //    selection
    //}

    /// Checks `self` and `other` for overlap.
    #[must_use]
    pub fn overlaps(&self, other: &Selection) -> bool{
        self.start() == other.start() || 
        self.end() == other.end() || 
        (self.end() > other.start() && other.end() > self.start())
    }

    // Returns a bool indicating whether the provided char index is contained within the [`Selection`].
    pub fn contains(&self, idx: usize) -> bool{idx >= self.start() && idx <= self.end()}

    /// Returns a new [`Selection`] from the overlap of `self` and `other`.
    /// Returns Error if `self` and `other` are non-overlapping.
    #[allow(clippy::result_unit_err)]
    pub fn intersection(&self, other: &Selection) -> Result<Self, SelectionError>{
        if self.overlaps(other){
            Ok(Selection::new(self.start().max(other.start()), self.end().min(other.end())))
            // Selection{anchor: self.start().max(other.start()), head: self.end().min(other.end()), stored_line_position: text_util::offset_from_line_start(head, text)}   //if we want stored line position too
        }else{Err(SelectionError::NoOverlap)}
    }

    /// Create a new [`Selection`] by merging self with other.
    /// Indiscriminate merge. merges whether overlapping, consecutive, 
    /// contained, or disconnected entirely.
    /// resultant selection should be guaranteed to be within text bounds 
    /// because this uses previously initialized selections.
    #[must_use]
    pub fn merge(&self, other: &Selection, text: &Rope) -> Selection{
        let anchor = self.start().min(other.start());
        let head = self.end().max(other.end());
        let stored_line_position = text_util::offset_from_line_start(head, text);   //self.cursor instead of head?
        
        Selection{anchor, head, stored_line_position: Some(stored_line_position)}
    }
    
    /// Returns the char index of [`Selection`] cursor.
    /// left side of cursor if block cursor semantics
    #[must_use]
    pub fn cursor(&self, semantics: CursorSemantics) -> usize{
        match semantics{
            CursorSemantics::Bar => self.head,
            CursorSemantics::Block => {
                if self.head >= self.anchor{self.head.saturating_sub(1)}
                else{self.head}
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor at specified char index in rope.
    /// Will shift `anchor`/`head` positions to accommodate Bar/Block cursor semantics.
    /// Errors if `to`  > `text.len_chars()`.
    #[must_use]
    pub fn put_cursor(&self, to: usize, text: &Rope, movement: Movement, semantics: CursorSemantics, update_stored_line_position: bool) -> Result<Self, SelectionError>{
        if to > text.len_chars(){return Err(SelectionError::InvalidInput);}
        
        let mut selection = self.clone();
        match (semantics, movement){
            (CursorSemantics::Bar, Movement::Move) => {
                selection.anchor = to;
                selection.head = to;
            }
            (CursorSemantics::Bar, Movement::Extend) => selection.head = to,
            (CursorSemantics::Block, Movement::Move) => {
                selection.anchor = to;
                selection.head = to.saturating_add(1).min(text.len_chars().saturating_add(1));   //allowing one more char past text.len_chars() for block cursor
            }
            (CursorSemantics::Block, Movement::Extend) => {
                let new_anchor = if self.head >= self.anchor && to < self.anchor{   //if direction forward and to < self.anchor
                    if let Some(char_at_cursor) = text.get_char(self.cursor(semantics)){
                        if char_at_cursor == '\n'{
                            self.anchor
                        }else{
                            self.anchor.saturating_add(1).min(text.len_chars())
                        }
                    }else{
                        self.anchor.saturating_add(1).min(text.len_chars())
                    }
                }else if self.head < self.anchor && to >= self.anchor{  //if direction backward and to >= self.anchor
                    self.anchor.saturating_sub(1)
                }else{
                    self.anchor
                };

                if new_anchor <= to{
                    selection.anchor = new_anchor;
                    selection.head = to.saturating_add(1).min(text.len_chars().saturating_add(1))    //allowing one more char past text.len_chars() for block cursor
                }else{
                    selection.anchor = new_anchor;
                    selection.head = to;
                }
            }
        }
        if update_stored_line_position{
            selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(semantics), text));
        }

        assert!(selection.anchor <= text.len_chars());                  //is this needed?
        assert!(selection.cursor(semantics) <= text.len_chars());       //is this needed?

        Ok(selection)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved vertically by specified amount.
    /// Errors if `amount` < 1, or calculated new position is invalid.
    #[must_use]
    pub fn move_vertically(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if amount < 1{return Err(SelectionError::InvalidInput);}    // really this should be SelectionError::ResultsInSameState
        
        let mut selection = self.clone();
        
        let current_line = text.char_to_line(self.cursor(semantics));
        let goal_line_number = match direction{
            Direction::Forward => (current_line + amount).min(text.len_lines().saturating_sub(1)),
            Direction::Backward => current_line.saturating_sub(amount),
        };

        let start_of_line = text.line_to_char(goal_line_number);
        let line_width = text_util::line_width(text.line(goal_line_number), false);
    
        // Use the stored line position or calculate it if None
        let stored_line_position = self.stored_line_position.unwrap_or_else(|| {
            text_util::offset_from_line_start(self.cursor(semantics), text)
        });

        // Calculate the new position based on line width
        let new_position = if stored_line_position < line_width{
            start_of_line + stored_line_position
        }else{
            start_of_line + line_width
        };

        selection.stored_line_position = Some(stored_line_position);
        selection.put_cursor(new_position, text, movement, semantics, false)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved horizontally by specified amount.
    /// Errors if `amount` < 1, or calculated new position is invalid.
    #[must_use]
    pub fn move_horizontally(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if amount < 1{return Err(SelectionError::InvalidInput);}    // really this should be SelectionError::ResultsInSameState
        
        let new_position = match direction{
            //Direction::Forward => self.cursor(semantics).saturating_add(amount).min(text.len_chars()),    //ensures this does not move past text end
            Direction::Forward => {
                let mut index = self.cursor(semantics);
                for _ in 0..amount{
                    index = text_util::next_grapheme_index(index, text);
                }
                index.min(text.len_chars()) //ensures this does not move past text end
            }
            //Direction::Backward => self.cursor(semantics).saturating_sub(amount)
            Direction::Backward => {
                let mut index = self.cursor(semantics);
                for _ in 0..amount{
                    index = text_util::previous_grapheme_index(index, text);
                }
                index
            }
        };
        self.put_cursor(new_position, text, movement, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor set to specified 0-based line number.
    #[must_use]
    pub fn set_from_line_number(&self, line_number: usize, text: &Rope, movement: Movement, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if line_number >= text.len_lines(){return Err(SelectionError::InvalidInput);}
        
        let current_line = text.char_to_line(self.cursor(semantics));
        let (amount, direction) = if line_number < current_line{
            (current_line.saturating_sub(line_number), Direction::Backward)
        }else{
            (line_number.saturating_sub(current_line), Direction::Forward)
        };
        self.move_vertically(amount, text, movement, direction, semantics)
    }

    /// Returns a new instance of [`Selection`] with `anchor` aligned with cursor.
    #[must_use]
    pub fn collapse(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if !self.is_extended(semantics){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(self.cursor(semantics), text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved right.
    #[must_use]
    pub fn move_right(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Move, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved right to the nearest word boundary.
    pub fn move_right_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::next_word_boundary(self.cursor(semantics), text);
        let offset = goal_index.saturating_sub(self.cursor(semantics));
        
        match semantics{
            CursorSemantics::Bar => {
                self.move_horizontally(offset, text, Movement::Move, Direction::Forward, semantics)
            }
            CursorSemantics::Block => {
                if offset.saturating_sub(1) > 0{
                    self.move_horizontally(offset.saturating_sub(1), text, Movement::Move, Direction::Forward, semantics)
                }else{
                    self.move_horizontally(offset, text, Movement::Move, Direction::Forward, semantics)
                }
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor moved left.
    #[must_use]
    pub fn move_left(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Move, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved left to the nearest word boundary.
    pub fn move_left_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::previous_word_boundary(self.cursor(semantics), text);
        let offset = self.cursor(semantics).saturating_sub(goal_index);
        
        match semantics{
            CursorSemantics::Bar => {
                self.move_horizontally(offset, text, Movement::Move, Direction::Backward, semantics)
            }
            CursorSemantics::Block => {
                if offset.saturating_sub(1) > 0 && goal_index != 0{
                    self.move_horizontally(offset.saturating_sub(1), text, Movement::Move, Direction::Backward, semantics)
                }else{
                    self.move_horizontally(offset, text, Movement::Move, Direction::Backward, semantics)
                }
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor moved up.
    #[must_use]
    pub fn move_up(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Move, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved down.
    #[must_use]
    pub fn move_down(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Move, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved to line end.
    #[must_use]
    pub fn move_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line = text.line(line_number);
        let line_width = text_util::line_width(line, false);
        let line_start = text.line_to_char(line_number);
        let line_end = line_start.saturating_add(line_width);

        if self.cursor(semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(line_end, text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved to absolute start of line, or start of line text, depending on current cursor position.
    #[must_use]
    pub fn move_home(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        //if text_start == line_start && self.cursor(semantics) == line_start{return Err(());}    //would result in same state    //TODO: test
        if self.cursor(semantics) == text_start{self.move_line_start(text, semantics)}
        else{self.move_line_text_start(text, semantics)}
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the current line.
    #[must_use]
    pub fn move_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);

        if self.cursor(semantics) == line_start{return Err(SelectionError::ResultsInSameState);}    //TODO: test
        self.put_cursor(line_start, text, Movement::Move, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the text on the current line.
    #[must_use]
    pub fn move_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        if self.cursor(semantics) == text_start{return Err(SelectionError::ResultsInSameState);}    //TODO: test
        self.put_cursor(text_start, text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved up by the height of `client_view`.
    #[must_use]
    pub fn move_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved down by the height of `client_view`.
    #[must_use]
    pub fn move_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the document.
    #[must_use]
    pub fn move_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(0, text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the end of the document.
    #[must_use]
    pub fn move_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text.len_chars(), text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the right.
    #[must_use]
    pub fn extend_right(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the left.
    #[must_use]
    pub fn extend_left(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up.
    #[must_use]
    pub fn extend_up(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down.
    #[must_use]
    pub fn extend_down(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the end of the current line.
    #[must_use]
    pub fn extend_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line = text.line(line_number);
        let line_width = text_util::line_width(line, false);    //doesn't include newline
        let line_start = text.line_to_char(line_number);
        let line_end = line_start.saturating_add(line_width);   //index at end of line text, not including newline

        match semantics{
            CursorSemantics::Bar => {
                if self.cursor(semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
                self.put_cursor(line_end, text, Movement::Extend, semantics, true)
            }
            CursorSemantics::Block => {
                if self.cursor(semantics) == line_end.saturating_sub(1)
                || self.cursor(semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
                let start_line = text.char_to_line(self.start());
                let end_line = text.char_to_line(self.end());
                if self.cursor(semantics) == self.start() && end_line > start_line{
                    self.put_cursor(line_end, text, Movement::Extend, semantics, true)  //put cursor over newline, if extending from a line below
                }else{
                    self.put_cursor(line_end.saturating_sub(1), text, Movement::Extend, semantics, true)
                }
                
            }
        }
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to absolute start of line, or line text start, depending on [`Selection`] `head` position.
    #[must_use]
    pub fn extend_home(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        //if text_start == line_start && self.cursor(semantics) == line_start{return Err(());}    //would result in same state
        if self.cursor(semantics) == text_start{self.extend_line_start(text, semantics)}
        else{self.extend_line_text_start(text, semantics)}
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the current line.
    #[must_use]
    pub fn extend_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);

        if self.cursor(semantics) == line_start{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(line_start, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the text on the current line.
    #[must_use]
    pub fn extend_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        if self.cursor(semantics) == text_start{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text_start, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up by the height of `client_view`.
    #[must_use]
    pub fn extend_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Backward, semantics)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down by the height of `client_view`.
    #[must_use]
    pub fn extend_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Forward, semantics)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc start.
    #[must_use]
    pub fn extend_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(0, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc end.
    #[must_use]
    pub fn extend_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with [`Selection`] extended to encompass all text.
    #[must_use]
    pub fn select_all(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.start() == 0 && (self.end() == text.len_chars() || self.end() == text.len_chars().saturating_add(1)){return Err(SelectionError::ResultsInSameState);}
        let selection = self.put_cursor(0, text, Movement::Move, semantics, true)?;
        selection.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    }

    // should this be made purely functional?
    pub fn shift_and_extend(&mut self, amount: usize, text: &Rope, semantics: CursorSemantics){ //-> Result<(), SelectionError>{    //should this pass up possible errors from move/extend calls?
        for _ in 0..amount{
            //*self = self.move_left(text, semantics);  //or *self = self.move_left(text, semantics)?;
            if let Ok(new_selection) = self.move_left(text, semantics){
                *self = new_selection;
            }
        }
        if amount > 1{
            match semantics{
                CursorSemantics::Bar => {
                    for _ in 0..amount{
                        //*self = self.extend_right(text, semantics);
                        if let Ok(new_selection) = self.extend_right(text, semantics){
                            *self = new_selection;
                        }
                    }
                }
                CursorSemantics::Block => {
                    for _ in 0..amount.saturating_sub(1){
                        //*self = self.extend_right(text, semantics);
                        if let Ok(new_selection) = self.extend_right(text, semantics){
                            *self = new_selection;
                        }
                    }
                }
            }
        }
    }

    /// Translates a [`Selection`] to a [Selection2d].
    #[must_use]
    pub fn selection_to_selection2d(&self, text: &Rope, semantics: CursorSemantics) -> Selection2d{
        let line_number_head = text.char_to_line(self.cursor(semantics));
        let line_number_anchor = text.char_to_line(self.anchor);

        let head_line_start_idx = text.line_to_char(line_number_head);
        let anchor_line_start_idx = text.line_to_char(line_number_anchor);

        //let mut column_head = 0;
        //for grapheme in text.slice(head_line_start_idx..self.cursor(semantics)).to_string().graphemes(true){
        //    if grapheme == "\t"{
        //        column_head += TAB_WIDTH - (column_head % TAB_WIDTH);
        //    }else{
        //        column_head += 1;
        //    }
        //}
        //let mut column_anchor = 0;
        //for grapheme in text.slice(anchor_line_start_idx..self.anchor).to_string().graphemes(true){
        //    if grapheme == "\t"{
        //        column_anchor += TAB_WIDTH - (column_head % TAB_WIDTH);
        //    }else{
        //        column_anchor += 1;
        //    }
        //}
        Selection2d::new(
            Position::new(
                self.anchor.saturating_sub(anchor_line_start_idx),
                //column_anchor,
                line_number_anchor
            ),
            Position::new(
                self.cursor(semantics).saturating_sub(head_line_start_idx),
                //column_head,
                line_number_head
            ) 
        )
    }
}



/// 2 dimensional representation of a single selection(between anchor and head) within document text
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Selection2d{
    anchor: Position,
    head: Position, //TODO: should this be cursor? because we are using cursor in selection_to_selection2d...
}
impl Selection2d{
    pub fn new(anchor: Position, head: Position) -> Self{
        Self{
            anchor,
            head
        }
    }
    pub fn head(&self) -> &Position{
        &self.head
    }
    pub fn anchor(&self) -> &Position{
        &self.anchor
    }
}



#[derive(Debug, PartialEq)]
pub enum SelectionsError{
    SingleSelection,
    MultipleSelections,
    SpansMultipleLines,
    CannotAddSelectionAbove,
    CannotAddSelectionBelow,
}
/// A collection of [`Selection`]s. 
/// used in place of [Vec]<[`Selection`]> to ensure certain guarantees are enforced
/// ## Goal Guarantees:
/// - will always contain at least 1 {Selection}
/// - all {Selection}s are grapheme aligned
/// - all {Selection}s are sorted by increasing position in document
/// - all overlapping {Selection}s are merged
    //should this be handled in {Selection}?
/// - head and anchor are always within text boundaries for each selection
    //
/// - ...prob others i haven't thought of yet
#[derive(Debug, PartialEq, Clone)]
pub struct Selections{
    selections: Vec<Selection>,
    primary_selection_index: usize,
}
impl Selections{
    /// Returns new instance of [`Selections`] from provided input.
    /// #### Invariants:
    /// - will alway contain at least one [`Selection`]
    /// - [`Selection`]s are grapheme aligned
    /// - [`Selection`]s are sorted by ascending position in doc
    /// - overlapping [`Selection`]s are merged
    /// - all [`Selection`]s are within doc boundaries
    /// 
    /// # Panics
    /// `new` panics if `selections` input param is empty.
    pub fn new(selections: Vec<Selection>, primary_selection_index: usize, text: &Rope) -> Self{
        assert!(!selections.is_empty());
        //if selections.is_empty(){
        //    selections = vec![Selection::new(0, 0)];
        //    primary_selection_index = 0;
        //}

        let mut selections = Self{
            selections,
            primary_selection_index,
        };

        // selections.grapheme_align();
        selections = selections.sort();
        selections = selections.merge_overlapping(text);

        assert!(selections.count() > 0);
        selections
    }
    /// Returns the number of [`Selection`]s in [`Selections`].
    pub fn count(&self) -> usize{
        self.selections.len()
    }
    pub fn primary_selection_index(&self) -> usize{
        self.primary_selection_index
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Selection>{
        self.selections.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Selection>{
        self.selections.iter_mut()
    }
    /// Returns a new instance of [`Selections`] with the last element removed.
    pub fn pop(&self) -> Self{
        let mut new_selections = self.selections.clone();
        // Guarantee at least one selection
        if new_selections.len() > 1{new_selections.pop();}
        else{return self.clone();}

        // Is there a better way to determine new primary selection?
        let primary_selection_index = new_selections.len().saturating_sub(1);

        Self{
            selections: new_selections,
            primary_selection_index
        }
    }

    /// Prepends a [`Selection`] to the front of [Self], updating `primary_selection_index` if desired.
    pub fn push_front(&self, selection: Selection, update_primary: bool) -> Self{
        let mut new_selections = self.selections.clone();
        new_selections.insert(0, selection);
        Self{
            selections: new_selections,
            primary_selection_index: if update_primary{0}else{self.primary_selection_index.saturating_add(1)} //0
        }
    }
    
    /// Appends a [`Selection`] to the back of [Self], updating `primary_selection_index` if desired.
    pub fn push(&self, selection: Selection, update_primary: bool) -> Self{
        let mut new_selections = self.selections.clone();
        new_selections.push(selection);
        let primary_selection_index = new_selections.len().saturating_sub(1);
        Self{
            selections: new_selections,
            primary_selection_index: if update_primary{primary_selection_index}else{self.primary_selection_index}
        }
    }
    
    /// Returns a reference to the [`Selection`] at `primary_selection_index`.
    pub fn primary(&self) -> &Selection{
        &self.selections[self.primary_selection_index]
    }
    /// Returns a mutable reference to the [`Selection`] at `primary_selection_index`.
    pub fn primary_mut(&mut self) -> &mut Selection{
        &mut self.selections[self.primary_selection_index]
    }
    pub fn first(&self) -> &Selection{
        // unwrapping because we ensure at least one selection is always present
        self.selections.first().unwrap()
    }
    //pub fn first_mut(&mut self) -> &mut Selection{
    //    self.selections.first_mut().unwrap()
    //}
    pub fn last(&self) -> &Selection{
        // unwrapping because we ensure at least one selection is always present
        self.selections.last().unwrap()
    }
    pub fn nth_mut(&mut self, index: usize) -> &mut Selection{
        self.selections.get_mut(index).unwrap()
    }

    /// Increments `primary_selection_index`.
    #[must_use]
    pub fn increment_primary_selection(&self) -> Result<Self, SelectionsError>{
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}
        if self.primary_selection_index.saturating_add(1) < self.count(){
            Ok(Self{selections: self.selections.clone(), primary_selection_index: self.primary_selection_index + 1})
        }else{
            Ok(Self{selections: self.selections.clone(), primary_selection_index: 0})
        }
    }
    /// Decrements the primary selection index.
    #[must_use]
    pub fn decrement_primary_selection(&self) -> Result<Self, SelectionsError>{
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}
        if self.primary_selection_index > 0{
            Ok(Self{selections: self.selections.clone(), primary_selection_index: self.primary_selection_index - 1})
        }else{
            Ok(Self{selections: self.selections.clone(), primary_selection_index: self.count().saturating_sub(1)})
        }
    }

    /// Sorts each [`Selection`] in [Selections] by position.
    /// #### Invariants:
    /// - preserves primary selection through the sorting process
    #[must_use]
    pub fn sort(&self) -> Self{
        if self.count() < 2{return self.clone();}

        let primary = self.primary().clone();
        let mut sorted_selections = self.selections.clone();
        sorted_selections.sort_unstable_by_key(Selection::start);
    
        let primary_selection_index = sorted_selections
            .iter()
            .position(|selection| selection == &primary)
            .unwrap_or(0);
    
        Self{
            selections: sorted_selections,
            primary_selection_index,
        }
    }

    /// Merges overlapping [`Selection`]s.
    pub fn merge_overlapping(&mut self, text: &Rope) -> Self{
        if self.count() < 2{return self.clone();}   //should this error instead?...

        let mut primary = self.primary().clone();
        let mut new_selections = self.selections.clone();
        new_selections.dedup_by(|current_selection, prev_selection|{
                if prev_selection.overlaps(current_selection){
                    let merged_selection = current_selection.merge(prev_selection, text);

                    // Update primary selection to track index in next code block // Only clone if necessary
                    if prev_selection == &primary || current_selection == &primary{
                        primary = merged_selection.clone();
                    }
            
                    *prev_selection = merged_selection;
                    true
                }else{
                    false
                }
            });

        let primary_selection_index = new_selections.iter()
            .position(|selection| selection == &primary)
            .unwrap_or(0);

        assert!(self.count() > 0);

        Self{
            selections: new_selections,
            primary_selection_index,
        }
    }

    /// Removes all [`Selection`]s except [`Selection`] at `primary_selection_index`.
    /// Errors if [`Selections`] has only 1 [`Selection`].
    pub fn clear_non_primary_selections(&self) -> Result<Self, SelectionsError>{
        //assert!(self.count() > 1);
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}
        
        let primary_as_vec = vec![self.primary().clone()];
        assert!(primary_as_vec.len() == 1);
        
        Ok(Self{
            selections: primary_as_vec,
            primary_selection_index: 0
        })
    }

    //TODO: return head and anchor positions
    //TODO: return Vec<Position> document cursor positions
    //pub fn cursor_positions(&self, text: &Rope, semantics: CursorSemantics) -> Position{
    //    let cursor = self.primary();
    //    let document_cursor = cursor.selection_to_selection2d(text, semantics);
    //    
    //    Position::new(
    //        document_cursor.head().x().saturating_add(1), 
    //        document_cursor.head().y().saturating_add(1)
    //    )
    //}

    /// Adds a new [`Selection`] directly above the top-most [`Selection`], with the same start and end offsets from line start, if possible.
    // TODO: make new selection's head/anchor ordering match current selection's
    // TODO: test/make sure newlines are being handled correctly
    // TODO: test/make sure end of file is handled correctly
    // TODO: view doesn't follow top selection if we don't update primary selection, but if we do, clear non primary selections doesn't put us back to desired location...
        // frontend could call a scroll and update fn that follows the top-most selection instead of the primary...
    #[allow(clippy::result_unit_err)]
    pub fn add_selection_above(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionsError>{ //TODO: define possible errors
        assert!(self.count() > 0);  //ensure at least one selection in selections
        // should error if any selection spans multiple lines. //callee can determine appropriate response behavior in this case        //vscode behavior is to extend topmost selection up one line if any selection spans multiple lines
        for selection in self.selections.iter(){
            if selection.spans_multiple_lines(text, semantics){return Err(SelectionsError::SpansMultipleLines);}
        }
        let top_selection = self.first();
        let top_selection_line = text.char_to_line(top_selection.anchor);
        if top_selection_line == 0{
            return Err(SelectionsError::CannotAddSelectionAbove);
        }

        let anchor_offset = text_util::offset_from_line_start(self.primary().anchor, text);
        let head_offset = text_util::offset_from_line_start(self.primary().cursor(semantics), text);
        
        let line_above = top_selection_line.saturating_sub(1);
        let line_start = text.line_to_char(line_above);
        
        let line_text = text.line(line_above).to_string();
        let (start, end) = if line_text.is_empty() || line_text == "\n"{
            (line_start, line_start)
        }
        else if !self.primary().is_extended(semantics){ // not extended
            (line_start.saturating_add(anchor_offset), line_start.saturating_add(anchor_offset))
        }
        else{
            let line_width = text_util::line_width(text.line(line_above), false);
            // if anchor < cursor
            (line_start.saturating_add(anchor_offset), line_start.saturating_add(head_offset.min(line_width)))
            // else if cursor < anchor
            // (line_start.saturating_add(head_offset), line_start.saturating_add(anchor_offset.min(line_width)))
            // else // should not be reachable. already handled by !is_extended
            // unreachable!
        };

        match semantics{
            CursorSemantics::Bar => {Ok(self.push_front(Selection::new(start, end), false))}
            CursorSemantics::Block => {Ok(self.push_front(Selection::new(start, end.saturating_add(1)), false))}
        }
    }

    // TODO: selection added below at text end is not rendering on last line
    /// Adds a new [`Selection`] directly below bottom-most [`Selection`], with the same start and end offsets from line start, if possible.
    pub fn add_selection_below(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionsError>{
        assert!(self.count() > 0);  //ensure at least one selection in selections
        // should error if any selection spans multiple lines. //callee can determine appropriate response behavior in this case        //vscode behavior is to extend topmost selection down one line if any selection spans multiple lines
        for selection in self.selections.iter(){
            if selection.spans_multiple_lines(text, semantics){return Err(SelectionsError::SpansMultipleLines);}
        }
        let bottom_selection = self.last();
        let bottom_selection_line = text.char_to_line(bottom_selection.anchor);
        if bottom_selection_line >= text.len_lines().saturating_sub(1){ //bottom_selection_line must be zero based, and text.len_lines() one based...   //TODO: verify
            return Err(SelectionsError::CannotAddSelectionBelow);
        }
        
        let anchor_offset = text_util::offset_from_line_start(self.primary().anchor, text);
        let head_offset = text_util::offset_from_line_start(self.primary().cursor(semantics), text);

        let line_below = bottom_selection_line.saturating_add(1);
        let line_start = text.line_to_char(line_below);

        let line_text = text.line(line_below).to_string();
        let (start, end) = if line_text.is_empty() || line_text == "\n"{
            (line_start, line_start)
        }
        else if !self.primary().is_extended(semantics){
            (line_start.saturating_add(anchor_offset), line_start.saturating_add(anchor_offset))
        }
        else{
            let line_width = text_util::line_width(text.line(line_below), false);
            (line_start.saturating_add(anchor_offset), line_start.saturating_add(head_offset.min(line_width)))
        };

        match semantics{
            CursorSemantics::Bar => {Ok(self.push(Selection::new(start, end), false))}
            CursorSemantics::Block => {Ok(self.push(Selection::new(start, end.saturating_add(1)), false))}
        }
    }

    // should these be made purely functional?
    pub fn shift_subsequent_selections_forward(&mut self, current_selection_index: usize, amount: usize){
        for subsequent_selection_index in current_selection_index.saturating_add(1)..self.count(){
            let subsequent_selection = self.nth_mut(subsequent_selection_index);
            *subsequent_selection = Selection::new(subsequent_selection.anchor().saturating_add(amount), subsequent_selection.head().saturating_add(amount));
        }
    }
    pub fn shift_subsequent_selections_backward(&mut self, current_selection_index: usize, amount: usize){
        for subsequent_selection_index in current_selection_index.saturating_add(1)..self.count(){
            let subsequent_selection = self.nth_mut(subsequent_selection_index);
            *subsequent_selection = Selection::new(subsequent_selection.anchor().saturating_sub(amount), subsequent_selection.head().saturating_sub(amount));
        }
    }
}
