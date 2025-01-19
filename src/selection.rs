// follow documentation style from https://std-dev-guide.rust-lang.org/development/how-to-write-documentation.html
use ropey::Rope;
use crate::{
    text_util, view::View, Position, selection2d::Selection2d
};



//TODO: extension fns should not extend to 1 past doc end, because there are no selectable graphemes there.
// this is ok for movement fns, because the cursor needs to be able to move there to insert new graphemes.



#[derive(Clone, Copy, Debug, PartialEq)]
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
    DirectionMismatch
}
/// 1 dimensional representation of a single selection(between anchor and head) within a text rope.
/// a cursor is a selection with an anchor/head difference of 0 or 1(depending on cursor semantics)
/// Should ensure head/anchor are always within text bounds
#[derive(PartialEq, Clone, Debug)]
pub struct Selection{   //should anchor and head be pulled out into their own structure? struct Range{anchor: usize, head: usize} or maybe Range{start: usize, end: usize}
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
    pub fn new(anchor: usize, head: usize) -> Self{ // could init with cursor semantics: (anchor: usize, cursor: usize, semantics: CursorSemantics)
        Self{anchor, head, stored_line_position: None}
    }

    /// Returns a string for debugging selections over a text.
    /// key:
    ///     start = [
    ///     end = ]
    ///     anchor = |
    ///     head = < or >, depending on selection direction
    ///     cursor(left hand side) = :, if block cursor semantics
    pub fn debug(&self, text: &Rope, semantics: CursorSemantics) -> String{
        //TODO: return an actual error, instead of a magic string. although, it is kind of nice to not have to .unwrap() for every call...
        if semantics == CursorSemantics::Block && self.head == self.anchor{return "Selection head and anchor should not be equal using Block semantics.".to_string()}
        let mut debug_string = String::new();
        for index in 0..=text.len_chars().saturating_add(1){ //needed to add 1 to allow debug chars after text len to be pushed
            if index == self.start(){debug_string.push('[');}
            if index == self.anchor(){debug_string.push('|');}
            if index == self.cursor(text, semantics) && semantics == CursorSemantics::Block{debug_string.push(':');}
            if index == self.head() && self.direction(text, semantics) == Direction::Forward{debug_string.push('>');}
            if index == self.head() && self.direction(text, semantics) == Direction::Backward{debug_string.push('<');}
            if index == self.end(){debug_string.push(']');}
            if let Some(char) = text.get_char(index){
                if char == '\n'{debug_string.push('\n');}
                else if char == '\t'{debug_string.push('\t');}
                else{debug_string.push(char);}
            }
        }

        debug_string
    }
    
    /// Returns the char index of [`Selection`] anchor.
    pub fn anchor(&self) -> usize{self.anchor}
    
    /// Returns the char index of [`Selection`] head.
    pub fn head(&self) -> usize{self.head}

    /// Returns the char index of the start of the [`Selection`] from left to right.
    pub fn start(&self) -> usize{std::cmp::min(self.anchor, self.head)}
    
    /// Returns the char index of the end of the [`Selection`] from left to right.
    pub fn end(&self) -> usize{std::cmp::max(self.anchor, self.head)}

    /// Returns `true` if [`Selection`] len > 0 with bar cursor semantics, or 
    /// [`Selection`] len > 1 with block cursor semantics, or else returns `false`.
    pub fn is_extended(&self, semantics: CursorSemantics) -> bool{
        match semantics{
            CursorSemantics::Bar => self.end().saturating_sub(self.start()) > 0,
            CursorSemantics::Block => self.end().saturating_sub(self.start()) > 1  //if selection is greater than one grapheme //currently uses char count though...
        }

        //i think something like below code will be needed for UTF-8 support, because a single grapheme can be comprised of multiple chars
        //match semantics{  //this seems to cause shitloads of problems with existing code for some reason...
        //    CursorSemantics::Bar => self.start() != self.end(),
        //    CursorSemantics::Block => text_util::next_grapheme_index(self.start(), text) != self.end()
        //}
    }

