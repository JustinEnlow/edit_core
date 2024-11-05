// follow documentation style from https://std-dev-guide.rust-lang.org/development/how-to-write-documentation.html

use ropey::Rope;
use crate::{
    text_util, view::View, Position
};



#[derive(Clone, Copy, Debug)]
pub enum CursorSemantics{
    // default selection has width of 0
    Bar,
    // default selection has width of 1
    Block
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

/// 1 dimensional representation of a single selection(between anchor and head) within a text rope.
/// a cursor is a selection with an anchor/head difference of 0 or 1(depending on cursor semantics)
/// Should ensure head/anchor are always within text bounds
#[derive(PartialEq, Clone, Debug)]
pub struct Selection{
    /// the stationary portion of a selection.
    anchor: usize,
    /// the mobile portion of a selection. this is the portion a user can move to extend selection
    head: usize,
    /// the offset from the start of the line self.head is on
    stored_line_position: Option<usize>,
}
impl Selection{
    /// Returns a new instance of [`Selection`].
    #[must_use]
    pub fn new(anchor: usize, head: usize) -> Self{ // could init with cursor semantics: (anchor: usize, cursor: usize, semantics: CursorSemantics)
        Self{anchor, head, stored_line_position: None}
    }
    /// Returns a new instance of [`Selection`] with a specified `stored_line_position`.
    /// Mainly used for testing.
    pub fn with_stored_line_position(anchor: usize, head: usize, stored_line_position: usize) -> Self{
        Self{anchor, head, stored_line_position: Some(stored_line_position)}
    }
    /// Returns the char index of [`Selection`] anchor.
    #[must_use]
    pub fn anchor(&self) -> usize{
        self.anchor
    }
    /// Returns the char index of [`Selection`] head.
    #[must_use]
    pub fn head(&self) -> usize{
        self.head
    }

    /// Returns the char index of the start of the [`Selection`] from left to right.
    /// ```
    /// # use edit_core::selection::Selection;
    /// 
    /// assert_eq!(0, Selection::new(0, 4).start());
    /// assert_eq!(0, Selection::new(4, 0).start());
    /// ```
    #[must_use]
    pub fn start(&self) -> usize{
        std::cmp::min(self.anchor, self.head)
    }
    /// Returns the char index of the end of the [`Selection`] from left to right.
    /// ```
    /// # use edit_core::selection::Selection;
    /// 
    /// assert_eq!(4, Selection::new(0, 4).end());
    /// assert_eq!(4, Selection::new(4, 0).end());
    /// ```
    #[must_use]
    pub fn end(&self) -> usize{
        std::cmp::max(self.anchor, self.head)
    }

    /// Returns `true` if [`Selection`] len > 0 with bar cursor semantics, or 
    /// [`Selection`] len > 1 with block cursor semantics, or else returns `false`.
    /// ```
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// assert_eq!(Selection::new(0, 0).is_extended(CursorSemantics::Bar), false);
    /// assert_eq!(Selection::new(0, 1).is_extended(CursorSemantics::Bar), true);
    /// assert_eq!(Selection::new(1, 0).is_extended(CursorSemantics::Bar), true);
    /// 
    /// assert_eq!(Selection::new(0, 1).is_extended(CursorSemantics::Block), false);
    /// assert_eq!(Selection::new(1, 0).is_extended(CursorSemantics::Block), false);
    /// assert_eq!(Selection::new(0, 2).is_extended(CursorSemantics::Block), true);
    /// assert_eq!(Selection::new(2, 0).is_extended(CursorSemantics::Block), true);
    /// ```
    #[must_use]
    pub fn is_extended(&self, semantics: CursorSemantics) -> bool{
        //self.anchor != self.cursor(semantics)
        match semantics{
            CursorSemantics::Bar => self.end().saturating_sub(self.start()) > 0,
            CursorSemantics::Block => self.end().saturating_sub(self.start()) > 1  //if selection is greater than one grapheme //currently uses char count though...
        }
    }

    /// Returns the direction of [`Selection`].
    /// ```
    /// # use edit_core::selection::{Selection, Direction, CursorSemantics};
    /// 
    /// assert_eq!(Selection::new(0, 0).direction(CursorSemantics::Bar), Direction::Forward);
    /// assert_eq!(Selection::new(0, 1).direction(CursorSemantics::Bar), Direction::Forward);
    /// assert_eq!(Selection::new(1, 0).direction(CursorSemantics::Bar), Direction::Backward);
    /// //assert_eq!(Selection::new(0, 0).direction(CursorSemantics::Block), Direction::Backward);    //state shouldn't be possible with block cursor semantics, so this failure is fine.
    /// assert_eq!(Selection::new(0, 1).direction(CursorSemantics::Block), Direction::Forward);
    /// assert_eq!(Selection::new(1, 0).direction(CursorSemantics::Block), Direction::Backward);
    /// assert_eq!(Selection::new(1, 1).direction(CursorSemantics::Block), Direction::Backward); //state shouldn't be possible with block cursor semantics, but the result is still valid.
    /// ```
    #[must_use]
    pub fn direction(&self, semantics: CursorSemantics) -> Direction{
        //assert!(self.cursor(semantics) <= text.len_chars());  we would need a & to text
        //assert!(self.anchor <= text.len_chars());
        if self.cursor(semantics) < self.anchor{
            Direction::Backward
        }else{
            Direction::Forward
        }
    }

    /// Sets [`Selection`] direction to specified direction.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Direction, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::new(0, 0).set_direction(Direction::Forward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 0).set_direction(Direction::Backward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 5).set_direction(Direction::Backward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(5, 0, 0));
    /// assert_eq!(Selection::new(5, 0).set_direction(Direction::Forward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 5, 1));
    /// 
    /// assert_eq!(Selection::new(0, 1).set_direction(Direction::Backward, &text, CursorSemantics::Block), Selection::with_stored_line_position(1, 0, 0));
    /// assert_eq!(Selection::new(1, 0).set_direction(Direction::Forward, &text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(0, 5).set_direction(Direction::Backward, &text, CursorSemantics::Block), Selection::with_stored_line_position(5, 0, 0));
    /// assert_eq!(Selection::new(5, 0).set_direction(Direction::Forward, &text, CursorSemantics::Block), Selection::with_stored_line_position(0, 5, 0));
    /// ```
    #[must_use]
    pub fn set_direction(&self, direction: Direction, text: &Rope, semantics: CursorSemantics) -> Self{
        assert!(self.start() <= self.end());
        assert!(text.len_lines() > 0);
        
        let (anchor, head) = match direction {
            Direction::Forward => (self.start(), self.end()),
            Direction::Backward => (self.end(), self.start()),
        };
    
        let mut selection = Selection::new(anchor, head);
        selection.stored_line_position = Some(text_util::offset_from_line_start(selection.cursor(semantics), text));
    
        selection
    }

