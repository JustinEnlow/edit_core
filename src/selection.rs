// follow documentation style from https://std-dev-guide.rust-lang.org/development/how-to-write-documentation.html
use ropey::Rope;
use regex::Regex;
use crate::{
    text_util, view::View, Position, selection2d::Selection2d, range::Range
};



//TODO: can any of this functionality be implemented outside of this crate? 
//user could implement new functionality using core primitives, or exclude any undesired functionality
//should still be useable with `Selections` as long as custom impls retain this structure:
    //Fn(&Selection, &Rope, CursorSemantics) -> Result<Selection, SelectionError>   //move_fn(selection, text, semantics)
    //or
    //Fn(&Selection, &Rope, &View, CursorSemantics) -> Result<Selection, SelectionError>    //move_fn(selection, text, view, semantics)



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorSemantics{
    Bar,    //difference between anchor and head is 0
    Block   //difference between anchor and head is 1 grapheme
}
#[derive(PartialEq, Debug, Clone, Copy)]
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
    //InvalidInput,   //as in put_cursor  //to > text.len_chars()
    SpansMultipleLines,
    DirectionMismatch
}
/// 1 dimensional representation of a single selection(between anchor and head) within a text rope.
/// a cursor is a selection with an anchor/head difference of 0 or 1(depending on cursor semantics)
/// Should ensure head/anchor are always within text bounds
#[derive(PartialEq, Clone, Debug)]
pub struct Selection{
    pub range: Range,
    pub direction: Direction,
    //text: &Rope,                  //This may be useful to store instead of passing in to each fn. What implications would storing this here have?...
    //semantics: CursorSemantics,   //This may be useful to store instead of passing in to each fn. How would changing this on the fly be handled?...
    /// the offset from the start of the line self.cursor is on
    stored_line_position: Option<usize>,
}
impl Selection{
    /////////////////////////////////////////////////////////// Only for Testing ////////////////////////////////////////////////////////////////////
    /// Returns a new instance of [`Selection`] with a specified `stored_line_position`.
    #[must_use] pub fn with_stored_line_position(range: Range, direction: Direction, stored_line_position: usize) -> Self{
        Self{range, direction, stored_line_position: Some(stored_line_position)}
    }
    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    /// Returns a new instance of [`Selection`].
    // TODO: maybe should panic if block semantics and head == anchor
    // TODO: maybe take CursorSemantics as an argument and store it in Selection, this way we never have to pass it in again...
    // TODO: address TODOs in selection_tests/new.rs
    #[must_use] pub fn new(range: Range, direction: Direction) -> Self{
        //assert!(anchor >= 0);  //should be ensured by `usize` type
        //assert!(anchor <= text.len_chars().saturating_add(1));            //requires a reference to underlying Rope
        //assert!(head >= 0);   //should be ensured by `usize` type
        //assert!(head <= text.len_chars().saturating_add(1));              //requires a reference to underlying Rope
        //if block semantics, assert!(anchor != head);                      //requires instance of CursorSemantics

        Self{range, direction, stored_line_position: None}
    }

    fn assert_invariants(&self, text: &Rope, semantics: CursorSemantics){
        //assert!(self.range.start >= 0);   //already ensured by `usize` type
        //assert!(self.range.end >= 0);   //already ensured by `usize` type
        match semantics{
            CursorSemantics::Bar => {
                assert!(self.range.start <= text.len_chars());
                assert!(self.range.end <= text.len_chars());
                assert!(self.range.start <= self.range.end);
            }
            CursorSemantics::Block => {
                if self.is_extended(semantics){
                    assert!(self.range.start <= text.len_chars());
                    assert!(self.range.end <= text.len_chars());
                }else{
                    assert!(self.range.start <= text.len_chars().saturating_add(1));    //sat_add(1) for if cursor on last empty line, and direction backwards
                    assert!(self.range.end <= text.len_chars().saturating_add(1));
                }
                assert!(self.range.start < self.range.end);
            }
        }
    }

    /// Returns a string for debugging selections over a text.
    /// key:
    ///     start = [
    ///     end = ]
    ///     anchor = |
    ///     head = < or >, depending on selection direction
    ///     cursor(left hand side) = :, if block cursor semantics
    #[must_use] pub fn debug(&self, text: &Rope, semantics: CursorSemantics) -> String{
        //TODO: return an actual error, instead of a magic string. although, it is kind of nice to not have to .unwrap() for every call...
        if semantics == CursorSemantics::Block && self.head() == self.anchor(){return "Selection head and anchor should not be equal using Block semantics.".to_string()}
        let mut debug_string = String::new();
        for index in 0..=text.len_chars().saturating_add(1){ //needed to add 1 to allow debug chars after text len to be pushed
            if index == self.range.start{debug_string.push('[');}
            if index == self.anchor(){debug_string.push('|');}
            if index == self.cursor(text, semantics) && semantics == CursorSemantics::Block{debug_string.push(':');}
            if index == self.head() && self.direction == Direction::Forward{debug_string.push('>');}
            if index == self.head() && self.direction == Direction::Backward{debug_string.push('<');}
            if index == self.range.end{debug_string.push(']');}
            if let Some(char) = text.get_char(index){
                if char == '\n'{debug_string.push('\n');}
                else if char == '\t'{debug_string.push('\t');}
                else{debug_string.push(char);}
            }
        }

        debug_string
    }

    // TODO: make fn to_rope_slice
    // TODO: make fn to_string
    
    /// Returns the char index of [`Selection`] anchor. Anchor is the stationary portion of an extended [`Selection`].
    #[must_use] pub fn anchor(&self) -> usize{
        match self.direction{
            Direction::Forward => self.range.start,
            Direction::Backward => self.range.end
        }
    }
    
    /// Returns the char index of [`Selection`] head. Head is the mobile portion of an extended [`Selection`].
    #[must_use] pub fn head(&self) -> usize{
        match self.direction{
            Direction::Forward => self.range.end,
            Direction::Backward => self.range.start
        }
    }

    /// Returns the char index of the start of the [`Selection`] from left to right.
    // note: not tested in selection_tests, and i don't think it should be because all relevant tests are done in range_tests module
    #[must_use] pub fn start(&self) -> usize{self.range.start}      //only needed for Selections::sort. figure out how to make that work without this...

    /// Returns `true` if [`Selection`] len > 0 with bar cursor semantics, or 
    /// [`Selection`] len > 1 with block cursor semantics, or else returns `false`.
    #[must_use] pub fn is_extended(&self, semantics: CursorSemantics) -> bool{
        match semantics{
            CursorSemantics::Bar => self.range.end.saturating_sub(self.range.start) > 0,
            CursorSemantics::Block => self.range.end.saturating_sub(self.range.start) > 1  //if selection is greater than one grapheme //currently uses char count though...
        }

        //i think something like below code will be needed for UTF-8 support, because a single grapheme can be comprised of multiple chars
        //match semantics{  //this seems to cause shitloads of problems with existing code for some reason...
        //    CursorSemantics::Bar => self.start() != self.end(),
        //    CursorSemantics::Block => text_util::next_grapheme_index(self.start(), text) != self.end()
        //}
    }

    /// Returns a bool indicating whether the selection spans multiple lines.
    #[must_use] pub fn spans_multiple_lines(&self, text: &Rope, semantics: CursorSemantics) -> bool{
        // ensure the selection does not exceed the length of the text
        if self.range.end > text.len_chars(){return false;}

        let start_line = text.char_to_line(self.range.start);
        let end_line = text.char_to_line(self.range.end);

        // if selection is not extended or is extended on the same line
        if !self.is_extended(semantics) || start_line == end_line{return false;}
        // if selection extends to a newline char, but doesn't span multiple lines
        if end_line.saturating_sub(start_line) == 1 && text.line_to_char(end_line) == self.range.end{return false;}

        // all other cases
        true
    }
    
    /// Returns a new [`Selection`] from the overlapping [`Range`]s of `self` and `other`, with a reasonable `stored_line_position` calculated.
    pub fn merge_overlapping(&self, other: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
        //assert!(self.semantics == other.semantics)    //for future consideration...
        //assert!(self.text == other.text)  //for future consideration...
        if self.range.overlaps(&other.range){
            // perform indiscriminate merge to get selection range
            let new_range = self.range.merge(&other.range);
            //let mut selection = Selection::new(new_range.start, new_range.end);
            let mut selection = Selection::new(Range::new(new_range.start, new_range.end), Direction::Forward); //maybe move match here instead of assigning Forward
            
            // set resultant direction, based on inputs
            match (self.direction, other.direction, self.is_extended(semantics), other.is_extended(semantics)){
                (Direction::Forward, Direction::Forward, false, false) => selection.direction = Direction::Forward,
                (Direction::Forward, Direction::Forward, true, false) => selection.direction = Direction::Forward,
                (Direction::Forward, Direction::Forward, false, true) => selection.direction = Direction::Forward,
                (Direction::Forward, Direction::Forward, true, true) => selection.direction = Direction::Forward,

                (Direction::Forward, Direction::Backward, false, false) => selection.direction = Direction::Forward,
                (Direction::Forward, Direction::Backward, true, false) => selection.direction = Direction::Forward,
                (Direction::Forward, Direction::Backward, false, true) => selection.direction = Direction::Backward,
                (Direction::Forward, Direction::Backward, true, true) => selection.direction = Direction::Forward,

                (Direction::Backward, Direction::Forward, false, false) => selection.direction = Direction::Forward,
                (Direction::Backward, Direction::Forward, true, false) => selection.direction = Direction::Backward,
                (Direction::Backward, Direction::Forward, false, true) => selection.direction = Direction::Forward,
                (Direction::Backward, Direction::Forward, true, true) => selection.direction = Direction::Forward,

                (Direction::Backward, Direction::Backward, false, false) => selection.direction = Direction::Backward,
                (Direction::Backward, Direction::Backward, true, false) => selection.direction = Direction::Backward,
                (Direction::Backward, Direction::Backward, false, true) => selection.direction = Direction::Backward,
                (Direction::Backward, Direction::Backward, true, true) => selection.direction = Direction::Backward,
            }
            
            // calculate new stored_line_position
            selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(text, semantics), text));
            
            // return merged selection
            Ok(selection)
        }else{Err(SelectionError::NoOverlap)}
    }
    
    /// Returns the char index of [`Selection`] cursor.
    /// left side of cursor if block cursor semantics
    /// For example:
    ///     In the string "idk\nsome\nshit\n", at char index 5
    ///         bar(using "|" symbol):          i d k \n s|o m e \n s h i t \n
    ///         block(using "[ and ]" symbols): i d k \n s[o]m e \n s h i t \n
    #[must_use] pub fn cursor(&self, text: &Rope, semantics: CursorSemantics) -> usize{
        match semantics{
            CursorSemantics::Bar => self.head(),
            CursorSemantics::Block => {
                match self.direction{
                    Direction::Forward => text_util::previous_grapheme_index(self.head(), text),
                    Direction::Backward => self.head()
                }
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor at specified char index in rope.
    /// Will shift `anchor`/`head` positions to accommodate Bar/Block cursor semantics.
    /// If movement == `Movement::Move`, returned selection will always be `Direction::Forward`.
    /// `to` saturates at doc or text boundaries.
    //TODO: even if we saturate `to` at boundaries, we should assert it here, to ensure all calling functions are handling this correctly, and catching errors as early as possible
    pub fn put_cursor(&self, to: usize, text: &Rope, movement: Movement, semantics: CursorSemantics, update_stored_line_position: bool) -> Result<Self, SelectionError>{
        let mut selection = match (semantics, movement){
            (CursorSemantics::Bar, Movement::Move) => {
                //assert!(to <= text.len_chars());
                let to = to.min(text.len_chars());
                //if self.range.start == to && self.range.end == to{return Err(SelectionError::ResultsInSameState);}
                Selection::new(Range::new(to, to), Direction::Forward)
            }
            (CursorSemantics::Bar, Movement::Extend) => {
                //assert!(to <= text.len_chars());
                let to = to.min(text.len_chars());
                //if self.range.start == self.anchor() && self.range.end == to{return Err(SelectionError::ResultsInSameState);}
                Selection::new(Range::new(self.anchor(), to), if to < self.anchor(){Direction::Backward}else{Direction::Forward})
            }
            (CursorSemantics::Block, Movement::Move) => {
                //assert!(to <= text.len_chars());
                let to = to.min(text.len_chars());
                //if self.range.start == to && self.range.end == text_util::next_grapheme_index(to, text).min(text.len_chars().saturating_add(1)){return Err(SelectionError::ResultsInSameState);}
                Selection::new(Range::new(to, text_util::next_grapheme_index(to, text).min(text.len_chars().saturating_add(1))), Direction::Forward)
            }
            (CursorSemantics::Block, Movement::Extend) => {
                //assert!(to <= text_util::previous_grapheme_index(text.len_chars(), text));
                let to = to.min(text_util::previous_grapheme_index(text.len_chars(), text));
                let new_anchor = match self.direction{
                    Direction::Forward => {
                        if to < self.anchor(){  //could also do self.range.start
                            if let Some(char_at_cursor) = text.get_char(self.cursor(text, semantics)){
                                if char_at_cursor == '\n'{self.anchor()}
                                else{text_util::next_grapheme_index(self.anchor(), text).min(text.len_chars())}
                            }else{text_util::next_grapheme_index(self.anchor(), text).min(text.len_chars())}
                        }else{self.anchor()}
                    }
                    Direction::Backward => {
                        if to >= self.anchor(){text_util::previous_grapheme_index(self.anchor(), text)} //could also do self.range.end
                        else{self.anchor()}
                    }
                };

                if new_anchor <= to{    //allowing one more char past text.len_chars() for block cursor
                    //if self.range.start == new_anchor && self.range.end == text_util::next_grapheme_index(to, text).min(text.len_chars().saturating_add(1)){return Err(SelectionError::ResultsInSameState);}
                    Selection::new(Range::new(new_anchor, text_util::next_grapheme_index(to, text).min(text.len_chars().saturating_add(1))), Direction::Forward)
                }else{
                    //if self.range.start == to && self.range.end == new_anchor{return Err(SelectionError::ResultsInSameState);}
                    Selection::new(Range::new(to, new_anchor), Direction::Backward)
                }
            }
        };

        selection.stored_line_position = if update_stored_line_position{    //TODO: this really ought to be handled by calling fn...
            Some(text_util::offset_from_line_start(selection.cursor(text, semantics), text))
        }else{
            self.stored_line_position
        };

        selection.assert_invariants(text, semantics);
        Ok(selection)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved vertically by specified amount.
    /// Errors if `amount` < 1.
    pub fn move_vertically(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: error if current_line + amount > text.len_lines, or if current_line < amount when moving backward
        if amount < 1{return Err(SelectionError::ResultsInSameState);}  //and this may make sense to be an assert. we want the calling function to ensure any input is valid...
        
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
    /// Errors if `amount` < 1.
    pub fn move_horizontally(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        if amount < 1{return Err(SelectionError::ResultsInSameState);}     //and this may make sense to be an assert. we want the calling function to ensure any input is valid...
        
        let new_position = match direction{
            Direction::Forward => {
                let mut index = self.cursor(text, semantics);
                for _ in 0..amount{
                    index = text_util::next_grapheme_index(index, text);
                }
                index.min(text.len_chars()) //ensures this does not move past text end      //could match on semantics, and ensure extend does index.min(previous_grapheme_index(text.len_chars()))
            }
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
        self.assert_invariants(text, semantics);
        assert!(line_number < text.len_lines());

        if line_number == text.char_to_line(self.cursor(text, semantics)){return Err(SelectionError::ResultsInSameState);}
        
        let current_line = text.char_to_line(self.cursor(text, semantics));
        let (amount, direction) = if line_number < current_line{
            (current_line.saturating_sub(line_number), Direction::Backward)
        }else{
            (line_number.saturating_sub(current_line), Direction::Forward)
        };
        self.move_vertically(amount, text, movement, direction, semantics)
    }

    //TODO: we should allow collapsing to cursor, or collapse to anchor
    /// Returns a new instance of [`Selection`] with `anchor` aligned with cursor.
    pub fn collapse(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if !self.is_extended(semantics){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(self.cursor(text, semantics), text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved right.
    pub fn move_right(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Move, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the right.
    pub fn extend_right(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: ensure this can't extend past doc text end
        self.assert_invariants(text, semantics);
        if self.range.start == text.len_chars()
        || self.range.end == text.len_chars()
        || self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}

        self.move_horizontally(1, text, Movement::Extend, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with cursor moved right to the nearest word boundary.
    pub fn move_right_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
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
        self.assert_invariants(text, semantics);
        if self.range.start == text.len_chars()
        || self.range.end == text.len_chars()
        || self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::next_word_boundary(self.head(), text);
        match semantics{
            CursorSemantics::Bar => {
                self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
            }
            CursorSemantics::Block => {
                if goal_index == text.len_chars(){
                    //self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
                    self.put_cursor(text_util::previous_grapheme_index(text.len_chars(), text), text, Movement::Extend, semantics, true)
                }else{
                    self.put_cursor(
                        text_util::previous_grapheme_index(goal_index, text), 
                        text, 
                        Movement::Extend, 
                        semantics, 
                        true
                    )
                }
            }
        }
    }

    //TODO: maybe all movement functions need to check for extension for same state check. like move left

    /// Returns a new instance of [`Selection`] with cursor moved left.
    pub fn move_left(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if !self.is_extended(semantics) && self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Move, Direction::Backward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the left.
    pub fn extend_left(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_horizontally(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved left to the nearest word boundary.
    pub fn move_left_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::previous_word_boundary(self.cursor(text, semantics), text);
        self.put_cursor(goal_index, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with cursor extended left to the nearest word boundary.
    pub fn extend_left_word_boundary(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        
        let goal_index = text_util::previous_word_boundary(self.cursor(text, semantics), text);
        self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved up.
    pub fn move_up(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Move, Direction::Backward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up.
    pub fn extend_up(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved down.
    pub fn move_down(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(1, text, Movement::Move, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down.
    pub fn extend_down(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{ //TODO: ensure this can't extend past doc text end
        self.assert_invariants(text, semantics);
        //if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        let last_line = text.len_lines().saturating_sub(1);
        if text.char_to_line(self.range.start) == last_line
        || text.char_to_line(self.range.end) == last_line
        || text.char_to_line(self.cursor(text, semantics)) == last_line{return Err(SelectionError::ResultsInSameState);}
        
        self.move_vertically(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved to line end.
    pub fn move_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
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
        self.assert_invariants(text, semantics);
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
                let start_line = text.char_to_line(self.range.start);
                let end_line = text.char_to_line(self.range.end);
                if self.cursor(text, semantics) == self.range.start && end_line > start_line{
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
        self.assert_invariants(text, semantics);
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
        self.assert_invariants(text, semantics);
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
        self.assert_invariants(text, semantics);
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);

        if self.cursor(text, semantics) == line_start && !self.is_extended(semantics){return Err(SelectionError::ResultsInSameState);}    //TODO: test
        self.put_cursor(line_start, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the current line.
    pub fn extend_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);

        if self.cursor(text, semantics) == line_start{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(line_start, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the text on the current line.
    pub fn move_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

        if self.cursor(text, semantics) == text_start && !self.is_extended(semantics){return Err(SelectionError::ResultsInSameState);}    //TODO: test
        self.put_cursor(text_start, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the text on the current line.
    pub fn extend_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        let line_number = text.char_to_line(self.cursor(text, semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

        if self.cursor(text, semantics) == text_start{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text_start, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved up by the height of `client_view`.
    pub fn move_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Backward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up by the height of `client_view`.
    pub fn extend_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if text.char_to_line(self.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved down by the height of `client_view`.
    pub fn move_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Forward, semantics)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down by the height of `client_view`.
    pub fn extend_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Result<Self, SelectionError>{    //TODO: ensure this can't extend past doc text end
        self.assert_invariants(text, semantics);
        //if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
        let last_line = text.len_lines().saturating_sub(1);    //do we need to satsub 2, so that we are checking last viable extend line, not last empty line?...
        if text.char_to_line(self.range.start) == last_line
        || text.char_to_line(self.range.end) == last_line
        || text.char_to_line(self.cursor(text, semantics)) == last_line{return Err(SelectionError::ResultsInSameState);}

        //let last_line = text.len_lines().saturating_sub(1);
        let current_line = text.char_to_line(self.cursor(text, semantics));
        
        //ensure amount passed to move_vertically is always valid input
        let amount = client_view.height().saturating_sub(1);
        let max_amount = last_line.saturating_sub(current_line);
        let saturated_amount = amount.min(max_amount);
        if saturated_amount == 0{Err(SelectionError::ResultsInSameState)}
        else{self.move_vertically(saturated_amount, text, Movement::Extend, Direction::Forward, semantics)}
        //self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the document.
    pub fn move_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(0, text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc start.
    pub fn extend_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(0, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the end of the document.
    pub fn move_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        if self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        self.put_cursor(text.len_chars(), text, Movement::Move, semantics, true)
    }
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc end.
    pub fn extend_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{  //TODO: ensure this can't extend past doc text end
        self.assert_invariants(text, semantics);
        if self.range.start == text.len_chars()
        || self.range.end == text.len_chars()
        || self.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        
        self.put_cursor(
            match semantics{
                CursorSemantics::Bar => text.len_chars(), 
                CursorSemantics::Block => text_util::previous_grapheme_index(text.len_chars(), text)
            }, 
            text, 
            Movement::Extend, 
            semantics, 
            true
        )

        // or if we end up getting rid of put cursor...
        //let mut selection = Selection::new(self.anchor(), text.len_chars());
        //selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(text, semantics), text));
        //Ok(selection)
    }
    
    /// Returns a new instance of [`Selection`] encompassing the current line.
    //TODO: make pub fn select_line //should this include newline at end of line? //should this include indentation at start of line? //vscode includes both, as does kakoune
    //TODO: if called on empty last line, this moves the selection to second to last line end, instead it should error
    pub fn select_line(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{
        self.assert_invariants(text, semantics);
        //vs code selects all spanned lines...  maybe caller can make that determination...
        if self.spans_multiple_lines(text, semantics){return Err(SelectionError::SpansMultipleLines);}    //make specific error. SpansMultipleLines or something...
        if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}

        let line = text.char_to_line(self.range.start);
        let line_start = text.line_to_char(line);
        let line_end = line_start + text_util::line_width(text.line(line), true);

        if self.range.start == line_start && self.range.end == line_end{Err(SelectionError::ResultsInSameState)}
        else{
            //Ok(Selection::new(line_start, line_end))
            Ok(Selection::new(Range::new(line_start, line_end), Direction::Forward))
        }
    }

    /// Returns a new instance of [`Selection`] with [`Selection`] extended to encompass all text.
    pub fn select_all(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionError>{  //TODO: ensure this can't extend past doc text end
        self.assert_invariants(text, semantics);
        if self.range.start == 0 && (self.range.end == text.len_chars() || self.range.end == text.len_chars().saturating_add(1)){return Err(SelectionError::ResultsInSameState);}
        let selection = self.put_cursor(0, text, Movement::Move, semantics, true)?;
        //selection.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
        selection.put_cursor(match semantics{CursorSemantics::Bar => text.len_chars(), CursorSemantics::Block => text_util::previous_grapheme_index(text.len_chars(), text)}, text, Movement::Extend, semantics, true)
    }

    //TODO: make smart_select_grow  //grows selection to ecompass next largest text object(word -> long_word -> long_word+surrounding punctuation or whitespace -> inside brackets -> sentence -> line -> paragraph -> all)
    //TODO: make smart_select_shrink    //opposite of above

    //TODO: impl and test
    //TODO: future improvement: for each char search loop, spawn a thread to do the search, so we can process them simultaneously.
    //TODO: error if searching backwards and reach previous selection range end, or if searching forward and reach next selection range start   //maybe this logic needs to be in selections
        //should operate over a rope slice from (start of doc if no previous selection, or previous selection end) to (end of doc text if no next selection, or next selection start)
    /// Returns a new [`Selection`] inside but excluding specified input char.
    pub fn select_inside_instances_of_single_char(&self, input: char, text: &Rope) -> Result<Self, SelectionError>{     //TODO: this is really more of a "search around selection for instances of single char"
        let mut new_selection = self.clone();
        
        //second version
        let mut found_backward = false;
        //for (i, current_char) in text.slice(0..self.range.start).to_string().chars().rev().enumerate(){ //can this be done without converting to string?...
        for (i, &current_char) in text.slice(0..self.range.start).chars().collect::<Vec<_>>().iter().rev().enumerate(){
            if current_char == input{
                new_selection.range.start = new_selection.range.start.saturating_sub(i);// - (i+1);
                found_backward = true;
                break;
            }
        }
        
        let mut found_forward = false;
        for (i, current_char) in text.slice(self.range.end..).chars().enumerate(){
            if current_char == input{
                new_selection.range.end = new_selection.range.end.saturating_add(i);// + (i-1);
                found_forward = true;
                break;
            }
        }

        if found_forward && found_backward{
            Ok(new_selection)
        }else{
            Err(SelectionError::ResultsInSameState)
        }
    }
    /// Returns a new [`Selection`] inside but excluding specified char pair.
    pub fn select_inside_pair(&self, leading_char: char, trailing_char: char, text: &Rope) -> Result<Self, SelectionError>{     //TODO: this is really more of a "search around selection for char pair"
        let mut new_selection = self.clone();

        let mut found_backward = false;
        for (i, &current_char) in text.slice(0..self.range.start).chars().collect::<Vec<_>>().iter().rev().enumerate(){
            println!("backward: {} at {}", current_char, i);
            if current_char == leading_char{
                new_selection.range.start = new_selection.range.start.saturating_sub(i);// - (i+1);
                found_backward = true;
                break;
            }
        }
        let mut found_forward = false;
        for (i, current_char) in text.slice(self.range.end..).chars().enumerate(){
            println!("forward: {} at {}", current_char, i);
            if current_char == trailing_char{
                new_selection.range.end = new_selection.range.end.saturating_add(i);// + (i-1);
                found_forward = true;
                break;
            }
        }

        if found_forward && found_backward{
            Ok(new_selection)
        }else{
            Err(SelectionError::ResultsInSameState)
        }
    }
    //fn select_inside_text_object(){}    //for paragraphs, words, and the like
    
    //TODO: make pub fn select_until    //extend selection until provided character/string is selected (should have one for forwards and one for backwards)
    
    /// Returns a [`Vec`] of [`Selection`]s where the underlying text is a match for the `input` search string.
    #[must_use] pub fn search(&self, input: &str, text: &Rope) -> Vec<Selection>{   //text should be the text within a selection, not the whole document text       //TODO: -> Result<Vec<Selection>>
        let mut selections = Vec::new();
        let start = self.range.start;

        //match Regex::new(input){
        //    Ok(regex) => {
        //        for search_match in regex.find_iter(&text.to_string()[start..self.range.end.min(text.len_chars())]){
        //            selections.push(Selection::new(search_match.start().saturating_add(start), search_match.end().saturating_add(start)));
        //        }
        //    }
        //    Err(_) => {}    //return error FailedToParseRegex
        //}
        if let Ok(regex) = Regex::new(input){
            for search_match in regex.find_iter(&text.to_string()[start..self.range.end.min(text.len_chars())]){
                //selections.push(Selection::new(search_match.start().saturating_add(start), search_match.end().saturating_add(start)));
                selections.push(Selection::new(Range::new(search_match.start().saturating_add(start), search_match.end().saturating_add(start)), Direction::Forward))
            }
        }
        //else{/*return error FailedToParseRegex*/}

        if selections.is_empty(){
            //return NoMatch error      //this error is not strictly necessary since caller can just check for an empty return vec
        }
        selections
    }

    /// Returns a [`Vec`] of [`Selection`]s containing each part of the current selection except the split pattern.
    pub fn split(&self, pattern: &str, text: &Rope) -> Vec<Selection>{
        let mut selections = Vec::new();
        if let Ok(regex) = Regex::new(pattern){
            let mut start = self.range.start; //0;
            let mut found_split = false;
            // Iter over each split, and push the retained selection before it, if any...       TODO: test split at start of selection
            for split in regex.find_iter(&text.to_string()[self.range.start..self.range.end.min(text.len_chars())]){
                found_split = true;
                let selection_range = Range::new(start, split.start().saturating_add(self.range.start));
                if selection_range.start < selection_range.end{
                    //selections.push(Selection::new(selection_range.start, selection_range.end));
                    selections.push(Selection::new(Range::new(selection_range.start, selection_range.end), Direction::Forward))
                }
                start = split.end().saturating_add(self.range.start);
            }
            // Handle any remaining text after the last split
            //if split found and end of last split < selection end
            if found_split && start < self.range.end.min(text.len_chars()){
                //selections.push(Selection::new(start, self.range.end.min(text.len_chars())));
                selections.push(Selection::new(Range::new(start, self.range.end.min(text.len_chars())), Direction::Forward));
            }
        }
        selections
    }

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
    #[must_use] pub fn selection_to_selection2d(&self, text: &Rope, semantics: CursorSemantics) -> Selection2d{
        let line_number_head = text.char_to_line(self.cursor(text, semantics));
        let line_number_anchor = text.char_to_line(self.anchor());

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
                self.anchor().saturating_sub(anchor_line_start_idx),
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