    /// Returns a bool indicating whether the selection spans multiple lines.
    pub fn spans_multiple_lines(&self, text: &Rope, semantics: CursorSemantics) -> bool{
        // ensure the selection does not exceed the length of the text
        if self.end() > text.len_chars(){return false;}

        let start_line = text.char_to_line(self.start());
        let end_line = text.char_to_line(self.end());

        // if selection is not extended or is extended on the same line
        if !self.is_extended(semantics) || start_line == end_line{return false;}
        // if selection extends to a newline char, but doesn't span multiple lines
        if end_line.saturating_sub(start_line) == 1 && text.line_to_char(end_line) == self.end(){return false;}

        // all other cases
        true
    }

    /// Returns the direction of [`Selection`].
    pub fn direction(&self, text: &Rope, semantics: CursorSemantics) -> Direction{
        assert!(self.cursor(text, semantics) <= text.len_chars());  //we would need a & to text
        assert!(self.anchor <= text.len_chars());
        if self.cursor(text, semantics) < self.anchor{Direction::Backward}
        else{Direction::Forward}
    }

    ///// Sets [`Selection`] direction to specified direction.
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
    pub fn overlaps(&self, other: &Selection) -> bool{
        self.start() == other.start() || 
        self.end() == other.end() || 
        (self.end() > other.start() && other.end() > self.start())
    }

    // Returns a bool indicating whether the provided char index is contained within the [`Selection`].
    pub fn contains(&self, idx: usize) -> bool{idx >= self.start() && idx <= self.end()}

    /// Returns a new [`Selection`] from the overlap of `self` and `other`.
    /// Returns Error if `self` and `other` are non-overlapping.
    pub fn intersection(&self, other: &Selection) -> Result<Self, SelectionError>{
        if self.overlaps(other){
            Ok(Selection::new(self.start().max(other.start()), self.end().min(other.end())))
            // Selection{anchor: self.start().max(other.start()), head: self.end().min(other.end()), stored_line_position: text_util::offset_from_line_start(head, text)}   //if we want stored line position too
        }else{Err(SelectionError::NoOverlap)}
    }

