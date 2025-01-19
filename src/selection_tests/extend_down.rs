use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_down(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // to shorter line
    assert_eq!(Selection::with_stored_line_position(13, 18, 9), Selection::new(13, 13).extend_down(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(13, 19, 9), Selection::new(13, 14).extend_down(&text, CursorSemantics::Block).unwrap()); //idk\nsomething[:\n]else    //idk\nsomething[\nelse: ]
    
    // to longer line
    assert_eq!(Selection::with_stored_line_position(3, 7, 3), Selection::new(3, 3).extend_down(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(3, 8, 3), Selection::new(3, 4).extend_down(&text, CursorSemantics::Block).unwrap()); //idk[:\n]something\nelse    //idk[\nsom:e]thing\nelse
}
#[test]
fn extend_down_errors_if_on_last_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(18, 18).extend_down(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(18, 19).extend_down(&text, CursorSemantics::Block).is_err());
}