    /// Checks `self` and `other` for overlap.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::Selection;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // test key: 
    /// //    selection1 anchor = [
    /// //    selection1 head   = ]
    /// //    selection2 anchor = <
    /// //    selection2 head   = >
    /// 
    /// // non zero width selections, no overlap
    /// assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(3, 6)), false); //[idk]<\nso>me\nshit\n
    /// assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(6, 3)), false); //[idk]>\nso<me\nshit\n
    /// assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(3, 6)), false); //]idk[<\nso>me\nshit\n
    /// assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(6, 3)), false); //]idk[>\nso<me\nshit\n
    /// assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(0, 3)), false); //<idk>[\nso]me\nshit\n
    /// assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(3, 0)), false); //>idk<[\nso]me\nshit\n
    /// assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(0, 3)), false); //<idk>]\nso[me\nshit\n
    /// assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(3, 0)), false); //>idk<]\nso[me\nshit\n
    /// 
    /// // non-zero-width selections, overlap.
    /// assert_eq!(Selection::new(0, 4).overlaps(&Selection::new(3, 6)), true);  //[idk<\n]so>me\nshit\n
    /// assert_eq!(Selection::new(0, 4).overlaps(&Selection::new(6, 3)), true);  //[idk>\n]so<me\nshit\n
    /// assert_eq!(Selection::new(4, 0).overlaps(&Selection::new(3, 6)), true);  //]idk<\n[so>me\nshit\n
    /// assert_eq!(Selection::new(4, 0).overlaps(&Selection::new(6, 3)), true);  //]idk>\n[so<me\nshit\n
    /// assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(0, 4)), true);  //<idk[\n>so]me\nshit\n
    /// assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(4, 0)), true);  //>idk[\n<so]me\nshit\n
    /// assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(0, 4)), true);  //<idk]\n>so[me\nshit\n
    /// assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(4, 0)), true);  //>idk]\n<so[me\nshit\n
    /// 
    /// // Zero-width and non-zero-width selections, overlap.
    /// assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(3, 3)), true);  //[idk<>]\nsome\nshit\n
    /// assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(3, 3)), true);  //]idk<>[\nsome\nshit\n
    /// assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(0, 3)), true);  //<idk[]>\nsome\nshit\n
    /// assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(3, 0)), true);  //>idk[]<\nsome\nshit\n
    /// 
    /// // Zero-width and non-zero-width selections, overlap.
    /// assert_eq!(Selection::new(1, 4).overlaps(&Selection::new(1, 1)), true);  //i[<>dk\n]some\nshit\n
    /// assert_eq!(Selection::new(4, 1).overlaps(&Selection::new(1, 1)), true);  //i]<>dk\n[some\nshit\n
    /// assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(1, 4)), true);  //i[<]dk\n>some\nshit\n
    /// assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(4, 1)), true);  //i[>]dk\n<some\nshit\n
    /// assert_eq!(Selection::new(1, 4).overlaps(&Selection::new(3, 3)), true);  //i[dk<>\n]some\nshit\n
    /// assert_eq!(Selection::new(4, 1).overlaps(&Selection::new(3, 3)), true);  //i]dk<>\n[some\nshit\n
    /// assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(1, 4)), true);  //i<dk[]\n>some\nshit\n
    /// assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(4, 1)), true);  //i>dk[]\n<some\nshit\n
    /// 
    /// // zero-width selections, no overlap.
    /// assert_eq!(Selection::new(0, 0).overlaps(&Selection::new(1, 1)), false); //[]i<>dk\nsome\nshit\n
    /// assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(0, 0)), false); //<>i[]dk\nsome\nshit\n
    /// 
    /// // zero-width selections, overlap.
    /// assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(1, 1)), true);  //i[<>]dk\nsome\nshit\n
    /// ```
    #[must_use]
    pub fn overlaps(&self, other: &Selection) -> bool{
        assert!(self.start() <= self.end());
        assert!(other.start() <= other.end());
        
        self.start() == other.start() || 
        self.end() == other.end() || 
        (self.end() > other.start() && other.end() > self.start())
    }

    pub fn contains(&self, idx: usize) -> bool{
        idx >= self.start() && idx <= self.end()
    }

    /// Returns a new [`Selection`] from the overlap of `self` and `other`.
    /// ```
    /// # use edit_core::selection::Selection;
    /// 
    /// let first = Selection::new(0, 6);
    /// let second = Selection::new(3, 9);
    /// assert_eq!(Selection::new(3, 6), first.intersection(&second).unwrap());
    /// 
    /// let first = Selection::new(1, 5);
    /// let second = Selection::new(2, 3);
    /// assert_eq!(Selection::new(2, 3), first.intersection(&second).unwrap());
    /// ```
    pub fn intersection(&self, other: &Selection) -> Result<Self, ()>{
        if self.overlaps(other){
            Ok(Selection::new(self.start().max(other.start()), self.end().min(other.end())))
        }else{
            Err(())
        }
    }

    /// Create a new [`Selection`] by merging self with other.
    ///// Indiscriminate merge. merges whether overlapping, consecutive, 
    ///// contained, or disconnected entirely.
    /// resultant selection should be guaranteed to be within text bounds 
    /// because this uses previously initialized selections.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::Selection;
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // when self.anchor > self.head && other.anchor > other.head
    /// assert_eq!(Selection::new(4, 0).merge(&Selection::new(5, 1), &text), Selection::with_stored_line_position(0, 5, 1));
    /// assert_eq!(Selection::new(5, 1).merge(&Selection::new(4, 0), &text), Selection::with_stored_line_position(0, 5, 1));
    /// 
    /// // when self.anchor < self.head && other.anchor < other.head
    /// assert_eq!(Selection::new(0, 4).merge(&Selection::new(1, 5), &text), Selection::with_stored_line_position(0, 5, 1));
    /// assert_eq!(Selection::new(1, 5).merge(&Selection::new(0, 4), &text), Selection::with_stored_line_position(0, 5, 1));
    /// 
    /// // when self.anchor > self.head && other.anchor < other.head
    /// assert_eq!(Selection::new(4, 0).merge(&Selection::new(1, 5), &text), Selection::with_stored_line_position(0, 5, 1));
    /// assert_eq!(Selection::new(1, 5).merge(&Selection::new(4, 0), &text), Selection::with_stored_line_position(0, 5, 1));
    /// 
    /// // when self.anchor < self.head && other.anchor > other.head
    /// assert_eq!(Selection::new(0, 4).merge(&Selection::new(5, 1), &text), Selection::with_stored_line_position(0, 5, 1));
    /// assert_eq!(Selection::new(5, 1).merge(&Selection::new(0, 4), &text), Selection::with_stored_line_position(0, 5, 1));
    /// 
    /// // consecutive
    /// assert_eq!(Selection::new(0, 1).merge(&Selection::new(1, 2), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(1, 0).merge(&Selection::new(1, 2), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(1, 0).merge(&Selection::new(2, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(0, 1).merge(&Selection::new(2, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(1, 2).merge(&Selection::new(0, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(2, 1).merge(&Selection::new(0, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(2, 1).merge(&Selection::new(1, 0), &text), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(1, 2).merge(&Selection::new(1, 0), &text), Selection::with_stored_line_position(0, 2, 2));
    ///
    /// // overlapping
    /// assert_eq!(Selection::new(0, 2).merge(&Selection::new(1, 4), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(2, 0).merge(&Selection::new(1, 4), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(2, 0).merge(&Selection::new(4, 1), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(0, 2).merge(&Selection::new(4, 1), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(1, 4).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(4, 1).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(4, 1).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(1, 4).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 4, 0));
    /// 
    /// // contained
    /// assert_eq!(Selection::new(0, 6).merge(&Selection::new(2, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(6, 0).merge(&Selection::new(2, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(6, 0).merge(&Selection::new(4, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(0, 6).merge(&Selection::new(4, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(2, 4).merge(&Selection::new(0, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(4, 2).merge(&Selection::new(0, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(4, 2).merge(&Selection::new(6, 0), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(2, 4).merge(&Selection::new(6, 0), &text), Selection::with_stored_line_position(0, 6, 2));
    /// 
    /// // disconnected
    /// assert_eq!(Selection::new(0, 2).merge(&Selection::new(4, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(2, 0).merge(&Selection::new(4, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(2, 0).merge(&Selection::new(6, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(0, 2).merge(&Selection::new(6, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(4, 6).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(6, 4).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(6, 4).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 6, 2));
    /// assert_eq!(Selection::new(4, 6).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 6, 2));
    /// ```
    #[must_use]
    pub fn merge(&self, other: &Selection, text: &Rope) -> Selection{
        assert!(text.len_lines() > 0);
        assert!(self.head <= text.len_chars());
        assert!(self.anchor <= text.len_chars());
        assert!(other.head <= text.len_chars());
        assert!(other.anchor <= text.len_chars());
        
        let anchor = self.start().min(other.start());
        let head = self.end().max(other.end());
        let stored_line_position = text_util::offset_from_line_start(head, text);   //self.cursor instead of head?
        
        Selection{anchor, head, stored_line_position: Some(stored_line_position)}
    }