    // TODO: deprecate merge, in favor of application specific merges, so we can calculate appropriate stored_line_positions for resultant selection
    // or maybe just have this function return a selection with no stored_line_position, and use application specific merges for handling stored_line_position...
    /// Create a new [`Selection`] by merging self with other.
    /// Indiscriminate merge. merges whether overlapping, consecutive, 
    /// contained, or disconnected entirely.
    /// resultant selection should be guaranteed to be within text bounds 
    /// because this uses previously initialized selections.
        // note: merges need to have a stored line position, so that movements after merge work correctly
        //TODO: maybe error if Direction of self and other are mismatched...    
            //though i can't think of how this case would occur with normal text editing...   
            //nevermind. this can happen when non extended selection added above, and extended left
            // maybe it is better to have the merge succeed despite direction, then figure out a reasonable stored line position instead?...
    pub fn merge(&self, other: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{   //resultant Selection should be the Direction of self
        //let anchor = self.start().min(other.start());
        //let head = self.end().max(other.end());
        //let stored_line_position = text_util::offset_from_line_start(head, text);   //self.cursor instead of head?    //if neither extended, self.cursor. then base on selection direction?
        //
        //Selection{anchor, head, stored_line_position: Some(stored_line_position)}

        //cannot merge selections with differing directions
        if self.direction(text, semantics) != other.direction(text, semantics){ //TODO: i think this is why some multicursor merges are failing...
            return Err(SelectionError::DirectionMismatch);
        }
        match self.direction(text, semantics){
            Direction::Forward => {
                let anchor = self.start().min(other.start());
                let head = self.end().max(other.end());
                let cursor = match semantics{
                    CursorSemantics::Bar => {head}
                    CursorSemantics::Block => {text_util::previous_grapheme_index(head, text)}
                };
                let stored_line_position = text_util::offset_from_line_start(cursor, text);

                Ok(Selection{anchor, head, stored_line_position: Some(stored_line_position)})
            }
            Direction::Backward => {
                let anchor = self.end().max(other.end());
                let head = self.start().min(other.start());
                let stored_line_position = text_util::offset_from_line_start(head, text);

                Ok(Selection{anchor, head, stored_line_position: Some(stored_line_position)})
            }
        }
    }

    /// Returns a new [`Selection`] from the overlapping range between `self` and `other`, calculating a reasonable `stored_line_position`.
    pub fn merge_overlapping(&self, other: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
        if self.overlaps(other){
            // perform indiscriminate merge to get selection range
            let start = self.start().min(other.start());
            let end = self.end().max(other.end());
            // set resultant direction, based on inputs
            let mut selection = match (self.direction(text, semantics), other.direction(text, semantics), self.is_extended(semantics), other.is_extended(semantics)){
                // if using range from alt_edit_core, this would just set selection.direction...
                (Direction::Forward, Direction::Forward, false, false) => Selection::new(start, end),   //Forward
                (Direction::Forward, Direction::Forward, true, false) => Selection::new(start, end),    //Forward
                (Direction::Forward, Direction::Forward, false, true) => Selection::new(start, end),    //Forward
                (Direction::Forward, Direction::Forward, true, true) => Selection::new(start, end),     //Forward

                (Direction::Forward, Direction::Backward, false, false) => Selection::new(start, end),  //Forward
                (Direction::Forward, Direction::Backward, true, false) => Selection::new(start, end),   //Forward
                (Direction::Forward, Direction::Backward, false, true) => Selection::new(end, start),   //Backward
                (Direction::Forward, Direction::Backward, true, true) => Selection::new(start, end),    //Forward

                (Direction::Backward, Direction::Forward, false, false) => Selection::new(start, end),  //Forward
                (Direction::Backward, Direction::Forward, true, false) => Selection::new(end, start),   //Backward
                (Direction::Backward, Direction::Forward, false, true) => Selection::new(start, end),   //Forward
                (Direction::Backward, Direction::Forward, true, true) => Selection::new(start, end),    //Forward

                (Direction::Backward, Direction::Backward, false, false) => Selection::new(end, start), //Backward
                (Direction::Backward, Direction::Backward, true, false) => Selection::new(end, start),  //Backward
                (Direction::Backward, Direction::Backward, false, true) => Selection::new(end, start),  //Backward
                (Direction::Backward, Direction::Backward, true, true) => Selection::new(end, start),   //Backward
            };
            // calculate new stored_line_position
            //selection.stored_line_position = Some(text_util::offset_from_line_start(self.cursor(text, semantics), text));
            selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(text, semantics), text));
            // return merged selection
            Ok(selection)
        }else{return Err(SelectionError::NoOverlap)}
    }
    
    /// Returns the char index of [`Selection`] cursor.
    /// left side of cursor if block cursor semantics
    /// For example:
    ///     In the string "idk\nsome\nshit\n", at char index 5
    ///         bar(using "|" symbol):          i d k \n s|o m e \n s h i t \n
    ///         block(using "[ and ]" symbols): i d k \n s[o]m e \n s h i t \n
    pub fn cursor(&self, text: &Rope, semantics: CursorSemantics) -> usize{
        match semantics{
            CursorSemantics::Bar => self.head,
            CursorSemantics::Block => {
                if self.head >= self.anchor{text_util::previous_grapheme_index(self.head, text)}
                else{self.head}
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor at specified char index in rope.
    /// Will shift `anchor`/`head` positions to accommodate Bar/Block cursor semantics.
    /// Errors if `to`  > `text.len_chars()`.
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
                //selection.head = to.saturating_add(1).min(text.len_chars().saturating_add(1));   //allowing one more char past text.len_chars() for block cursor
                selection.head = text_util::next_grapheme_index(to, text).min(text.len_chars().saturating_add(1));  //allowing one more char past text.len_chars() for block cursor
            }
            (CursorSemantics::Block, Movement::Extend) => {
                let new_anchor = if self.head >= self.anchor && to < self.anchor{   //if direction forward and to < self.anchor
                    if let Some(char_at_cursor) = text.get_char(self.cursor(text, semantics)){
                        if char_at_cursor == '\n'{
                            self.anchor
                        }else{
                            //self.anchor.saturating_add(1).min(text.len_chars())
                            text_util::next_grapheme_index(self.anchor, text).min(text.len_chars())
                        }
                    }else{
                        //self.anchor.saturating_add(1).min(text.len_chars())
                        text_util::next_grapheme_index(self.anchor, text).min(text.len_chars())
                    }
                }else if self.head < self.anchor && to >= self.anchor{  //if direction backward and to >= self.anchor
                    //self.anchor.saturating_sub(1)
                    text_util::previous_grapheme_index(self.anchor, text)
                }else{  //direction forward and to >= self.anchor || if direction backward and to < self.anchor
                    self.anchor
                };

                if new_anchor <= to{
                    selection.anchor = new_anchor;
                    //selection.head = to.saturating_add(1).min(text.len_chars().saturating_add(1))    //allowing one more char past text.len_chars() for block cursor
                    selection.head = text_util::next_grapheme_index(to, text).min(text.len_chars().saturating_add(1));  //allowing one more char past text.len_chars() for block cursor
                }else{
                    selection.anchor = new_anchor;
                    selection.head = to;
                }
            }
        }
        if update_stored_line_position{
            selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(text, semantics), text));
        }

        assert!(selection.anchor <= text.len_chars());                  //is this needed?
        assert!(selection.cursor(text, semantics) <= text.len_chars());       //is this needed?

        Ok(selection)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved vertically by specified amount.
    /// Errors if `amount` < 1, or calculated new position is invalid.
    pub fn move_vertically(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: error if current_line + amount > text.len_lines, or if current_line < amount when moving backward
        if amount < 1{return Err(SelectionError::InvalidInput);}    // really this should be SelectionError::ResultsInSameState
        
        let mut selection = self.clone();
        
        let current_line = text.char_to_line(self.cursor(text, semantics));
        let goal_line_number = match direction{
            Direction::Forward => (current_line + amount).min(text.len_lines().saturating_sub(1)),
            Direction::Backward => current_line.saturating_sub(amount),
        };

        let start_of_line = text.line_to_char(goal_line_number);
        let line_width = text_util::line_width(text.line(goal_line_number), false);
    
        // Use the stored line position or calculate it if None
        let stored_line_position = self.stored_line_position.unwrap_or_else(|| {
            text_util::offset_from_line_start(self.cursor(text, semantics), text)
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
    // TODO: should this error instead of saturating at text.len_chars?
    pub fn move_horizontally(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if amount < 1{return Err(SelectionError::InvalidInput);}    // really this should be SelectionError::ResultsInSameState
        
        let new_position = match direction{
            //Direction::Forward => self.cursor(semantics).saturating_add(amount).min(text.len_chars()),    //ensures this does not move past text end
            Direction::Forward => {
                let mut index = self.cursor(text, semantics);
                for _ in 0..amount{
                    index = text_util::next_grapheme_index(index, text);
                }
                index.min(text.len_chars()) //ensures this does not move past text end      //could match on semantics, and ensure extend does index.min(previous_grapheme_index(text.len_chars()))
            }
            //Direction::Backward => self.cursor(semantics).saturating_sub(amount)
            Direction::Backward => {
                let mut index = self.cursor(text, semantics);
                for _ in 0..amount{
                    index = text_util::previous_grapheme_index(index, text);
                }
                index
            }
        };
        self.put_cursor(new_position, text, movement, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor set to specified 0-based line number.
    pub fn set_from_line_number(&self, line_number: usize, text: &Rope, movement: Movement, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if line_number >= text.len_lines(){return Err(SelectionError::InvalidInput);}
        
        let current_line = text.char_to_line(self.cursor(text, semantics));
        let (amount, direction) = if line_number < current_line{
            (current_line.saturating_sub(line_number), Direction::Backward)
        }else{
            (line_number.saturating_sub(current_line), Direction::Forward)
        };
        self.move_vertically(amount, text, movement, direction, semantics)
    }

    /// Returns a new instance of [`Selection`] with `anchor` aligned with cursor.
    pub fn collapse(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if !self.is_extended(semantics){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(self.cursor(text, semantics), text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved right.
    pub fn move_right(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Move, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the right.
    pub fn extend_right(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: ensure this can't extend past doc text end
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved right to the nearest word boundary.
    pub fn move_right_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::next_word_boundary(self.head(), text);
        match semantics{
            CursorSemantics::Bar => {
                self.put_cursor(goal_index, text, Movement::Move, semantics, true)
            }
            CursorSemantics::Block => {
                if goal_index == text.len_chars(){
                    self.put_cursor(goal_index, text, Movement::Move, semantics, true)
                }else{
                    self.put_cursor(text_util::previous_grapheme_index(goal_index, text), text, Movement::Move, semantics, true)
                }
            }
        }
    }
    /// Returns a new instance of [`Selection`] with cursor extended right to the nearest word boundary.
    pub fn extend_right_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{  //TODO: ensure this can't extend past doc text end
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::next_word_boundary(self.head(), text);
        match semantics{
            CursorSemantics::Bar => {
                self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
            }
            CursorSemantics::Block => {
                if goal_index == text.len_chars(){
                    self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
                }else{
                    self.put_cursor(text_util::previous_grapheme_index(goal_index, text), text, Movement::Extend, semantics, true)
                }
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor moved left.
    pub fn move_left(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Move, Direction::Backward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the left.
    pub fn extend_left(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved left to the nearest word boundary.
    pub fn move_left_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::previous_word_boundary(self.cursor(text, semantics), text);
        self.put_cursor(goal_index, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with cursor extended left to the nearest word boundary.
    pub fn extend_left_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::previous_word_boundary(self.cursor(text, semantics), text);
        self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved up.
    pub fn move_up(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Move, Direction::Backward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up.
    pub fn extend_up(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved down.
    pub fn move_down(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Move, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down.
    pub fn extend_down(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{ //TODO: ensure this can't extend past doc text end
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved to line end.
    pub fn move_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line = text.line(line_number);
        let line_width = text_util::line_width(line, false);
        let line_start = text.line_to_char(line_number);
        let line_end = line_start.saturating_add(line_width);   //nth_next_grapheme_index(line_start, line_width, text)?

        if self.cursor(text, semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(line_end, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the end of the current line.
    pub fn extend_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: ensure this can't extend past doc text end
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line = text.line(line_number);
        let line_width = text_util::line_width(line, false);    //doesn't include newline
        let line_start = text.line_to_char(line_number);
        let line_end = line_start.saturating_add(line_width);   //index at end of line text, not including newline  //nth_next_grapheme_index(line_start, line_width, text)?

        match semantics{
            CursorSemantics::Bar => {
                if self.cursor(text, semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
                self.put_cursor(line_end, text, Movement::Extend, semantics, true)
            }
            CursorSemantics::Block => {
                //if self.cursor(semantics) == line_end.saturating_sub(1)
                if self.cursor(text, semantics) == text_util::previous_grapheme_index(line_end, text)
                || self.cursor(text, semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
                let start_line = text.char_to_line(self.start());
                let end_line = text.char_to_line(self.end());
                if self.cursor(text, semantics) == self.start() && end_line > start_line{
                    self.put_cursor(line_end, text, Movement::Extend, semantics, true)  //put cursor over newline, if extending from a line below
                }else{
                    //self.put_cursor(line_end.saturating_sub(1), text, Movement::Extend, semantics, true)
                    self.put_cursor(text_util::previous_grapheme_index(line_end, text), text, Movement::Extend, semantics, true)
                }
                
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor moved to absolute start of line, or start of line text, depending on current cursor position.
    pub fn move_home(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

        //if text_start == line_start && self.cursor(semantics) == line_start{return Err(());}    //would result in same state    //TODO: test
        if self.cursor(text, semantics) == text_start{self.move_line_start(text, semantics)}
        else{self.move_line_text_start(text, semantics)}
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to absolute start of line, or line text start, depending on [`Selection`] `head` position.
    pub fn extend_home(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

        //if text_start == line_start && self.cursor(semantics) == line_start{return Err(());}    //would result in same state
        if self.cursor(text, semantics) == text_start{self.extend_line_start(text, semantics)}
        else{self.extend_line_text_start(text, semantics)}
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the current line.
    pub fn move_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);

        if self.cursor(text, semantics) == line_start{return Err(SelectionError::ResultsInSameState);}    //TODO: test
        self.put_cursor(line_start, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the current line.
    pub fn extend_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);

        if self.cursor(text, semantics) == line_start{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(line_start, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the text on the current line.
    pub fn move_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

        if self.cursor(text, semantics) == text_start{return Err(SelectionError::ResultsInSameState);}    //TODO: test
        self.put_cursor(text_start, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the text on the current line.
    pub fn extend_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

        if self.cursor(text, semantics) == text_start{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text_start, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved up by the height of `client_view`.
    pub fn move_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Backward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up by the height of `client_view`.
    pub fn extend_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved down by the height of `client_view`.
    pub fn move_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down by the height of `client_view`.
    pub fn extend_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: ensure this can't extend past doc text end
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the document.
    pub fn move_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(0, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc start.
    pub fn extend_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(0, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the end of the document.
    pub fn move_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text.len_chars(), text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc end.
    pub fn extend_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{  //TODO: ensure this can't extend past doc text end
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with [`Selection`] extended to encompass all text.
    pub fn select_all(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{  //TODO: ensure this can't extend past doc text end
        if self.start() == 0 && (self.end() == text.len_chars() || self.end() == text.len_chars().saturating_add(1)){return Err(SelectionError::ResultsInSameState);}
        let selection = self.put_cursor(0, text, Movement::Move, semantics, true)?;
        selection.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    }

    //TODO: make pub fn select_line //should this include newline at end of line? //should this include indentation at start of line? //vscode includes both, as does kakoune
    //TODO: make pub fn select_inside   //for bracket pairs and the like
    //TODO: make pub fn select_until    //extend selection until provided character is selected (should have one for forwards and one for backwards)
    //TODO: make pub fn align_selected_text_vertically //maybe this belongs in document.rs, since it would have to be an edit...
    //TODO: make pub fn rotate_selected_text   //maybe this belongs in document.rs, since it would have to be an edit...

    //TODO: should this be made purely functional?
    //TODO: should this pass up possible errors from move/extend calls?
    pub fn shift_and_extend(&mut self, amount: usize, text: &Rope, semantics: CursorSemantics){ //-> Result<(), SelectionError>{
        for _ in 0..amount{
            if let Ok(new_selection) = self.move_left(text, semantics){
                *self = new_selection;
            }
        }
        if amount > 1{
            //match semantics{
            //    CursorSemantics::Bar => {
            //        for _ in 0..amount{
            //            //*self = self.extend_right(text, semantics);
            //            if let Ok(new_selection) = self.extend_right(text, semantics){
            //                *self = new_selection;
            //            }
            //        }
            //    }
            //    CursorSemantics::Block => {
            //        for _ in 0..amount.saturating_sub(1){
            //            //*self = self.extend_right(text, semantics);
            //            if let Ok(new_selection) = self.extend_right(text, semantics){
            //                *self = new_selection;
            //            }
            //        }
            //    }
            //}
            for _ in match semantics{   //match semantics to determine our iter range
                CursorSemantics::Bar => 0..amount,
                CursorSemantics::Block => 0..amount.saturating_sub(1)
            }{
                if let Ok(new_selection) = self.extend_right(text, semantics){*self = new_selection;}
            }
        }
    }

    /// Translates a [`Selection`] to a [Selection2d].
    pub fn selection_to_selection2d(&self, text: &Rope, semantics: CursorSemantics) -> Selection2d{
        let line_number_head = text.char_to_line(self.cursor(text, semantics));
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
                self.cursor(text, semantics).saturating_sub(head_line_start_idx),
                //column_head,
                line_number_head
            ) 
        )
    }
}
