use ropey::Rope;
use crate::selection::{CursorSemantics, Selection, Selection2d, Selections};
use crate::Position;



#[derive(Debug, PartialEq)]
pub enum ViewError{
    ResultsInSameState,
    InvalidInput,
}
/// The dimensions of the area a client has for displaying a document
/// origin is top left
#[derive(Debug, Default, Clone, PartialEq)]
pub struct View{
    /// from left to right
    horizontal_start: usize,
    /// from top to bottom
    vertical_start: usize,
    width: usize,
    height: usize,
}
impl View{
    /// Returns a new instance of [`View`] from provided inputs.
    pub fn new(horizontal_start: usize, vertical_start: usize, width: usize, height: usize) -> Self{
        Self{horizontal_start, vertical_start, width, height}
    }
    pub fn set_size(&mut self, width: usize, height: usize){
        self.width = width;
        self.height = height;
    }
    pub fn height(&self) -> usize{
        self.height
    }
    pub fn horizontal_start(&self) -> usize{
        self.horizontal_start
    }

    /// Returns a new instance of [`View`] with `vertical_start` increased by specified amount.
    pub fn scroll_down(&self, amount: usize, text: &Rope) -> Result<Self, ViewError>{
        assert!(text.len_lines() > 0);

        if amount == 0{return Err(ViewError::InvalidInput);}

        let max_scrollable_position = text.len_lines().saturating_sub(self.height);
        if self.vertical_start == max_scrollable_position{return Err(ViewError::ResultsInSameState);}
        
        let new_vertical_start = self.vertical_start.saturating_add(amount);

        if new_vertical_start <= max_scrollable_position{
            Ok(Self::new(self.horizontal_start, new_vertical_start, self.width, self.height))
        }else{
            Ok(Self::new(self.horizontal_start, max_scrollable_position, self.width, self.height))
        }
    }
    /// Returns a new instance of [`View`] with `horizontal_start` decreased by specified amount.
    pub fn scroll_left(&self, amount: usize) -> Result<Self, ViewError>{
        if amount == 0{return Err(ViewError::InvalidInput);}
        if self.horizontal_start == 0{return Err(ViewError::ResultsInSameState);}
        Ok(Self::new(self.horizontal_start.saturating_sub(amount), self.vertical_start, self.width, self.height))
    }
    /// Returns a new instance of [`View`] with `horizontal_start` increased by specified amount.
    pub fn scroll_right(&self, amount: usize, text: &Rope) -> Result<Self, ViewError>{
        if amount == 0{return Err(ViewError::InvalidInput);}

        // TODO: cache longest as a field in [`View`] struct to eliminate having to calculate this on each call
        // Calculate the longest line width in a single pass
        let longest = text.lines()
            .map(|line| crate::text_util::line_width(line, false))
            .max()
            .unwrap_or(0); // Handle the case where there are no lines

        let new_horizontal_start = self.horizontal_start.saturating_add(amount);

        if new_horizontal_start + self.width <= longest{
            Ok(Self::new(new_horizontal_start, self.vertical_start, self.width, self.height))
        }else{
            //Ok(self.clone())
            Err(ViewError::ResultsInSameState)
        }
    }
    /// Returns a new instance of [`View`] with `vertical_start` decreased by specified amount.
    pub fn scroll_up(&self, amount: usize) -> Result<View, ViewError>{
        if amount == 0{return Err(ViewError::InvalidInput);}
        if self.vertical_start == 0{return Err(ViewError::ResultsInSameState);}
        Ok(Self::new(self.horizontal_start, self.vertical_start.saturating_sub(amount), self.width, self.height))
    }
    /// Returns a `bool` indicating whether the [`View`] should be scrolled or not. If `head` of primary [`Selection2d`]
    /// is outside [`View`] boundaries, [`View`] should be scrolled.
    pub fn should_scroll(&self, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> bool{
        assert!(selection.cursor(text, semantics) <= text.len_chars());

        let cursor = selection.selection_to_selection2d(text, semantics);
        let cursor_y = cursor.head().y();
        let cursor_x = cursor.head().x();

        let within_vertical_bounds = cursor_y >= self.vertical_start && cursor_y < self.vertical_start.saturating_add(self.height);
        let within_horizontal_bounds = cursor_x >= self.horizontal_start && cursor_x < self.horizontal_start.saturating_add(self.width);

        !(within_vertical_bounds && within_horizontal_bounds)
    }

    /// Returns a new instance of [`View`] with `horizontal_start` and/or `vertical_start` shifted to keep `head` of
    /// [`Selection`] in [`View`].
    /// Can follow any specified selection, not just primary selection.
    pub fn scroll_following_cursor(&self, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Self{
        assert!(selection.cursor(text, semantics) <= text.len_chars());

        let cursor = selection.selection_to_selection2d(text, semantics);
        let cursor_y = cursor.head().y();
        let cursor_x = cursor.head().x();

        let mut new_view = self.clone();

        // Adjust vertical view based on cursor position
        if cursor_y < self.vertical_start{
            new_view.vertical_start = cursor_y;
        }else if cursor_y >= self.vertical_start.saturating_add(self.height){
            new_view.vertical_start = cursor_y.saturating_sub(self.height).saturating_add(1);
        }

        // Adjust horizontal view based on cursor position
        if cursor_x < self.horizontal_start{
            new_view.horizontal_start = cursor_x;
        }else if cursor_x >= self.horizontal_start.saturating_add(self.width){
            new_view.horizontal_start = cursor_x.saturating_sub(self.width).saturating_add(1);
        }

        new_view
    }

    /// Returns an instance of [`View`] vertically centered around specified cursor.
    pub fn center_vertically_around_cursor(&self, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Self, ViewError>{
        assert!(selection.cursor(text, semantics) <= text.len_chars());    //ensure selection is valid
        assert!(text.len_lines() > 0);  //ensure text is not empty
        
        let current_line = text.char_to_line(selection.cursor(text, semantics));
        //let view_is_even_numbered = self.height % 2 == 0;
        let half_view_height = self.height / 2; //current impl will be biased towards the bottom of the view, if view is even numbered

        //TODO: consider how even numbered view heights should be handled...
        // maybe < half_view_height.saturating_sub(1)
        if current_line <= half_view_height{return Err(ViewError::ResultsInSameState);} //maybe return error cursor before doc_start + half the view height
        if current_line >= text.len_lines().saturating_sub(half_view_height){return Err(ViewError::ResultsInSameState);}    //maybe return error cursor after doc_end - half the view height

        // Calculate the new vertical start position
        let new_vertical_start = if current_line > half_view_height{
            current_line.saturating_sub(half_view_height)
        }else{
            0
        }.min(text.len_lines().saturating_sub(self.height));    //should self.height be half_view_height?

        // if view_is_even_numbered && (current_line == new_vertical_start || current_line == new_vertical_start.saturating_sub(1)){return Err(ViewError::ResultsInSameState);}
        if current_line == new_vertical_start{return Err(ViewError::ResultsInSameState);}   //maybe return error already centered   //TODO: and test
        //

        Ok(Self::new(self.horizontal_start, new_vertical_start, self.width, self.height))
    }

    /// Returns a `String` containing the text that can be contained within [`View`] boundaries.
    // TODO: need to handle displaying TAB_WIDTH spaces instead of a "\t" character.
    // TODO: test
    //pub fn text(&self, text: &Rope) -> String{
    //    // preallocate memory for String based on expected size
    //    let mut client_view_text = String::with_capacity(self.height * (self.width + 1));   //+1 for added new line
    //
    //    let vertical_range = self.vertical_start..self.vertical_start + self.height;
    //    let horizontal_range = self.horizontal_start..self.horizontal_start + self.width;
    //
    //    for (y, line) in text.lines().enumerate(){
    //        if !vertical_range.contains(&y){
    //            continue;
    //        }
    //
    //        let bounded_line: String = line.chars()
    //            .enumerate()
    //            .filter_map(|(x, char)|{
    //                if horizontal_range.contains(&x) && char != '\n'{
    //                    Some(char)
    //                }else{
    //                    None
    //                }
    //            })
    //            .collect();
    //
    //        client_view_text.push_str(&bounded_line);
    //        client_view_text.push('\n'); // Append newline after each line
    //    }
    //
    //    client_view_text
    //}
    // returns text using view blocks, but may be harder to implement hard tab handling, or other wide characters
    pub fn text(&self, text: &Rope) -> String{
        let view_blocks = self.view_blocks(text, false);
        let mut client_view_text = String::new();
    
        for view_block in view_blocks.iter(){
            client_view_text.push_str(&text.slice(view_block.start()..view_block.end()).to_string());
            client_view_text.push('\n');
        }
    
        client_view_text
    }
    

    /// Returns a `String` containing the line numbers of the text that can be contained within [`View`] boundaries.
    pub fn line_numbers(&self, text: &Rope) -> String{
        //enhance performance by building the string using a vector and then joining it at the end
        let mut line_numbers_vec = Vec::with_capacity(self.height);

        let vertical_range = self.vertical_start..self.vertical_start + self.height;

        for (y, _) in text.lines().enumerate(){
            if vertical_range.contains(&y){
                line_numbers_vec.push((y + 1).to_string()); // Convert number to string
            }
        }

        line_numbers_vec.join("\n") // Join with newline
    }

    /// Returns a [`Vec`] of [`Selection2d`]s that represent [`Selection`]s with any portion of itself within the boundaries of [`View`].
    /// Returned selections should be in screen space coordinates.
    /// Assumes selections are already sorted and merged.
    // should this return Option<Vec<Selection, usize>> with usize being the selection's line number instead?
    pub fn selections(&self, selections: &Selections, text: &Rope) -> Option<Vec<Selection2d>>{
        let view_blocks = self.view_blocks(text, true); //make sure to adjust tests to include newline
        let mut selections_in_view = Vec::with_capacity(view_blocks.len() * self.width);

        for (y, view_block) in view_blocks.iter().enumerate(){
            let view_start = view_block.anchor();
            let mut intersected = false;
            for selection in selections.iter(){
                if let Ok(selected) = view_block.intersection(selection){
                    // add intersecting to list //this represents a selection in view bounds
                    let new_anchor = Position::new(selected.anchor() - view_start, y);
                    let new_head = Position::new(selected.head() - view_start, y);
                    //selections_in_view.push(Selection2d::new(Position::new(selected.anchor() - view_start, y), Position::new(selected.head() - view_start, y)));
                    selections_in_view.push(Selection2d::new(new_anchor, new_head));
                    intersected = true;
                }
            }

            if !intersected{
                // retain non intersecting view_blocks  //this represents no selection in view
                selections_in_view.push(Selection2d::new(Position::new(0, y), Position::new(0, y)));
            }
        }

        Some(selections_in_view)
    }

    /// Maps a [`View`] as a [`Vec`] of [`Selection`]s over a text rope.
    // should this include newlines('\n') in its width calculation? maybe pass in include_newline bool?
    // we want to highlight newlines as well
    // but that may mess with the logic for "empty" lines...idk
    pub fn view_blocks(&self, text: &Rope, include_newline: bool) -> Vec<Selection>{
        let mut view_blocks = Vec::new();
        let vertical_range = self.vertical_start..self.vertical_start + self.height;

        for (y, line) in text.lines().enumerate(){
            // only include lines in vertical bounds
            if vertical_range.contains(&y){
                let line_start = text.line_to_char(y);
                let line_width = crate::text_util::line_width(line, include_newline);
                let line_end = line_start + line_width;
                
                let mut view_start = line_start + self.horizontal_start;    //start view at horizontal offset of view
                let mut view_end = view_start + line_width.min(self.width); //restrict view width

                if line_end < view_start{   //handle view shifted right past end of line text(includes empty lines)
                    view_start = line_start;
                    view_end = line_start;  //zero width output selections represent a line with no text in view bounds
                }
                else if line_end < view_end{
                    view_end = line_end;
                }
                view_blocks.push(Selection::new(view_start, view_end));
            }
        }

        view_blocks
    }
    

    /// Returns cursor positions that are within [`View`] boundaries.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics, Selections};
    /// # use edit_core::view::View;
    /// # use edit_core::document::Document;
    /// # use edit_core::Position;
    /// 
    /// fn test(selection: Selection, expected: Vec<Position>, view: View, semantics: CursorSemantics) -> bool{
    ///     let text = Rope::from("idk\nsome\nshit\n");
    ///     let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text)).with_view(view);
    ///     println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.view().cursor_positions(&text, &doc.selections(), semantics));
    ///     doc.view().cursor_positions(&text, &doc.selections(), semantics) == expected
    /// }
    /// 
    /// assert!(test(Selection::new(0, 0), vec![Position::new(0, 0)], View::new(0, 0, 2, 2), CursorSemantics::Bar));
    /// assert!(test(Selection::new(0, 1), vec![Position::new(0, 0)], View::new(0, 0, 2, 2), CursorSemantics::Block));
    /// assert!(test(Selection::new(0, 0), Vec::new(), View::new(1, 0, 2, 2), CursorSemantics::Bar));
    /// assert!(test(Selection::new(0, 1), Vec::new(), View::new(1, 0, 2, 2), CursorSemantics::Block));
    /// assert!(test(Selection::new(0, 0), Vec::new(), View::new(1, 1, 2, 2), CursorSemantics::Bar));
    /// assert!(test(Selection::new(0, 1), Vec::new(), View::new(1, 1, 2, 2), CursorSemantics::Block));
    /// ```
    pub fn cursor_positions(&self, text: &Rope, selections: &Selections, semantics: CursorSemantics) -> Vec<Position>{
        selections.iter()
            .filter_map(|cursor|{
                Self::cursor_position(cursor.selection_to_selection2d(text, semantics), self)
            })
            .collect()
    }
    // translates a document cursor position to a client view cursor position. if outside client view, returns None
    fn cursor_position(doc_cursor: Selection2d, client_view: &View) -> Option<Position>{
        let head_x = doc_cursor.head().x();
        let head_y = doc_cursor.head().y();

        let in_horizontal_bounds = head_x >= client_view.horizontal_start
            && head_x < client_view.horizontal_start.saturating_add(client_view.width);
    
        let in_vertical_bounds = head_y >= client_view.vertical_start
            && head_y < client_view.vertical_start.saturating_add(client_view.height);

        if in_horizontal_bounds && in_vertical_bounds{
            Some(Position::new(
                head_x.saturating_sub(client_view.horizontal_start),
                head_y.saturating_sub(client_view.vertical_start),
            ))
        }else{
            None
        }
    }
    pub fn primary_cursor_position(&self, text: &Rope, selections: &Selections, semantics: CursorSemantics) -> Option<Position>{
        let primary = selections.primary();
        Self::cursor_position(primary.selection_to_selection2d(text, semantics), self)
    }
}
