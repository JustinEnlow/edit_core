use ropey::Rope;
use crate::selection::{CursorSemantics, Selection, Selection2d, Selections};
use crate::Position;
use crate::document::TAB_WIDTH;



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
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // scrolls when vertical space remaining in text
    /// let view = View::new(0, 0, 2, 2);
    /// assert_eq!(View::new(0, 1, 2, 2), view.scroll_down(1, &text));
    /// assert_eq!(String::from("so\nsh\n"), view.scroll_down(1, &text).text(&text));
    /// 
    /// // scrolling saturates at limits of text
    /// let view = View::new(0, 2, 2, 2);
    /// assert_eq!(View::new(0, 2, 2, 2), view.scroll_down(1, &text));
    /// assert_eq!(String::from("sh\n\n"), view.scroll_down(1, &text).text(&text));
    /// ```
    #[must_use]
    pub fn scroll_down(&self, amount: usize, text: &Rope) -> Self{
        assert!(amount > 0);
        assert!(text.len_lines() > 0);
        
        let new_vertical_start = self.vertical_start.saturating_add(amount);
        let max_scrollable_position = text.len_lines().saturating_sub(self.height);

        if new_vertical_start <= max_scrollable_position{
            Self::new(self.horizontal_start, new_vertical_start, self.width, self.height)
        }else{
            Self::new(self.horizontal_start, max_scrollable_position, self.width, self.height)
        }
    }
    /// Returns a new instance of [`View`] with `horizontal_start` decreased by specified amount.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // scrolling saturates at limits of text
    /// let view = View::new(0, 0, 2, 2);
    /// assert_eq!(View::new(0, 0, 2, 2), view.scroll_left(1));
    /// assert_eq!(String::from("id\nso\n"), view.scroll_left(1).text(&text));
    /// 
    /// // scrolls when horizontal space remaining in text
    /// let view = View::new(2, 0, 2, 2);
    /// assert_eq!(View::new(1, 0, 2, 2), view.scroll_left(1));
    /// assert_eq!(String::from("dk\nom\n"), view.scroll_left(1).text(&text));
    /// ```
    #[must_use]
    pub fn scroll_left(&self, amount: usize) -> Self{
        assert!(amount > 0);
        Self::new(self.horizontal_start.saturating_sub(amount), self.vertical_start, self.width, self.height)
    }
    /// Returns a new instance of [`View`] with `horizontal_start` increased by specified amount.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // scrolling saturates at limits of text
    /// let view = View::new(2, 0, 2, 2);
    /// assert_eq!(View::new(2, 0, 2, 2), view.scroll_right(1, &text));
    /// assert_eq!(String::from("k\nme\n"), view.scroll_right(1, &text).text(&text));
    /// 
    /// // scrolls when horizontal space remaining in text
    /// let view = View::new(0, 0, 2, 2);
    /// assert_eq!(View::new(1, 0, 2, 2), view.scroll_right(1, &text));
    /// assert_eq!(String::from("dk\nom\n"), view.scroll_right(1, &text).text(&text));
    /// ```
    #[must_use]
    pub fn scroll_right(&self, amount: usize, text: &Rope) -> Self{
        assert!(amount > 0);

        // TODO: cache longest as a field in [`View`] struct to eliminate having to calculate this on each call
        // Calculate the longest line width in a single pass
        let longest = text.lines()
            .map(crate::text_util::line_width_excluding_newline)
            .max()
            .unwrap_or(0); // Handle the case where there are no lines

        let new_horizontal_start = self.horizontal_start.saturating_add(amount);

        if new_horizontal_start + self.width <= longest{
            Self::new(new_horizontal_start, self.vertical_start, self.width, self.height)
        }else{
            self.clone()
        }
    }
    /// Returns a new instance of [`View`] with `vertical_start` decreased by specified amount.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // scrolls when vertical space remaining in text
    /// let view = View::new(0, 2, 2, 2);
    /// assert_eq!(View::new(0, 1, 2, 2), view.scroll_up(1));
    /// assert_eq!(String::from("so\nsh\n"), view.scroll_up(1).text(&text));
    /// 
    /// // scrolling saturates at limits of text
    /// let view = View::new(0, 0, 2, 2);
    /// assert_eq!(View::new(0, 0, 2, 2), view.scroll_up(1));
    /// assert_eq!(String::from("id\nso\n"), view.scroll_up(1).text(&text));
    /// ```
    #[must_use]
    pub fn scroll_up(&self, amount: usize) -> View{
        assert!(amount > 0);
        Self::new(self.horizontal_start, self.vertical_start.saturating_sub(amount), self.width, self.height)
    }
    /// Returns a `bool` indicating whether the [`View`] should be scrolled or not. If `head` of primary [`Selection2d`]
    /// is outside [`View`] boundaries, [`View`] should be scrolled.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let view = View::new(0, 0, 2, 2);
    /// 
    /// // in view
    /// let selection = Selection::new(0, 0);
    /// assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    /// let selection = Selection::new(0, 1);
    /// assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Block));
    /// 
    /// // out of view horizontally
    /// let selection = Selection::new(3, 3);
    /// assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    /// let selection = Selection::new(3, 4);
    /// assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
    /// 
    /// // out of view vertically
    /// let selection = Selection::new(10, 10);
    /// assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    /// let selection = Selection::new(10, 11);
    /// assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
    /// ```
    #[must_use]
    pub fn should_scroll(&self, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> bool{
        assert!(selection.cursor(semantics) <= text.len_chars());

        let cursor = selection.selection_to_selection2d(text, semantics);
        let cursor_y = cursor.head().y();
        let cursor_x = cursor.head().x();

        let within_vertical_bounds = cursor_y >= self.vertical_start && cursor_y < self.vertical_start.saturating_add(self.height);
        let within_horizontal_bounds = cursor_x >= self.horizontal_start && cursor_x < self.horizontal_start.saturating_add(self.width);

        !(within_vertical_bounds && within_horizontal_bounds)
    }

    /// Returns a new instance of [`View`] with `horizontal_start` and/or `vertical_start` shifted to keep `head` of
    /// [`Selection`] in [`View`].
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let view = View::new(0, 0, 2, 2);
    /// 
    /// // return self when primary [`Selection`] `head` within [`View`] bounds
    /// let selection = Selection::new(0, 0);
    /// assert_eq!(view, view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar));
    /// assert_eq!(String::from("id\nso\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar).text(&text));
    /// let selection = Selection::new(0, 1);
    /// assert_eq!(view, view.scroll_following_cursor(&selection, &text, CursorSemantics::Block));
    /// assert_eq!(String::from("id\nso\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block).text(&text));
    /// 
    /// // returns proper [`View`] when [`Selection`] `head` outside [`View`] bounds
    /// let selection = Selection::new(13, 13);
    /// assert_eq!(View::new(3, 1, 2, 2), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar));
    /// assert_eq!(String::from("e\nt\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar).text(&text));
    /// let selection = Selection::new(13, 14);
    /// assert_eq!(View::new(3, 1, 2, 2), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block));
    /// assert_eq!(String::from("e\nt\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block).text(&text));
    /// ```
    #[must_use]
    pub fn scroll_following_cursor(&self, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Self{
        assert!(selection.cursor(semantics) <= text.len_chars());

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
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::view::View;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\nand\nsomething\nelse\n");   //len 33
    /// let view = View::new(0, 0, 1, 1);
    /// 
    /// let selection = Selection::new(14, 14);
    /// assert_eq!(View::new(0, 3, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
    /// let selection = Selection::new(14, 15);
    /// assert_eq!(View::new(0, 3, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));
    /// 
    /// let selection = Selection::new(0, 0);
    /// assert_eq!(view, view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
    /// let selection = Selection::new(0, 1);
    /// assert_eq!(view, view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));
    /// 
    /// let selection = Selection::new(33, 33);
    /// assert_eq!(View::new(0, 6, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
    /// let selection = Selection::new(33, 34);
    /// assert_eq!(View::new(0, 6, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));
    /// ```
    #[must_use]
    pub fn center_vertically_around_cursor(&self, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Self{
        assert!(selection.cursor(semantics) <= text.len_chars());    //ensure selection is valid
        assert!(text.len_lines() > 0);  //ensure text is not empty
        
        let current_line = text.char_to_line(selection.cursor(semantics));
        let half_view_height = self.height / 2;

        // Calculate the new vertical start position
        let new_vertical_start = if current_line > half_view_height{
            current_line.saturating_sub(half_view_height)
        }else{
            0
        }.min(text.len_lines().saturating_sub(self.height));

        Self::new(self.horizontal_start, new_vertical_start, self.width, self.height)
    }
    /// Returns a `String` containing the text that can be contained within [`View`] boundaries.
    // TODO: need to handle displaying TAB_WIDTH spaces instead of a "\t" character.
    pub fn text(&self, text: &Rope) -> String{
        // preallocate memory for String based on expected size
        let mut client_view_text = String::with_capacity(self.height * (self.width + 1));   //+1 for added new line

        let vertical_range = self.vertical_start..self.vertical_start + self.height;
        let horizontal_range = self.horizontal_start..self.horizontal_start + self.width;

        for (y, line) in text.lines().enumerate(){
            if !vertical_range.contains(&y){
                continue;
            }

            let bounded_line: String = line.chars()
                .enumerate()
                .filter_map(|(x, char)|{
                    if horizontal_range.contains(&x) && char != '\n'{
                        Some(char)
                    }else{
                        None
                    }
                })
                .collect();

            client_view_text.push_str(&bounded_line);
            client_view_text.push('\n'); // Append newline after each line
        }

        client_view_text
    }
    //pub fn text(&self, text: &Rope) -> String {
    //    // preallocate memory for String based on expected size
    //    let mut client_view_text = String::with_capacity(self.height * (self.width + 1)); // +1 for added new line
    //
    //    let vertical_range = self.vertical_start..(self.vertical_start + self.height);
    //    let horizontal_range = self.horizontal_start..(self.horizontal_start + self.width);
    //
    //    for (y, line) in text.lines().enumerate(){
    //        // skip lines outside vertical range
    //        if !vertical_range.contains(&y){
    //            continue;
    //        }
    //
    //        let mut bounded_line = String::new();   // init new bounded line to be rendered
    //        let mut x = 0; // Track the current x position
    //
    //        for char in line.chars(){
    //            if x >= self.width{
    //                break; // Stop if maximum width reached
    //            }
    //
    //            match char{
    //                '\t' => {
    //                    let spaces_to_add = TAB_WIDTH - (x % TAB_WIDTH);
    //                    bounded_line.push_str(&" ".repeat(spaces_to_add));
    //                    x += spaces_to_add; // Update x position
    //                    x = x.min(self.width);
    //                }
    //                '\n' => {
    //                    // ignore newline characters. appropriate newlines will be appended in a later step
    //                    continue;
    //                }
    //                _ => {
    //                    // skip characters outside horizontal range
    //                    if horizontal_range.contains(&x){
    //                        bounded_line.push(char);
    //                        x += 1; // Update x position for regular characters
    //                    }
    //                }
    //            }
    //        }
    //
    //        client_view_text.push_str(&bounded_line);
    //        client_view_text.push('\n'); // Append newline after each line
    //    }
    //
    //    client_view_text
    //}
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

    /*
    pub fn selections(&self) -> 2dSelections?{
        for all selections in view,
        return selection with start position, end position, and cursor position
    }
    */

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
}
