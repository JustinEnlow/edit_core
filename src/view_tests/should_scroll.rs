use ropey::Rope;
use crate::view::View;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn should_scroll(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    
    // in view
    let selection = Selection::new(0, 0);
    assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(0, 1);
    assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Block));
    
    // out of view horizontally
    let selection = Selection::new(3, 3);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(3, 4);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
    
    // out of view vertically
    let selection = Selection::new(10, 10);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(10, 11);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
}
