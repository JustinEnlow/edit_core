use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction, SelectionError};
use crate::view::View;
use crate::text_util;



#[derive(Debug, PartialEq)]
pub enum SelectionsError{
    SingleSelection,
    MultipleSelections,
    SpansMultipleLines,
    CannotAddSelectionAbove,
    CannotAddSelectionBelow,
    NoSearchMatches,
    ResultsInSameState
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
    #[must_use] pub fn new(selections: Vec<Selection>, primary_selection_index: usize, _text: &Rope) -> Self{
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
        //selections = selections.merge_overlapping(text);  //TODO: fix this to use new merge_overlapping fn
        //if let Ok(merged_selections) = selections.merge_overlapping(text, semantics){
        //    selections = merged_selections;
        //}

        assert!(selections.count() > 0);
        //TODO: for every selection assert start >= 0 and end <= text.len_chars + 1(for final empty line)
        selections
    }
    /// Returns the number of [`Selection`]s in [`Selections`].
    // note: not tested in selections_tests module
    #[must_use] pub fn count(&self) -> usize{
        self.selections.len()
    }
    // note: not tested in selections_tests module
    #[must_use] pub fn primary_selection_index(&self) -> usize{
        self.primary_selection_index
    }
    // note: not tested in selections_tests module
    pub fn iter(&self) -> std::slice::Iter<'_, Selection>{
        self.selections.iter()
    }
    // note: not tested in selections_tests module
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Selection>{
        self.selections.iter_mut()
    }
    /// Returns a new instance of [`Selections`] with the last element removed.
    #[must_use] pub fn pop(&self) -> Self{
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
    #[must_use] pub fn push_front(&self, selection: Selection, update_primary: bool) -> Self{
        let mut new_selections = self.selections.clone();
        new_selections.insert(0, selection);
        Self{
            selections: new_selections,
            primary_selection_index: if update_primary{0}else{self.primary_selection_index.saturating_add(1)} //0
        }
    }
    
    /// Appends a [`Selection`] to the back of [Self], updating `primary_selection_index` if desired.
    #[must_use] pub fn push(&self, selection: Selection, update_primary: bool) -> Self{
        let mut new_selections = self.selections.clone();
        new_selections.push(selection);
        let primary_selection_index = new_selections.len().saturating_sub(1);
        Self{
            selections: new_selections,
            primary_selection_index: if update_primary{primary_selection_index}else{self.primary_selection_index}
        }
    }
    
    /// Returns a reference to the [`Selection`] at `primary_selection_index`.
    // note: not tested in selections_tests module
    #[must_use] pub fn primary(&self) -> &Selection{
        &self.selections[self.primary_selection_index]
    }
    /// Returns a mutable reference to the [`Selection`] at `primary_selection_index`.
    // note: not tested in selections_tests module
    pub fn primary_mut(&mut self) -> &mut Selection{
        &mut self.selections[self.primary_selection_index]
    }
    // note: not tested in selections_tests module
    #[must_use] pub fn first(&self) -> &Selection{
        // unwrapping because we ensure at least one selection is always present
        self.selections.first().unwrap()
    }
    //pub fn first_mut(&mut self) -> &mut Selection{
    //    self.selections.first_mut().unwrap()
    //}
    // note: not tested in selections_tests module
    #[must_use] pub fn last(&self) -> &Selection{
        // unwrapping because we ensure at least one selection is always present
        self.selections.last().unwrap()
    }
    // note: not tested in selections_tests module
    pub fn nth_mut(&mut self, index: usize) -> &mut Selection{
        self.selections.get_mut(index).unwrap()
    }

    /// Increments `primary_selection_index`.
    pub fn increment_primary_selection(&self) -> Result<Self, SelectionsError>{
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}
        if self.primary_selection_index.saturating_add(1) < self.count(){
            Ok(Self{selections: self.selections.clone(), primary_selection_index: self.primary_selection_index + 1})
        }else{
            Ok(Self{selections: self.selections.clone(), primary_selection_index: 0})
        }
    }
    /// Decrements the primary selection index.
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
    #[must_use] pub fn sort(&self) -> Self{ //TODO: return error instead...
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
    pub fn merge_overlapping(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionsError>{
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}

        let mut primary = self.primary().clone();
        let mut new_selections = self.selections.clone();
        new_selections.dedup_by(|current_selection, prev_selection|{
            //if prev_selection.overlaps(current_selection){
                //let merged_selection = match current_selection.merge(prev_selection, text, semantics){
                //    Ok(val) => val,
                //    Err(_) => {return false;}
                //};
                //let merged_selection = match current_selection.merge_overlapping(prev_selection, text, semantics){
                //    Ok(val) => val,
                //    Err(_) => {return false;}
                //};
                let Ok(merged_selection) = current_selection.merge_overlapping(prev_selection, text, semantics) //change suggested by clippy lint
                else{return false;};

                // Update primary selection to track index in next code block // Only clone if necessary
                if prev_selection == &primary || current_selection == &primary{
                    primary = merged_selection.clone();
                }

                *prev_selection = merged_selection;
                true
            //}else{false}
        });

        let primary_selection_index = new_selections.iter()
            .position(|selection| selection == &primary)
            .unwrap_or(0);

        assert!(self.count() > 0);

        Ok(Self{
            selections: new_selections,
            primary_selection_index,
        })
    }

    /// Removes all [`Selection`]s except [`Selection`] at `primary_selection_index`.
    /// Errors if [`Selections`] has only 1 [`Selection`].
    pub fn clear_non_primary_selections(&self) -> Result<Self, SelectionsError>{
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}
        
        let primary_as_vec = vec![self.primary().clone()];
        assert!(primary_as_vec.len() == 1);
        
        Ok(Self{
            selections: primary_as_vec,
            primary_selection_index: 0
        })
    }

    //TODO: add selection above/below fns don't work as expected when multiple selections on same line. only adds primary selection range above/below

    /// Adds a new [`Selection`] directly above the top-most [`Selection`], with the same start and end offsets from line start, if possible.
    pub fn add_selection_above(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionsError>{
        assert!(self.count() > 0);  //ensure at least one selection in selections

        let top_selection = self.first();
        let top_selection_line = text.char_to_line(top_selection.range.start);
        if top_selection_line == 0{return Err(SelectionsError::CannotAddSelectionAbove);}
        // should error if any selection spans multiple lines. //callee can determine appropriate response behavior in this case        //vscode behavior is to extend topmost selection up one line if any selection spans multiple lines
        for selection in &self.selections{  //self.selections.iter(){   //change suggested by clippy lint
            if selection.spans_multiple_lines(text, semantics){return Err(SelectionsError::SpansMultipleLines);}
        }

        // using primary selection here, because that is the selection we want our added selection to emulate, if possible with the available text
        let start_offset = text_util::offset_from_line_start(self.primary().range.start, text);
        let end_offset = start_offset.saturating_add(self.primary().range.end.saturating_sub(self.primary().range.start));  //start_offset + (end char index - start char index)
        let line_above = top_selection_line.saturating_sub(1);
        let line_start = text.line_to_char(line_above);
        let line_text = text.line(line_above);
        let line_width = text_util::line_width(line_text, false);
        let line_width_including_newline = text_util::line_width(line_text, true);
        let (start, end) = if line_text.to_string().is_empty() || line_text == "\n"{    //should be impossible for the text in the line above first selection to be empty. is_empty() check is redundant here...
            match semantics{
                CursorSemantics::Bar => (line_start, line_start),
                CursorSemantics::Block => (line_start, text_util::next_grapheme_index(line_start, text))
            }
        }
        else if self.primary().is_extended(semantics){
            if start_offset < line_width{   //should we exclusively handle start_offset < line_width && end_offset < line_width as well?
                (line_start.saturating_add(start_offset), line_start.saturating_add(end_offset.min(line_width_including_newline))) //start offset already verified within line text bounds
            }
            else{
                // currently same as non extended. this might change...
                match semantics{    //ensure adding the offsets doesn't make this go past line width
                    CursorSemantics::Bar => (line_start.saturating_add(start_offset.min(line_width)), line_start.saturating_add(start_offset.min(line_width))),
                    CursorSemantics::Block => (line_start.saturating_add(start_offset.min(line_width)), text_util::next_grapheme_index(line_start.saturating_add(start_offset.min(line_width)), text))
                }
            }
        }
        else{  //not extended
            match semantics{    //ensure adding the offsets doesn't make this go past line width
                CursorSemantics::Bar => (line_start.saturating_add(start_offset.min(line_width)), line_start.saturating_add(start_offset.min(line_width))),
                CursorSemantics::Block => (line_start.saturating_add(start_offset.min(line_width)), text_util::next_grapheme_index(line_start.saturating_add(start_offset.min(line_width)), text))
            }
        };

        match self.primary().direction{
            //Direction::Forward => Ok(self.push_front(Selection::new(start, end), false)),
            Direction::Forward => Ok(self.push_front(Selection::new(Range::new(start, end), Direction::Forward), false)),
            //Direction::Backward => Ok(self.push_front(Selection::new(end, start), false))
            Direction::Backward => Ok(self.push_front(Selection::new(Range::new(end, start), Direction::Backward), false))
        }
    }

    // TODO: selection added below at text end is not rendering on last line(this is a frontend issue though)
    /// Adds a new [`Selection`] directly below bottom-most [`Selection`], with the same start and end offsets from line start, if possible.
    pub fn add_selection_below(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionsError>{
        assert!(self.count() > 0);  //ensure at least one selection in selections

        let bottom_selection = self.last();
        let bottom_selection_line = text.char_to_line(bottom_selection.range.start);
        //bottom_selection_line must be zero based, and text.len_lines() one based...   //TODO: verify
        if bottom_selection_line >= text.len_lines().saturating_sub(1){return Err(SelectionsError::CannotAddSelectionBelow);}
        // should error if any selection spans multiple lines. //callee can determine appropriate response behavior in this case        //vscode behavior is to extend topmost selection down one line if any selection spans multiple lines
        for selection in &self.selections{  //self.selections.iter(){   //change suggested by clippy lint
            if selection.spans_multiple_lines(text, semantics){return Err(SelectionsError::SpansMultipleLines);}
        }

        // using primary selection here, because that is the selection we want our added selection to emulate, if possible with the available text
        let start_offset = text_util::offset_from_line_start(self.primary().range.start, text);
        let end_offset = start_offset.saturating_add(self.primary().range.end.saturating_sub(self.primary().range.start));  //start_offset + (end char index - start char index)
        let line_below = bottom_selection_line.saturating_add(1);
        let line_start = text.line_to_char(line_below);
        let line_text = text.line(line_below);
        let line_width = text_util::line_width(line_text, false);
        let line_width_including_newline = text_util::line_width(line_text, true);
        let (start, end) = if line_text.to_string().is_empty() || line_text == "\n"{    //should be impossible for the text in the line above first selection to be empty. is_empty() check is redundant here...
            match semantics{
                CursorSemantics::Bar => (line_start, line_start),
                CursorSemantics::Block => (line_start, text_util::next_grapheme_index(line_start, text))
            }
        }
        else if self.primary().is_extended(semantics){
            if start_offset < line_width{   //should we exclusively handle start_offset < line_width && end_offset < line_width as well?
                (line_start.saturating_add(start_offset), line_start.saturating_add(end_offset.min(line_width_including_newline))) //start offset already verified within line text bounds
            }
            else{
                // currently same as non extended. this might change...
                match semantics{    //ensure adding the offsets doesn't make this go past line width
                    CursorSemantics::Bar => (line_start.saturating_add(start_offset.min(line_width)), line_start.saturating_add(start_offset.min(line_width))),
                    CursorSemantics::Block => (line_start.saturating_add(start_offset.min(line_width)), text_util::next_grapheme_index(line_start.saturating_add(start_offset.min(line_width)), text))
                }
            }
        }
        else{  //not extended
            match semantics{    //ensure adding the offsets doesn't make this go past line width
                CursorSemantics::Bar => (line_start.saturating_add(start_offset.min(line_width)), line_start.saturating_add(start_offset.min(line_width))),
                CursorSemantics::Block => (line_start.saturating_add(start_offset.min(line_width)), text_util::next_grapheme_index(line_start.saturating_add(start_offset.min(line_width)), text))
            }
        };

        match self.primary().direction{
            //Direction::Forward => Ok(self.push(Selection::new(start, end), false)),
            Direction::Forward => Ok(self.push(Selection::new(Range::new(start, end), Direction::Forward), false)),
            //Direction::Backward => Ok(self.push(Selection::new(end, start), false))
            Direction::Backward => Ok(self.push(Selection::new(Range::new(end, start), Direction::Backward), false))
        }
    }

    /// Returns a new instance of [`Selections`] with the current primary selection removed, if possible.
    /// # Errors
    /// errors if `self` containts only a single `Selection`.
    pub fn remove_primary_selection(&self) -> Result<Self, SelectionsError>{
        if self.count() < 2{return Err(SelectionsError::SingleSelection);}
        
        let mut new_selections = Vec::new();
        for selection in &self.selections{
            if selection != self.primary(){
                new_selections.push(selection.clone());
            }
        }
        //keep the new primary selection relatively close by
        let new_primary_index = if self.primary_selection_index > 0{
            self.primary_selection_index.saturating_sub(1)
        }else{
            self.primary_selection_index
        };

        Ok(Self{selections: new_selections, primary_selection_index: new_primary_index})
    }

    // should these be made purely functional?  //for selection in selections{if selection <= current_selection_index{push selection to vec}}
    pub fn shift_subsequent_selections_forward(&mut self, current_selection_index: usize, amount: usize){
        for subsequent_selection_index in current_selection_index.saturating_add(1)..self.count(){
            let subsequent_selection = self.nth_mut(subsequent_selection_index);
            //*subsequent_selection = Selection::new(subsequent_selection.anchor().saturating_add(amount), subsequent_selection.head().saturating_add(amount));
            *subsequent_selection = Selection::new(Range::new(subsequent_selection.anchor().saturating_add(amount), subsequent_selection.head().saturating_add(amount)), Direction::Forward);   //TODO: figure out how to actually determine direction
        }
    }
    pub fn shift_subsequent_selections_backward(&mut self, current_selection_index: usize, amount: usize){
        for subsequent_selection_index in current_selection_index.saturating_add(1)..self.count(){
            let subsequent_selection = self.nth_mut(subsequent_selection_index);
            //*subsequent_selection = Selection::new(subsequent_selection.anchor().saturating_sub(amount), subsequent_selection.head().saturating_sub(amount));
            *subsequent_selection = Selection::new(Range::new(subsequent_selection.anchor().saturating_sub(amount), subsequent_selection.head().saturating_sub(amount)), Direction::Forward);   //TODO: figure out how to actually determine direction
        }
    }

    pub fn surround(&self, text: &Rope) -> Result<Self, SelectionsError>{
        let mut new_selections = Vec::with_capacity(2*self.count());
        let mut num_pushed: usize = 0;
        let primary_selection = self.primary();
        let mut primary_selection_index = self.primary_selection_index;
        for selection in &self.selections{
            let surrounds = selection.surround(text);
            if selection == primary_selection{
                primary_selection_index = num_pushed;//.saturating_sub(1);
            }
            for surround in surrounds{
                new_selections.push(surround);
                num_pushed = num_pushed + 1;
            }
        }
        if new_selections.is_empty(){Err(SelectionsError::ResultsInSameState)} //TODO: create better error?...
        else{
            Ok(Selections::new(new_selections, primary_selection_index, text))
        }
    }

    //TODO: for some reason, repeated calls after successfully selecting bracket pair do not return same state error...
    pub fn nearest_surrounding_pair(&self, text: &Rope, semantics: CursorSemantics) -> Result<Self, SelectionsError>{
        let mut new_selections = Vec::with_capacity(2*self.count());
        let mut num_pushed: usize = 0;
        let primary_selection = self.primary();
        let mut primary_selection_index = self.primary_selection_index;
        for selection in &self.selections{
            let surrounds = selection.nearest_surrounding_pair(text);
            if selection == primary_selection{
                primary_selection_index = num_pushed;
            }
            if surrounds.is_empty(){//push selection
                new_selections.push(selection.clone());
                num_pushed = num_pushed + 1;
            }
            else{//push surrounds
                for surround in surrounds{
                    new_selections.push(surround);
                    num_pushed = num_pushed + 1;
                }
            }
        }
        if new_selections.is_empty() || new_selections == self.selections{Err(SelectionsError::ResultsInSameState)}
        else{
            //Ok(Selections::new(new_selections, primary_selection_index, text))
            Selections::new(new_selections, primary_selection_index, text).merge_overlapping(text, semantics)
        }
    }

    //TODO: maybe. if no selection extended, search whole text
    /// 
    /// # Errors
    ///     - if no matches.
    pub fn search(&self, input: &str, text: &Rope) -> Result<Self, SelectionsError>{
        if input.is_empty(){return Err(SelectionsError::NoSearchMatches);}
        let mut new_selections = Vec::new();
        let mut num_pushed: usize = 0;
        let primary_selection = self.primary();
        //let mut primary_selection_index = self.primary_selection_index;
        let mut primary_selection_index = 0;
        
        for selection in &self.selections{  //self.selections.iter(){   //change suggested by clippy lint
            let matches = selection.search(input, text);
            if selection == primary_selection{
                primary_selection_index = num_pushed.saturating_sub(1);
            }
            for search_match in matches{
                new_selections.push(search_match);
                num_pushed = num_pushed + 1;
            }
        }

        if new_selections.is_empty(){Err(SelectionsError::NoSearchMatches)}
        else{
            Ok(Selections::new(new_selections, primary_selection_index, text))
        }
    }
    //TODO: pub fn search_whole_text

    //TODO: impl tests in src/selections_tests
    pub fn split(&self, input: &str, text: &Rope) -> Result<Self, SelectionsError>{
        if input.is_empty(){return Err(SelectionsError::NoSearchMatches);}
        let mut new_selections = Vec::new();
        let mut num_pushed: usize = 0;
        let primary_selection = self.primary();
        let mut primary_selection_index = 0;
        
        for selection in &self.selections{
            let matches = selection.split(input, text);
            if selection == primary_selection{
                primary_selection_index = num_pushed.saturating_sub(1);
            }
            for search_match in matches{
                new_selections.push(search_match);
                num_pushed = num_pushed + 1;
            }
        }

        if new_selections.is_empty(){Err(SelectionsError::NoSearchMatches)}
        else{
            Ok(Selections::new(new_selections, primary_selection_index, text))
        }
    }

//TODO: impl multiselection movement/extend functions
    /// Intended to ease the use of Selection functions, when used over multiple selections, where the returned selections could be overlapping.
    ///     intended for use with:
    ///         move up
    ///         move down
    ///         move left
    ///         move right
    ///         move backward word boundary
    ///         move forward word boundary
    ///         move line end
    ///         move line start
    ///         move line text start
    ///         move home (switches between line start and line text start)
    ///         extend up
    ///         extend down
    ///         extend left
    ///         extend right
    ///         extend backward word boundary
    ///         extend forward word boundary
    ///         extend line end
    ///         extend line start
    ///         extend line text start
    ///         extend home (switches between line start and line text start)
    ///         extend doc start
    ///         extend doc end
    ///         select line
    pub fn move_cursor_potentially_overlapping<F>(&self, text: &Rope, semantics: CursorSemantics, move_fn: F) -> Result<Self, SelectionsError>
        where F: Fn(&Selection, &Rope, CursorSemantics) -> Result<Selection, SelectionError>
    {
        let mut new_selections = Vec::with_capacity(self.count());  //the maximum size this vec should ever be is num selections in self
        for selection in self.iter(){
            match move_fn(selection, text, semantics){
                Ok(new_selection) => {new_selections.push(new_selection);}
                Err(e) => {
                    match e{
                        SelectionError::ResultsInSameState => {
                            if self.count() == 1{return Err(SelectionsError::ResultsInSameState)}
                            new_selections.push(selection.clone()); //retains selections with no change resulting from move_fn
                        }
                        //TODO: figure out what to do with other errors, if they can even happen...
                        //are we guaranteed by fn impls to never have these errors returned?
                        //what if user passes an unintended move_fn to this one?...
                        SelectionError::DirectionMismatch |
                        SelectionError::SpansMultipleLines |//InvalidInput |
                        SelectionError::NoOverlap => {unreachable!()}   //if this is reached, move_fn called on one of the selections has probably put us in an unintended state. prob best to panic
                    }
                }
            }
        }
        let mut new_selections = Selections::new(new_selections, self.primary_selection_index, text);
        if let Ok(merged_selections) = new_selections.merge_overlapping(text, semantics){
            new_selections = merged_selections;
        }
        if &new_selections == self{return Err(SelectionsError::ResultsInSameState);}    //this should handle multicursor at doc end and another extend all the way right at text and, and no same state error
        Ok(new_selections)
    }
    
    /// Intended to ease the use of Selection functions, when used over multiple selections, where the returned selections should definitely not be overlapping.
    ///     intended for use with:
    ///         collapse selection
    ///         maybe others...i thought there would be more use cases, but that hasn't proven to be the case yet
    pub fn move_cursor_non_overlapping<F>(&self, text: &Rope, semantics: CursorSemantics, move_fn: F) -> Result<Self, SelectionsError>
        where F: Fn(&Selection, &Rope, CursorSemantics) -> Result<Selection, SelectionError>
    {
        let mut new_selections = Vec::with_capacity(self.count());  //the maximum size this vec should ever be is num selections in self
        let mut movement_succeeded = false;
        for selection in self.iter(){
            match move_fn(selection, text, semantics){
                Ok(new_selection) => {
                    new_selections.push(new_selection);
                    movement_succeeded = true;
                }
                Err(e) => {
                    match e{
                        SelectionError::ResultsInSameState => {new_selections.push(selection.clone());} //same state handled later in fn
                        //figure out what to do with other errors, if they can even happen...
                        SelectionError::DirectionMismatch |
                        SelectionError::SpansMultipleLines |//InvalidInput |
                        SelectionError::NoOverlap => {unreachable!()}   //if this is reached, move_fn called on one of the selections has probably put us in an unintended state. prob best to panic
                    }
                }
            }
        }
        if !movement_succeeded{return Err(SelectionsError::ResultsInSameState)}
        let new_selections = Selections::new(new_selections, self.primary_selection_index, text);
        Ok(new_selections)
    }
    
    /// Intended to ease the use of Selection functions, when used over multiple selections, where movement should result in a single selection.
    ///     intended for use with:
    ///         move doc start
    ///         move doc end
    ///         select all
    pub fn move_cursor_clearing_non_primary<F>(&self, text: &Rope, semantics: CursorSemantics, move_fn: F) -> Result<Self, SelectionsError>
    where
        F: Fn(&Selection, &Rope, CursorSemantics) -> Result<Selection, SelectionError>
    {
        let mut new_selections = self.clone();
        if let Ok(primary_only) = self.clear_non_primary_selections(){new_selections = primary_only;}   //intentionally ignoring any errors
        match move_fn(&new_selections.primary().clone(), text, semantics){
            Ok(new_selection) => {
                new_selections = Selections::new(vec![new_selection], 0, text);
            }
            Err(e) => {
                match e{
                    SelectionError::ResultsInSameState => {return Err(SelectionsError::ResultsInSameState);}
                    //figure out what to do with other errors, if they can even happen...
                    SelectionError::DirectionMismatch |
                    SelectionError::SpansMultipleLines |//InvalidInput |
                    SelectionError::NoOverlap => {unreachable!()}   //if this is reached, move_fn called on one of the selections has probably put us in an unintended state. prob best to panic
                }
            }
        }
        Ok(new_selections)
    }
    
    //TODO: move_cursor_page    //should this be like move_cursor_clearing_non_primary or move_cursor_potentially_overlapping?...   vscode behavior seems to be equivalent to move_cursor_potentially_overlapping
    /// Intended to ease the use of Selection functions, when used over multiple selections, where the returned selections are moved by view height and could be overlapping.
    ///     intended for use with:
    ///         move page up
    ///         move page down
    ///         extend page up
    ///         extend page down
    pub fn move_cursor_page<F>(&self, text: &Rope, view: &View, semantics: CursorSemantics, move_fn: F) -> Result<Self, SelectionsError>
        where F: Fn(&Selection, &Rope, &View, CursorSemantics) -> Result<Selection, SelectionError>
    {
        let mut new_selections = Vec::with_capacity(self.count());  //the maximum size this vec should ever be is num selections in self
        for selection in self.iter(){
            match move_fn(selection, text, view, semantics){
                Ok(new_selection) => {new_selections.push(new_selection);}
                Err(e) => {
                    match e{
                        SelectionError::ResultsInSameState => {
                            if self.count() == 1{return Err(SelectionsError::ResultsInSameState)}
                            new_selections.push(selection.clone()); //retains selections with no change resulting from move_fn
                        }
                        //TODO: figure out what to do with other errors, if they can even happen...
                        //are we guaranteed by fn impls to never have these errors returned?
                        //what if user passes an unintended move_fn to this one?...
                        SelectionError::DirectionMismatch |
                        SelectionError::SpansMultipleLines |//InvalidInput |
                        SelectionError::NoOverlap => {
                            //unreachable!()
                            println!("{e:#?}");
                        }   //if this is reached, move_fn called on one of the selections has probably put us in an unintended state. prob best to panic
                    }
                }
            }
        }
        let mut new_selections = Selections::new(new_selections, self.primary_selection_index, text);
        if let Ok(merged_selections) = new_selections.merge_overlapping(text, semantics){
            new_selections = merged_selections;
        }
        Ok(new_selections)
    }
}
