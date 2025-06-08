use ropey::Rope;
use crate::view::View;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test]
fn should_scroll(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    
    // in view
    let selection = Selection::new(Range::new(0, 0), Direction::Forward);
    assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Block));
    
    // out of view horizontally
    let selection = Selection::new(Range::new(3, 3), Direction::Forward);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(Range::new(3, 4), Direction::Forward);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
    
    // out of view vertically
    let selection = Selection::new(Range::new(10, 10), Direction::Forward);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(Range::new(10, 11), Direction::Forward);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));

    //i'm having the cursor disappear when at the end of a line longer than the view is wide. it happens inconsistently. not sure how to fix, but this seems to be working...
    //[i d k]\n
    //[s o m]e \n
    // s h i t \n
    // _
    let view = View::new(0, 0, 3, 2);
    let selection = Selection::new_from_range(Range::new(3, 4), Direction::Forward, &text, CursorSemantics::Block);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
    let selection = Selection::new_from_range(Range::new(8, 9), Direction::Forward, &text, CursorSemantics::Block);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
}
