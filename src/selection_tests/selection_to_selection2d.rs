use ropey::Rope;
use crate::Position;
use crate::selection::{Selection, CursorSemantics};
use crate::selection2d::Selection2d;

#[test]
fn selection_to_selection2d(){
    //use edit_core::selection::Selection2d;
    //use edit_core::Position;

    let text = Rope::from("idk\nsomething");
    
    // when selection head/anchor same, and on same line
    //id[]k
    //something
    assert_eq!(Selection::new(2, 2).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(2, 0), Position::new(2, 0))); //id[]k\nsomething
    assert_eq!(Selection::new(2, 3).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(2, 0), Position::new(2, 0)));
    
    // when selection head/anchor different, but on same line
    //i[d]k
    //something
    assert_eq!(Selection::new(1, 2).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(1, 0), Position::new(2, 0))); //i[d]k\nsomething
    assert_eq!(Selection::new(1, 3).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(1, 0), Position::new(2, 0)));
    
    // when selection head/anchor same, but on new line
    //idk
    //[]something
    assert_eq!(Selection::new(4, 4).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(0, 1), Position::new(0, 1))); //idk\n[]something
    assert_eq!(Selection::new(4, 5).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(0, 1), Position::new(0, 1)));
    
    // when selection head/anchor different, and on different lines
    //id[k
    //s]omething
    assert_eq!(Selection::new(2, 5).selection_to_selection2d(&text, CursorSemantics::Bar), Selection2d::new(Position::new(2, 0), Position::new(1, 1))); //id[k\ns]omething
    assert_eq!(Selection::new(2, 6).selection_to_selection2d(&text, CursorSemantics::Block), Selection2d::new(Position::new(2, 0), Position::new(1, 1)));
}
