use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn merge(){
    let text = Rope::from("idk\nsome\nshit\n");

    // verify non extended selections
    assert_eq!(Selection::new(0, 0).merge(&Selection::new(0, 0), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 1).merge(&Selection::new(0, 1), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(1, 0).merge(&Selection::new(1, 0), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 0, 0));

    // errors when direction of first selection is different than direction of other selection
    assert!(Selection::new(0, 1).merge(&Selection::new(1, 0), &text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).merge(&Selection::new(1, 0), &text, CursorSemantics::Block).is_err());
    assert!(Selection::new(1, 0).merge(&Selection::new(0, 1), &text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(1, 0).merge(&Selection::new(0, 1), &text, CursorSemantics::Block).is_err());

    // when self.anchor > self.head && other.anchor > other.head
    assert_eq!(Selection::new(4, 0).merge(&Selection::new(5, 1), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    assert_eq!(Selection::new(4, 0).merge(&Selection::new(5, 1), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    assert_eq!(Selection::new(5, 1).merge(&Selection::new(4, 0), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    assert_eq!(Selection::new(5, 1).merge(&Selection::new(4, 0), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    
    // when self.anchor < self.head && other.anchor < other.head
    assert_eq!(Selection::new(0, 4).merge(&Selection::new(1, 5), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(0, 4).merge(&Selection::new(1, 5), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 5, 0));
    assert_eq!(Selection::new(1, 5).merge(&Selection::new(0, 4), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(1, 5).merge(&Selection::new(0, 4), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 5, 0));
}
                    
#[test]
fn consecutive(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::new(0, 1).merge(&Selection::new(1, 2), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(0, 1).merge(&Selection::new(1, 2), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));   //TODO: these using block semantics aren't really consecutive
    
    assert_eq!(Selection::new(1, 0).merge(&Selection::new(2, 1), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 0, 0));
    assert_eq!(Selection::new(1, 0).merge(&Selection::new(2, 1), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 0, 0));
    
    assert_eq!(Selection::new(1, 2).merge(&Selection::new(0, 1), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(1, 2).merge(&Selection::new(0, 1), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));
    
    assert_eq!(Selection::new(2, 1).merge(&Selection::new(1, 0), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 0, 0));
    assert_eq!(Selection::new(2, 1).merge(&Selection::new(1, 0), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 0, 0));
}
                    
#[test]
fn overlapping(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(1, 4), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(1, 4), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 4, 3));
    
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(4, 1), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 0, 0));
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(4, 1), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 0, 0));
    
    assert_eq!(Selection::new(1, 4).merge(&Selection::new(0, 2), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(1, 4).merge(&Selection::new(0, 2), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 4, 3));
    
    assert_eq!(Selection::new(4, 1).merge(&Selection::new(2, 0), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 0, 0));
    assert_eq!(Selection::new(4, 1).merge(&Selection::new(2, 0), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 0, 0));
}
                    
#[test]
fn contained(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::new(0, 6).merge(&Selection::new(2, 4), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(0, 6).merge(&Selection::new(2, 4), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    
    assert_eq!(Selection::new(6, 0).merge(&Selection::new(4, 2), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(6, 0).merge(&Selection::new(4, 2), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    
    assert_eq!(Selection::new(2, 4).merge(&Selection::new(0, 6), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(2, 4).merge(&Selection::new(0, 6), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    
    assert_eq!(Selection::new(4, 2).merge(&Selection::new(6, 0), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(4, 2).merge(&Selection::new(6, 0), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(6, 0, 0));
}
                    
#[test]
fn disconnected(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(4, 6), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(4, 6), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(6, 4), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(6, 4), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    
    assert_eq!(Selection::new(4, 6).merge(&Selection::new(0, 2), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(4, 6).merge(&Selection::new(0, 2), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    
    assert_eq!(Selection::new(6, 4).merge(&Selection::new(2, 0), &text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(6, 4).merge(&Selection::new(2, 0), &text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(6, 0, 0));
}