    /////////////////////////////////// Alignment Methods ///////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////

    /////////////////////////////////// Block Cursor Methods ///////////////////////////////////
    
    /// Returns the char index of [`Selection`] cursor.
    /// left side of cursor if block cursor semantics
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // key:
    /// // anchor             = |
    /// // head               = > if forward, < if backward
    /// // block_virtual_head = :
    /// 
    /// assert_eq!(Selection::new(0, 0).cursor(CursorSemantics::Bar), 0);   //|>idk\nsome\nshit\n
    /// assert_eq!(Selection::new(1, 2).cursor(CursorSemantics::Block), 1); //i|:d>k\nsome\nshit\n
    /// assert_eq!(Selection::new(2, 1).cursor(CursorSemantics::Block), 1); //i:<d|k\nsome\nshit\n
    /// assert_eq!(Selection::new(2, 2).cursor(CursorSemantics::Block), 1); //i:d|>k\nsome\nshit\n  //though this state should be impossible with block cursor semantics
    /// ```
    #[must_use]
    pub fn cursor(&self, semantics: CursorSemantics) -> usize{
        //assert!(self.cursor(semantics) <= text.len_chars());  //head? head may need text.len_chars() + 1 for block cursor
        //assert!(self.anchor <= text.len_chars());
        match semantics{
            CursorSemantics::Bar => self.head,
            CursorSemantics::Block => {
                if self.head >= self.anchor{
                    self.head.saturating_sub(1)   //prev_grapheme_boundary(text, self.head)
                }else{
                    self.head
                }
            }
        }
    }

    /// Returns a new instance of [`Selection`] with cursor at specified char index in rope.
    /// Will shift `anchor`/`head` positions to accommodate Bar/Block cursor semantics.
    /// 
    /// # Panics
    /// `put_cursor` panics if `to`  > `text.len_chars()`.
    /// ```should_panic
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Movement, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n"); //len 14
    /// Selection::new(0, 0).put_cursor(15, &text, Movement::Move, CursorSemantics::Block, true);
    /// ```
    /// 
    /// # Examples
    ///```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Movement, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::new(0, 0).put_cursor(5, &text, Movement::Move, CursorSemantics::Bar, true), Selection::with_stored_line_position(5, 5, 1));
    /// assert_eq!(Selection::new(5, 5).put_cursor(0, &text, Movement::Move, CursorSemantics::Bar, true), Selection::with_stored_line_position(0, 0, 0));
    /// 
    /// assert_eq!(Selection::new(0, 0).put_cursor(5, &text, Movement::Extend, CursorSemantics::Bar, true), Selection::with_stored_line_position(0, 5, 1));
    /// assert_eq!(Selection::new(5, 5).put_cursor(0, &text, Movement::Extend, CursorSemantics::Bar, true), Selection::with_stored_line_position(5, 0, 0));
    /// 
    /// assert_eq!(Selection::new(0, 1).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true), Selection::with_stored_line_position(5, 6, 1));
    /// assert_eq!(Selection::new(1, 0).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true), Selection::with_stored_line_position(5, 6, 1));
    /// assert_eq!(Selection::new(5, 6).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(6, 5).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true), Selection::with_stored_line_position(0, 1, 0));
    /// 
    /// assert_eq!(Selection::new(0, 1).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true), Selection::with_stored_line_position(0, 6, 1));
    /// assert_eq!(Selection::new(1, 0).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true), Selection::with_stored_line_position(0, 6, 1));
    /// assert_eq!(Selection::new(5, 6).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true), Selection::with_stored_line_position(6, 0, 0));
    /// assert_eq!(Selection::new(6, 5).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true), Selection::with_stored_line_position(6, 0, 0));
    /// 
    /// // test putting cursor at end of text
    /// assert_eq!(Selection::new(0, 0).put_cursor(14, &text, Movement::Move, CursorSemantics::Bar, true), Selection::with_stored_line_position(14, 14, 0));
    /// assert_eq!(Selection::new(0, 0).put_cursor(14, &text, Movement::Extend, CursorSemantics::Bar, true), Selection::with_stored_line_position(0, 14, 0));
    /// assert_eq!(Selection::new(0, 1).put_cursor(14, &text, Movement::Move, CursorSemantics::Block, true), Selection::with_stored_line_position(14, 15, 0));
    /// assert_eq!(Selection::new(0, 1).put_cursor(14, &text, Movement::Extend, CursorSemantics::Block, true), Selection::with_stored_line_position(0, 15, 0));
    /// ```
    #[must_use]
    pub fn put_cursor(&self, to: usize, text: &Rope, movement: Movement, semantics: CursorSemantics, update_stored_line_position: bool) -> Self{
        assert!(text.len_lines() > 0);
        assert!(to <= text.len_chars());
        assert!(self.cursor(semantics) <= text.len_chars());
        assert!(self.anchor <= text.len_chars());
        
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

        assert!(selection.anchor <= text.len_chars());
        assert!(selection.cursor(semantics) <= text.len_chars());

        selection
    }

    /////////////////////////////////// Movement Methods ///////////////////////////////////

