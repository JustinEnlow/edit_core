use ropey::Rope;
use crate::view::View;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

// TODO: make separate tests for a view with odd number of lines, and a view with even number of lines. ensure behavior is as expected
#[test]
fn center_vertically_around_cursor(){
    let text = Rope::from("idk\nsome\nshit\nand\nsomething\nelse\n");   //len 33
    //let view = View::new(0, 0, 1, 1);
    
    //works when past half the view in text
    //let selection = Selection::new(14, 14);
    //assert_eq!(View::new(0, 3, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).unwrap());
    //let selection = Selection::new(14, 15);
    //assert_eq!(View::new(0, 3, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).unwrap());
    
    // old tests
        //let selection = Selection::new(0, 0);
        //assert_eq!(view, view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
        //let selection = Selection::new(0, 1);
        //assert_eq!(view, view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));
        //
        //let selection = Selection::new(33, 33);
        //assert_eq!(View::new(0, 6, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
        //let selection = Selection::new(33, 34);
        //assert_eq!(View::new(0, 6, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));

    //errors near doc start
    //let selection = Selection::new(0, 0);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    //let selection = Selection::new(0, 1);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    //errors near doc end
    //let selection = Selection::new(33, 33);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    //let selection = Selection::new(33, 34);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    let view = View::new(0, 0, 3, 3);
    //|i d k|
    //|s o m|e
    //|s h i|t
    // a n d
    // s o m e t h i n g
    // e l s e

    // works when cursor past half view height and before doc end - half view height
    //let selection = Selection::new(9, 9);
    let selection = Selection::new(Range::new(9, 9), Direction::Forward);
    assert_eq!(View::new(0, 1, 3, 3), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).unwrap());
    //let selection = Selection::new(9, 10);
    let selection = Selection::new(Range::new(9, 10), Direction::Forward);
    assert_eq!(View::new(0, 1, 3, 3), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).unwrap());

    // errors when cursor before half view height
    //let selection = Selection::new(0, 0);
    let selection = Selection::new(Range::new(0, 0), Direction::Forward);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    //let selection = Selection::new(0, 1);
    let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    // errors when cursor after doc end - half view height
    //let selection = Selection::new(33, 33);
    let selection = Selection::new(Range::new(33, 33), Direction::Forward);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    //let selection = Selection::new(33, 34);
    let selection = Selection::new(Range::new(33, 34), Direction::Forward);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    // errors when cursor already centered
    //let selection = Selection::new(4, 4);
    let selection = Selection::new(Range::new(4, 4), Direction::Forward);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
}
