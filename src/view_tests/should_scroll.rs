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
}