    /// Returns a new instance of [`Selection`] with the cursor moved vertically by specified amount.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Movement, Direction, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse\n");
    /// 
    /// assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(4, 0, 0));
    /// 
    /// assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 0));
    /// assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(0, 5, 0));
    /// assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(5, 0, 0));
    /// 
    /// // handles moving/extending to text bounds correctly
    /// assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(19, 19, 0)); //idk\nsomething\nelse\n[]
    /// assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));    //[]idk\nsomething\nelse\n
    /// assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 19, 0));    //idk\nsomething\nelse\n[]
    /// assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(19, 0, 0)); //[]idk\nsomething\nelse\n
    /// 
    /// assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(19, 20, 0));   //idk\nsomething\nelse\n|: >    //is this the desired functionality?...
    /// assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(0, 20, 0));
    /// assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(19, 0, 0));
    /// ```
    #[must_use]
    pub fn move_vertically(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Self{
        assert!(text.len_lines() > 0);
        assert!(self.cursor(semantics) <= text.len_chars());
        assert!(self.anchor <= text.len_chars());
        assert!(amount > 0);
        
        let mut selection = self.clone();
        
        let current_line = text.char_to_line(self.cursor(semantics));
        let goal_line_number = match direction{
            Direction::Forward => (current_line + amount).min(text.len_lines().saturating_sub(1)),
            Direction::Backward => current_line.saturating_sub(amount),
        };

        let start_of_line = text.line_to_char(goal_line_number);
        //let line_width = text_util::line_width_excluding_newline(text.line(goal_line_number));
        let line_width = text_util::line_width(text.line(goal_line_number), false);
    
        // Use the stored line position or calculate it if None
        let stored_line_position = self.stored_line_position.unwrap_or_else(|| {
            text_util::offset_from_line_start(self.cursor(semantics), text)
        });

        // Calculate the new position based on line width
        let new_position = if stored_line_position < line_width {
            start_of_line + stored_line_position
        } else {
            start_of_line + line_width
        };

        selection.stored_line_position = Some(stored_line_position);
        selection.put_cursor(new_position, text, movement, semantics, false)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved horizontally by specified amount.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Movement, Direction, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse\n");    //len 19
    /// 
    /// assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(1, 1, 1));
    /// assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 1, 1));
    /// assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(1, 0, 0));
    /// 
    /// assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(1, 2, 1));
    /// assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(0, 2, 1));
    /// assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(2, 0, 0));
    /// 
    /// // handles moving/extending to text bounds correctly
    /// assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(19, 19, 0));
    /// assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar), Selection::with_stored_line_position(0, 19, 0));
    /// assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar), Selection::with_stored_line_position(19, 0, 0));
    /// 
    /// assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(19, 20, 0));
    /// assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block), Selection::with_stored_line_position(0, 20, 0));
    /// assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block), Selection::with_stored_line_position(19, 0, 0)); //:<idk\nsomething\nelse\n|
    /// ```
    #[must_use]
    pub fn move_horizontally(&self, amount: usize, text: &Rope, movement: Movement, direction: Direction, semantics: CursorSemantics) -> Self{
        assert!(text.len_lines() > 0);
        assert!(self.cursor(semantics) <= text.len_chars());
        assert!(self.anchor <= text.len_chars());
        
        let new_position = match direction{
            Direction::Forward => self.cursor(semantics).saturating_add(amount).min(text.len_chars()),    //ensures this does not move past text end
            Direction::Backward => self.cursor(semantics).saturating_sub(amount)
        };
        self.put_cursor(new_position, text, movement, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor set to specified 0-based line number.
    /// 
    /// # Panics
    /// `set_from_line_number` panics if `line_number` < `text.len_lines()`.
    /// ```should_panic
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Movement, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse\n");    //num lines 4
    /// Selection::new(0, 0).set_from_line_number(5, &text, Movement::Move, CursorSemantics::Bar);
    /// ```
    /// # Examples
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Movement, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse\n");
    /// 
    /// // normal use
    /// assert_eq!(Selection::new(0, 0).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Bar), Selection::with_stored_line_position(14, 14, 0));
    /// assert_eq!(Selection::new(0, 1).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Block), Selection::with_stored_line_position(14, 15, 0));
    /// assert_eq!(Selection::new(0, 0).set_from_line_number(2, &text, Movement::Extend, CursorSemantics::Bar), Selection::with_stored_line_position(0, 14, 0));
    /// assert_eq!(Selection::new(0, 1).set_from_line_number(2, &text, Movement::Extend, CursorSemantics::Block), Selection::with_stored_line_position(0, 15, 0));
    /// 
    /// // restricts cursor to line end when stored_line_position > line width
    /// assert_eq!(Selection::new(13, 13).set_from_line_number(0, &text, Movement::Move, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 9));
    /// assert_eq!(Selection::new(13, 14).set_from_line_number(0, &text, Movement::Move, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 9));
    /// assert_eq!(Selection::new(13, 13).set_from_line_number(0, &text, Movement::Extend, CursorSemantics::Bar), Selection::with_stored_line_position(13, 3, 9));
    /// assert_eq!(Selection::new(13, 14).set_from_line_number(0, &text, Movement::Extend, CursorSemantics::Block), Selection::with_stored_line_position(13, 3, 9));    //if at end of line, sets anchor before newline char
    /// 
    /// //from end of text
    /// assert_eq!(Selection::new(19, 19).set_from_line_number(1, &text, Movement::Move, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// assert_eq!(Selection::new(19, 20).set_from_line_number(1, &text, Movement::Move, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 0));
    /// assert_eq!(Selection::new(19, 19).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Bar), Selection::with_stored_line_position(14, 14, 0));
    /// assert_eq!(Selection::new(19, 20).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Block), Selection::with_stored_line_position(14, 15, 0));
    /// ```
    #[must_use]
    pub fn set_from_line_number(&self, line_number: usize, text: &Rope, movement: Movement, semantics: CursorSemantics) -> Self{
        assert!(text.len_lines() > 0);
        assert!(self.cursor(semantics) <= text.len_chars());
        assert!(self.anchor <= text.len_chars());
        assert!(line_number < text.len_lines());
        // deprecate in favor of assert? //let line_number = line_number.min(text.len_lines().saturating_sub(1));  //restrict line_number to doc length(-1 because len_lines is 1 based)
        let current_line = text.char_to_line(self.cursor(semantics));
        
        let (amount, direction) = if line_number < current_line{
            (current_line.saturating_sub(line_number), Direction::Backward)
        }else{
            (line_number.saturating_sub(current_line), Direction::Forward)
        };
    
        self.move_vertically(amount, text, movement, direction, semantics)
    }

    /// Returns a new instance of [`Selection`] with `anchor` aligned with cursor.
    /// ``` 
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // head < anchor
    /// assert_eq!(Selection::new(4, 0).collapse(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));  //<idk\n|some\nshit\n   //<|idk\nsome\nshit\n
    /// assert_eq!(Selection::new(4, 0).collapse(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));    //:<idk\n|some\nshit\n  //|:i>dk\nsome\nshit\n
    /// 
    /// // anchor < head
    /// assert_eq!(Selection::new(0, 4).collapse(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));  //|idk\n>some\nshit\n   //idk\n|>some\nshit\n
    /// assert_eq!(Selection::new(0, 4).collapse(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));    //|idk\n>some\nshit\n   //idk|:\n>some\nshit\n
    /// 
    /// // test setting cursor to end of text
    /// assert_eq!(Selection::new(0, 14).collapse(&text, CursorSemantics::Bar), Selection::with_stored_line_position(14, 14, 0));   //|idk\nsome\nshit\n>   //idk\nsome\nshit\n|>
    /// assert_eq!(Selection::new(0, 14).collapse(&text, CursorSemantics::Block), Selection::with_stored_line_position(13, 14, 4)); //|idk\nsome\nshit:\n>  //idk\nsome\nshit|:\n>
    /// ```
    #[must_use]
    pub fn collapse(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.put_cursor(self.cursor(semantics), text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved right.
    /// ``` 
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::new(14, 14).move_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(14, 14, 0));
    /// assert_eq!(Selection::new(14, 15).move_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(14, 15, 0));
    /// 
    /// // normal use
    /// assert_eq!(Selection::new(0, 0).move_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(1, 1, 1));
    /// assert_eq!(Selection::new(0, 1).move_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(1, 2, 1));
    /// 
    /// // new line resets stored line position
    /// assert_eq!(Selection::new(3, 3).move_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// assert_eq!(Selection::new(3, 4).move_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 0));
    /// 
    /// // with selection extended, collapses selection, then performs move
    /// assert_eq!(Selection::new(0, 3).move_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// assert_eq!(Selection::new(3, 0).move_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(1, 1, 1));
    /// assert_eq!(Selection::new(0, 3).move_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));
    /// assert_eq!(Selection::new(3, 0).move_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(1, 2, 1));
    /// ```
    #[must_use]
    pub fn move_right(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_horizontally(1, text, Movement::Move, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved left.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse\n");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::new(0, 0).move_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 1).move_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// 
    /// // normal use
    /// assert_eq!(Selection::new(1, 1).move_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(1, 2).move_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// 
    /// // move to previous line resets stored line position
    /// assert_eq!(Selection::new(4, 4).move_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 3));
    /// assert_eq!(Selection::new(4, 5).move_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));
    /// 
    /// // with selection extended, collapses selection, then performs move
    /// assert_eq!(Selection::new(1, 4).move_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 3));
    /// assert_eq!(Selection::new(4, 1).move_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(1, 4).move_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(2, 3, 2));   // i[d k:\n]s o m e t h i n g \n e l s e
    ///                                                                                                                             // i d[k]\n s o m e t h i n g \n e l s e
    /// assert_eq!(Selection::new(4, 1).move_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));   // i]d k \n[s o m e t h i n g \n e l s e
    ///                                                                                                                             //[i]d k \n s o m e t h i n g \n e l s e
    /// ```
    #[must_use]
    pub fn move_left(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_horizontally(1, text, Movement::Move, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved up.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::new(0, 0).move_up(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 1).move_up(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// 
    /// // to shorter line
    /// assert_eq!(Selection::new(13, 13).move_up(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 9));
    /// assert_eq!(Selection::new(13, 14).move_up(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 9));
    /// 
    /// // to longer line
    /// assert_eq!(Selection::new(18, 18).move_up(&text, CursorSemantics::Bar), Selection::with_stored_line_position(8, 8, 4));
    /// assert_eq!(Selection::new(18, 19).move_up(&text, CursorSemantics::Block), Selection::with_stored_line_position(8, 9, 4));
    /// 
    /// // with selection extended, collapses selection, then performs move
    /// assert_eq!(Selection::new(14, 14).move_up(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// assert_eq!(Selection::new(14, 4).move_up(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(4, 14).move_up(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 9));
    /// assert_eq!(Selection::new(14, 4).move_up(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// ```
    #[must_use]
    pub fn move_up(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_vertically(1, text, Movement::Move, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved down.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::new(18, 18).move_down(&text, CursorSemantics::Bar), Selection::with_stored_line_position(18, 18, 4));
    /// assert_eq!(Selection::new(18, 19).move_down(&text, CursorSemantics::Block), Selection::with_stored_line_position(18, 19, 4));
    /// 
    /// // to longer line
    /// assert_eq!(Selection::new(3, 3).move_down(&text, CursorSemantics::Bar), Selection::with_stored_line_position(7, 7, 3));
    /// assert_eq!(Selection::new(3, 4).move_down(&text, CursorSemantics::Block), Selection::with_stored_line_position(7, 8, 3));
    /// 
    /// // to shorter line
    /// assert_eq!(Selection::new(13, 13).move_down(&text, CursorSemantics::Bar), Selection::with_stored_line_position(18, 18, 9));
    /// assert_eq!(Selection::new(13, 14).move_down(&text, CursorSemantics::Block), Selection::with_stored_line_position(18, 19, 9));
    /// 
    /// // with selection extended, collapses selection, then performs move
    /// assert_eq!(Selection::new(0, 4).move_down(&text, CursorSemantics::Bar), Selection::with_stored_line_position(14, 14, 0));
    /// assert_eq!(Selection::new(4, 0).move_down(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// //[i d k \n]s o m e \n s h i t \n
    /// // i d k \n s o m[e]\n s h i t \n
    /// assert_eq!(Selection::new(0, 4).move_down(&text, CursorSemantics::Block), Selection::with_stored_line_position(7, 8, 3));
    /// assert_eq!(Selection::new(4, 0).move_down(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 0));
    /// ```
    #[must_use]
    pub fn move_down(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_vertically(1, text, Movement::Move, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with cursor moved to line end.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse\n");
    /// 
    /// assert_eq!(Selection::new(0, 0).move_line_text_end(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 3));
    /// assert_eq!(Selection::new(0, 1).move_line_text_end(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));
    /// assert_eq!(Selection::new(3, 4).move_line_text_end(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));  //verify repeated calls result in appropriate behavior
    /// 
    /// // with selection extended, collapse and move
    /// assert_eq!(Selection::new(0, 2).move_line_text_end(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 3));
    /// assert_eq!(Selection::new(2, 0).move_line_text_end(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 3, 3));
    /// assert_eq!(Selection::new(0, 2).move_line_text_end(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));
    /// assert_eq!(Selection::new(2, 0).move_line_text_end(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 4, 3));
    /// ```
    #[must_use]
    pub fn move_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line = text.line(line_number);
        //let line_width = text_util::line_width_excluding_newline(line);
        let line_width = text_util::line_width(line, false);
        let line_start = text.line_to_char(line_number);
        let line_end = line_start.saturating_add(line_width);

        self.put_cursor(line_end, text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with cursor moved to absolute start of line, or start of line text, depending on current cursor position.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("    idk\n");
    /// 
    /// // moves to text start when cursor past text start
    /// assert_eq!(Selection::new(6, 6).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 4));
    /// assert_eq!(Selection::new(6, 7).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 4));
    /// 
    /// // moves to line start when cursor at text start
    /// assert_eq!(Selection::new(4, 4).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(4, 5).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// 
    /// // moves to text start when cursor before text start
    /// assert_eq!(Selection::new(1, 1).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 4));
    /// assert_eq!(Selection::new(1, 2).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 4));
    /// 
    /// // with selection extended, collapse and move
    /// assert_eq!(Selection::new(0, 5).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 4));
    /// assert_eq!(Selection::new(0, 3).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 4));
    /// assert_eq!(Selection::new(0, 4).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(5, 0).move_home(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 4));
    /// assert_eq!(Selection::new(0, 6).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 4));
    /// assert_eq!(Selection::new(0, 4).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 4));
    /// assert_eq!(Selection::new(0, 5).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// assert_eq!(Selection::new(5, 0).move_home(&text, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 4));
    /// ```
    #[must_use]
    pub fn move_home(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        if self.cursor(semantics) == text_start{
            self.move_line_start(text, semantics)
        }else{
            self.move_line_text_start(text, semantics)
        }
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the current line.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::new(3, 3).move_line_start(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(3, 4).move_line_start(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// ```
    #[must_use]
    pub fn move_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);

        self.put_cursor(line_start, text, Movement::Move, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the text on the current line.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("  idk\n");
    /// 
    /// assert_eq!(Selection::new(0, 0).move_line_text_start(&text, CursorSemantics::Bar), Selection::with_stored_line_position(2, 2, 2));
    /// assert_eq!(Selection::new(0, 1).move_line_text_start(&text, CursorSemantics::Block), Selection::with_stored_line_position(2, 3, 2));
    /// ```
    #[must_use]
    pub fn move_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        self.put_cursor(text_start, text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved up by the height of `client_view`.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// let client_view = View::new(0, 0, 2, 2);
    /// assert_eq!(Selection::new(6, 6).move_page_up(&text, &client_view, CursorSemantics::Bar), Selection::with_stored_line_position(2, 2, 2));
    /// assert_eq!(Selection::new(6, 7).move_page_up(&text, &client_view, CursorSemantics::Block), Selection::with_stored_line_position(2, 3, 2));
    /// ```
    #[must_use]
    pub fn move_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Self{
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved down by the height of `client_view`.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// let client_view = View::new(0, 0, 2, 2);
    /// assert_eq!(Selection::new(0, 0).move_page_down(&text, &client_view, CursorSemantics::Bar), Selection::with_stored_line_position(4, 4, 0));
    /// assert_eq!(Selection::new(0, 1).move_page_down(&text, &client_view, CursorSemantics::Block), Selection::with_stored_line_position(4, 5, 0));
    /// ```
    #[must_use]
    pub fn move_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Self{
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Move, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the start of the document.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\n");
    /// assert_eq!(Selection::new(4, 4).move_doc_start(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(4, 5).move_doc_start(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// ```
    #[must_use]
    pub fn move_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.put_cursor(0, text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the cursor moved to the end of the document.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit");
    /// assert_eq!(Selection::new(0, 0).move_doc_end(&text, CursorSemantics::Bar), Selection::with_stored_line_position(13, 13, 4));
    /// assert_eq!(Selection::new(0, 1).move_doc_end(&text, CursorSemantics::Block), Selection::with_stored_line_position(13, 14, 4));
    /// ```
    #[must_use]
    pub fn move_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.put_cursor(text.len_chars(), text, Movement::Move, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the right.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // stays within bounds
    /// assert_eq!(Selection::new(14, 14).extend_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(14, 14, 0));
    /// assert_eq!(Selection::new(14, 15).extend_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(14, 15, 0));
    /// 
    /// // normal use
    /// assert_eq!(Selection::new(0, 0).extend_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 1, 1));
    /// assert_eq!(Selection::new(0, 1).extend_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 2, 1));
    /// 
    /// // resets stored line position after new line
    /// assert_eq!(Selection::new(3, 3).extend_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 4, 0));
    /// assert_eq!(Selection::new(3, 4).extend_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 5, 0));
    /// 
    /// // previously extended
    /// assert_eq!(Selection::new(0, 3).extend_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 4, 0));
    /// assert_eq!(Selection::new(3, 0).extend_right(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 1, 1));
    /// assert_eq!(Selection::new(0, 3).extend_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 4, 3));
    /// assert_eq!(Selection::new(3, 0).extend_right(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 1, 1));
    /// ```
    #[must_use]
    pub fn extend_right(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_horizontally(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the left.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::new(0, 0).extend_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
    /// assert_eq!(Selection::new(0, 1).extend_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
    /// 
    /// // normal use
    /// assert_eq!(Selection::new(2, 2).extend_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(2, 1, 1));
    /// assert_eq!(Selection::new(2, 3).extend_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 1, 1)); //id[:k]\nsomething\nelse   //i:]dk[\nsomething\nelse
    /// 
    /// //updates stored line position on line change
    /// assert_eq!(Selection::new(4, 4).extend_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(4, 3, 3));
    /// assert_eq!(Selection::new(4, 5).extend_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(5, 3, 3)); //idk\n[s]omething\nelse    //idk:]\ns[omething\nelse
    /// 
    /// //previously extended
    /// assert_eq!(Selection::new(0, 3).extend_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 2, 2));
    /// assert_eq!(Selection::new(3, 1).extend_left(&text, CursorSemantics::Bar), Selection::with_stored_line_position(3, 0, 0));
    /// assert_eq!(Selection::new(0, 3).extend_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(0, 2, 1)); //[id:k]\nsomething\nelse   //[i:d]k\nsomething\nelse
    /// assert_eq!(Selection::new(3, 1).extend_left(&text, CursorSemantics::Block), Selection::with_stored_line_position(3, 0, 0)); //i:]dk[\nsomething\nelse   //:]idk[\nsomething\nelse
    /// ```
    #[must_use]
    pub fn extend_left(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_horizontally(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::with_stored_line_position(0, 0, 0), Selection::new(0, 0).extend_up(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(0, 1, 0), Selection::new(0, 1).extend_up(&text, CursorSemantics::Block));
    /// 
    /// // to shorter line
    /// assert_eq!(Selection::with_stored_line_position(13, 3, 9), Selection::new(13, 13).extend_up(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(13, 3, 9), Selection::new(13, 14).extend_up(&text, CursorSemantics::Block)); //if at end of line, sets anchor before newline char
    /// 
    /// // to longer line
    /// assert_eq!(Selection::with_stored_line_position(18, 8, 4), Selection::new(18, 18).extend_up(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(18, 8, 4), Selection::new(18, 19).extend_up(&text, CursorSemantics::Block)); //idk\nsomething\nelse[: ]   //idk\nsome:]thing\nelse[
    /// ```
    #[must_use]
    pub fn extend_up(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_vertically(1, text, Movement::Extend, Direction::Backward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// 
    /// // stays within doc bounds
    /// assert_eq!(Selection::with_stored_line_position(18, 18, 4), Selection::new(18, 18).extend_down(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(18, 19, 4), Selection::new(18, 19).extend_down(&text, CursorSemantics::Block));
    /// 
    /// // to shorter line
    /// assert_eq!(Selection::with_stored_line_position(13, 18, 9), Selection::new(13, 13).extend_down(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(13, 19, 9), Selection::new(13, 14).extend_down(&text, CursorSemantics::Block)); //idk\nsomething[:\n]else    //idk\nsomething[\nelse: ]
    /// 
    /// // to longer line
    /// assert_eq!(Selection::with_stored_line_position(3, 7, 3), Selection::new(3, 3).extend_down(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(3, 8, 3), Selection::new(3, 4).extend_down(&text, CursorSemantics::Block)); //idk[:\n]something\nelse    //idk[\nsom:e]thing\nelse
    /// ```
    #[must_use]
    pub fn extend_down(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.move_vertically(1, text, Movement::Extend, Direction::Forward, semantics)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the end of the current line.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\n");
    /// 
    /// assert_eq!(Selection::with_stored_line_position(0, 3, 3), Selection::new(0, 0).extend_line_text_end(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_end(&text, CursorSemantics::Block));
    /// ```
    #[must_use]
    pub fn extend_line_text_end(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.head);
        let line = text.line(line_number);
        //let line_width = text_util::line_width_excluding_newline(line);
        let line_width = text_util::line_width(line, false);
        let line_start = text.line_to_char(line_number);
        let line_end = match semantics{
            CursorSemantics::Bar => line_start.saturating_add(line_width),
            CursorSemantics::Block => line_start.saturating_add(line_width).saturating_sub(1)
        };

        self.put_cursor(line_end, text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to absolute start of line, or line text start, depending on [`Selection`] `head` position.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("    idk\n");
    /// 
    /// // extends selection to text start when head past text start
    /// assert_eq!(Selection::with_stored_line_position(6, 4, 4), Selection::new(6, 6).extend_home(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(7, 4, 4), Selection::new(6, 7).extend_home(&text, CursorSemantics::Block));
    /// 
    /// // extends selection to line start when head at text start
    /// assert_eq!(Selection::with_stored_line_position(4, 0, 0), Selection::new(4, 4).extend_home(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(5, 0, 0), Selection::new(4, 5).extend_home(&text, CursorSemantics::Block));   //    [:i]dk\n  //:]    i[dk\n
    /// 
    /// // extends selection to text start when head before text start
    /// assert_eq!(Selection::with_stored_line_position(1, 4, 4), Selection::new(1, 1).extend_home(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(1, 5, 4), Selection::new(1, 2).extend_home(&text, CursorSemantics::Block)); // [: ]  idk\n  // [   :i]dk\n
    /// ```
    #[must_use]
    pub fn extend_home(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        if self.cursor(semantics) == text_start{
            self.extend_line_start(text, semantics)
        }else{
            self.extend_line_text_start(text, semantics)
        }
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the current line.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 3).extend_line_start(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 4).extend_line_start(&text, CursorSemantics::Block));   //special case  //if at end of line, sets anchor before newline char
    /// ```
    #[must_use]
    pub fn extend_line_start(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);

        self.put_cursor(line_start, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the text on the current line.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("  idk\n");
    /// 
    /// assert_eq!(Selection::with_stored_line_position(0, 2, 2), Selection::new(0, 0).extend_line_text_start(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_start(&text, CursorSemantics::Block));
    /// ```
    #[must_use]
    pub fn extend_line_text_start(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let line_number = text.char_to_line(self.cursor(semantics));
        let line_start = text.line_to_char(line_number);
        let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
        let text_start = line_start.saturating_add(text_start_offset);

        self.put_cursor(text_start, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended up by the height of `client_view`.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// let client_view = View::new(0, 0, 2, 2);
    /// 
    /// assert_eq!(Selection::with_stored_line_position(6, 2, 2), Selection::new(6, 6).extend_page_up(&text, &client_view, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(7, 2, 2), Selection::new(6, 7).extend_page_up(&text, &client_view, CursorSemantics::Block));    //idk\nso[m]ething\nelse    //id:]k\nsom[ething\nelse
    /// ```
    #[must_use]
    pub fn extend_page_up(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Self{
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Backward, semantics)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended down by the height of `client_view`.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// # use edit_core::view::View;
    /// 
    /// let text = Rope::from("idk\nsomething\nelse");
    /// let client_view = View::new(0, 0, 2, 2);
    /// 
    /// assert_eq!(Selection::with_stored_line_position(0, 4, 0), Selection::new(0, 0).extend_page_down(&text, &client_view, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(0, 5, 0), Selection::new(0, 1).extend_page_down(&text, &client_view, CursorSemantics::Block));  //[i]dk\nsomething\nelse    //[idk\n:s]omething\nelse
    /// ```
    #[must_use]
    pub fn extend_page_down(&self, text: &Rope, client_view: &View, semantics: CursorSemantics) -> Self{
        self.move_vertically(client_view.height().saturating_sub(1), text, Movement::Extend, Direction::Forward, semantics)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc start.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::with_stored_line_position(9, 0, 0), Selection::new(9, 9).extend_doc_start(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(10, 0, 0), Selection::new(9, 10).extend_doc_start(&text, CursorSemantics::Block));  //idk\nsome\n[s]hit\n   //:]idk\nsome\ns[hit\n
    /// ```
    #[must_use]
    pub fn extend_doc_start(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.put_cursor(0, text, Movement::Extend, semantics, true)
    }
    
    /// Returns a new instance of [`Selection`] with the [`Selection`] extended to doc end.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).extend_doc_end(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(0, 15, 0), Selection::new(0, 1).extend_doc_end(&text, CursorSemantics::Block));
    /// ```
    #[must_use]
    pub fn extend_doc_end(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        self.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    }

    /// Returns a new instance of [`Selection`] with [`Selection`] extended to encompass all text.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, CursorSemantics};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).select_all(&text, CursorSemantics::Bar));
    /// assert_eq!(Selection::with_stored_line_position(0, 15, 0), Selection::new(0, 1).select_all(&text, CursorSemantics::Block));
    /// ```
    #[must_use]
    pub fn select_all(&self, text: &Rope, semantics: CursorSemantics) -> Self{
        let selection = self.put_cursor(0, text, Movement::Move, semantics, true);
        selection.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    }

    /// Translates a [`Selection`] to a [Selection2d].
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selection2d, CursorSemantics};
    /// # use edit_core::Position;
    /// 
    /// let text = Rope::from("idk\nsomething");
    /// 
    /// // when selection head/anchor same, and on same line
    /// //id[]k
    /// //something
    /// assert_eq!(Selection::new(2, 2).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(2, 0), Position::new(2, 0))); //id[]k\nsomething
    /// assert_eq!(Selection::new(2, 3).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(2, 0), Position::new(2, 0)));
    /// 
    /// // when selection head/anchor different, but on same line
    /// //i[d]k
    /// //something
    /// assert_eq!(Selection::new(1, 2).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(1, 0), Position::new(2, 0))); //i[d]k\nsomething
    /// assert_eq!(Selection::new(1, 3).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(1, 0), Position::new(2, 0)));
    /// 
    /// // when selection head/anchor same, but on new line
    /// //idk
    /// //[]something
    /// assert_eq!(Selection::new(4, 4).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(0, 1), Position::new(0, 1))); //idk\n[]something
    /// assert_eq!(Selection::new(4, 5).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(0, 1), Position::new(0, 1)));
    /// 
    /// // when selection head/anchor different, and on different lines
    /// //id[k
    /// //s]omething
    /// assert_eq!(Selection::new(2, 5).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(2, 0), Position::new(1, 1))); //id[k\ns]omething
    /// assert_eq!(Selection::new(2, 6).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(2, 0), Position::new(1, 1)));
    /// ```
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
    /// ```should_panic
    /// # use ropey::Rope;
    /// # use edit_core::selection::Selections;
    /// 
    /// # let text = Rope::from("idk\nsome\nshit\n");
    /// Selections::new(vec![], 0, &text);  //panics
    /// ```
    /// 
    /// # Example
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// // sorts and merges overlapping
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut selections = Selections::new(vec![
    ///     Selection::new(2, 4),    // i d[k \n]s o m e \n s h i t \n
    ///     Selection::new(0, 5),    //[i d k \n s]o m e \n s h i t \n
    ///     Selection::new(3, 6)     // i d k[\n s o]m e \n s h i t \n
    /// ], 0, &text);
    /// let expected_selections = Selections::new(vec![
    ///     Selection::with_stored_line_position(0, 6, 2)     //[i d k \n s o]m e \n s h i t \n
    /// ], 0, &text);
    /// assert_eq!(expected_selections, selections);
    /// ```
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
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0)], 0, &text), selections.pop());
    /// 
    /// // always contains at least one selection
    /// let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0)], 0, &text), selections.pop());
    /// ```
    pub fn pop(&self) -> Self{
        let mut new_selections = self.selections.clone();
        if new_selections.len() > 1{    // Guarantee at least one selection
            new_selections.pop();
        }else{
            return self.clone();
        }

        // Is there a better way to determine new primary selection?
        let primary_selection_index = new_selections.len().saturating_sub(1);

        Self{
            selections: new_selections,
            primary_selection_index
        }
    }

    /// Prepends a [`Selection`] to the front of [Self], with `primary_selection_index` set to 0.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text), selections.push_front(Selection::new(0, 0)));
    /// ```
    // TODO: this function should take an input bool update_primary, that indicates whether the pushed selection should become the primary
    pub fn push_front(&self, selection: Selection) -> Self{
        let mut new_selections = self.selections.clone();
        new_selections.insert(0, selection);
        Self{
            selections: new_selections,
            primary_selection_index: 0
        }
    }
    
    /// Appends a [`Selection`] to the back of [Self], with `primary_selection_index` set to num of selections - 1.
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut selections = Selections::new(vec![Selection::new(0, 0)], 0, &text); //[]idk\nsome\nshit\n
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push(Selection::new(4, 4)));
    /// ```
    // TODO: this function should take an input bool update_primary, that indicates whether the pushed selection should become the primary
    pub fn push(&self, selection: Selection) -> Self{
        let mut new_selections = self.selections.clone();
        new_selections.push(selection);
        let primary_selection_index = new_selections.len().saturating_sub(1);
        Self{
            selections: new_selections,
            primary_selection_index
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
    /// 
    /// # Panics
    /// `increment_primary_selection` panics if [`Selections`] contains only 1 [`Selection`].
    /// ```should_panic
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// # let text = Rope::from("idk\nsome\nshit\n");
    /// Selections::new(vec![Selection::new(0, 0)], 0, &text).increment_primary_selection();
    /// ```
    /// # Examples
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // increments
    /// let mut selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text), selections.increment_primary_selection());
    /// 
    /// // wraps on last selection
    /// let mut selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text), selections.increment_primary_selection());
    /// ```
    #[must_use]
    pub fn increment_primary_selection(&self) -> Self{
        assert!(self.count() > 1);  // multiple selections required to increment
        if self.primary_selection_index.saturating_add(1) < self.count(){
            Self{selections: self.selections.clone(), primary_selection_index: self.primary_selection_index + 1}
        }else{
            Self{selections: self.selections.clone(), primary_selection_index: 0}
        }
    }
    /// Decrements the primary selection index.
    /// 
    /// # Panics
    /// `decrement_primary_selection` panics if [`Selections`] contains only 1 [`Selection`].
    /// ```should_panic
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// # let text = Rope::from("idk\nsome\nshit\n");
    /// Selections::new(vec![Selection::new(0, 0)], 0, &text).decrement_primary_selection();
    /// ```
    /// # Examples
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // decrements
    /// let mut selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text), selections.decrement_primary_selection());
    /// 
    /// // wraps on first selection
    /// let mut selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text), selections.decrement_primary_selection());
    /// ```
    #[must_use]
    pub fn decrement_primary_selection(&self) -> Self{
        assert!(self.count() > 1);  // multiple selections required to decrement
        if self.primary_selection_index > 0{
            Self{selections: self.selections.clone(), primary_selection_index: self.primary_selection_index - 1}
        }else{
            Self{selections: self.selections.clone(), primary_selection_index: self.count().saturating_sub(1)}
        }
    }

    /// Sorts each [`Selection`] in [Selections] by position.
    /// #### Invariants:
    /// - preserves primary selection through the sorting process
    /// 
    /// # Example
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut selections = Selections::new(vec![
    ///     Selection::new(2, 4),
    ///     Selection::new(0, 5),
    ///     Selection::new(3, 6)
    /// ], 0, &text);
    /// let expected_selections = Selections::new(vec![
    ///     Selection::new(0, 5),
    ///     Selection::new(2, 4),
    ///     Selection::new(3, 6)
    /// ], 1, &text);
    /// assert_eq!(expected_selections, selections.sort());
    /// ```
    #[must_use]
    pub fn sort(&self) -> Self{
        if self.count() < 2{
            return self.clone();
        }

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
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// let mut selections = Selections::new(vec![
    ///     Selection::new(0, 2),    //[i d]k \n s o m e \n s h i t \n
    ///     Selection::new(1, 4),    // i[d k \n]s o m e \n s h i t \n
    ///     Selection::new(5, 7),    // i d k \n s[o m]e \n s h i t \n
    ///     Selection::new(8, 10),   // i d k \n s o m e[\n s]h i t \n
    ///     Selection::new(9, 12)    // i d k \n s o m e \n[s h i]t \n
    /// ], 4, &text);
    /// let expected_selections = Selections::new(vec![
    ///     Selection::with_stored_line_position(0, 4, 0),    //[i d k \n]s o m e \n s h i t \n
    ///     Selection::new(5, 7),    // i d k \n s[o m]e \n s h i t \n
    ///     Selection::with_stored_line_position(8, 12, 3)    // i d k \n s o m e[\n s h i]t \n
    /// ], 2, &text);
    /// assert_eq!(expected_selections, selections.merge_overlapping(&text));
    /// ```
    pub fn merge_overlapping(&mut self, text: &Rope) -> Self{
        if self.count() < 2{
            return self.clone();
        }

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
    /// 
    /// # Panics
    /// `clear_non_primary_selections` panics if [`Selections`] has only 1 [`Selection`].
    /// ```should_panic
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// // should panic
    /// # let text = Rope::from("idk\nsome\nshit\n");
    /// Selections::new(vec![Selection::new(0, 0)], 0, &text).clear_non_primary_selections();
    /// ```
    /// # Example
    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// 
    /// // normal use
    /// let mut selections = Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text);
    /// assert_eq!(Selections::new(vec![Selection::new(4, 4)], 0, &text), selections.clear_non_primary_selections());
    /// ```
    pub fn clear_non_primary_selections(&self) -> Self{
        assert!(self.count() > 1);
        
        let cleared_selections = vec![self.primary().clone()];
        
        assert!(cleared_selections.len() == 1);
        
        Self{
            selections: cleared_selections,
            primary_selection_index: 0
        }
    }

    //TODO: return head and anchor positions
    //TODO: return Vec<Position> document cursor positions
    pub fn cursor_positions(&self, text: &Rope, semantics: CursorSemantics) -> Position{
        let cursor = self.primary();
        let document_cursor = cursor.selection_to_selection2d(text, semantics);
        
        Position::new(
            document_cursor.head().x().saturating_add(1), 
            document_cursor.head().y().saturating_add(1)
        )
    }

    /// ```
    /// # use ropey::Rope;
    /// # use edit_core::selection::{Selection, Selections};
    /// 
    /// // intended use
    /// let text = Rope::from("idk\nsome\nshit\n");
    /// let mut selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    /// assert_eq!(
    ///     Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text)),
    ///     selections.add_selection_above(&text)
    /// );
    /// 
    /// let mut selections = Selections::new(vec![Selection::new(5, 7)], 0, &text);
    /// assert_eq!(
    ///     Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 0, &text)),
    ///     selections.add_selection_above(&text)
    /// );
    /// 
    /// // should error when top selection is on line 0
    /// let mut selections = Selections::new(vec![Selection::new(1, 3)], 0, &text);
    /// assert_eq!(Err(()), selections.add_selection_above(&text));
    /// 
    /// // should error when any selection is a multi-line selection
    /// let mut selections = Selections::new(vec![Selection::new(4, 9)], 0, &text);
    /// assert_eq!(Err(()), selections.add_selection_above(&text));
    /// ```
    // should this use start() and end() instead of head and anchor?
    pub fn add_selection_above(&self, text: &Rope) -> Result<Self, ()>{
        assert!(self.count() > 0);  //ensure at least one selection in selections
        // should fail if any selection spans multiple lines. // should this be changed to allow this in the future?
        for selection in self.selections.iter(){
            if text.char_to_line(selection.anchor) != text.char_to_line(selection.head){
                return Err(()); //cannot add selection above
            }
        }
        let top_selection = self.first();
        let current_line = text.char_to_line(top_selection.anchor);
        if current_line == 0{
            return Err(());
        }
        let anchor_offset = text_util::offset_from_line_start(top_selection.anchor, text);
        let head_offset = text_util::offset_from_line_start(top_selection.head, text);
        let line_above = current_line.saturating_sub(1);
        let line_start = text.line_to_char(line_above);
        let line_width = text_util::line_width(text.line(line_above), true);
        Ok(self.push_front(Selection::new(line_start + anchor_offset, line_start + head_offset.min(line_width))))
    }
}
