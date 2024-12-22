use ropey::Rope;
use edit_core::selection::{Selection, CursorSemantics, Direction, Movement};

#[test]
fn start(){
    assert_eq!(0, Selection::new(0, 4).start());
    assert_eq!(0, Selection::new(4, 0).start());
}

#[test]
fn end(){
    assert_eq!(4, Selection::new(0, 4).end());
    assert_eq!(4, Selection::new(4, 0).end());
}

#[test]
fn is_extended_bar_semantics(){
    assert_eq!(Selection::new(0, 0).is_extended(CursorSemantics::Bar), false);
    assert_eq!(Selection::new(0, 1).is_extended(CursorSemantics::Bar), true);
    assert_eq!(Selection::new(1, 0).is_extended(CursorSemantics::Bar), true);
}

#[test]
fn is_extended_block_semantics(){
    assert_eq!(Selection::new(0, 1).is_extended(CursorSemantics::Block), false);
    assert_eq!(Selection::new(1, 0).is_extended(CursorSemantics::Block), false);
    assert_eq!(Selection::new(0, 2).is_extended(CursorSemantics::Block), true);
    assert_eq!(Selection::new(2, 0).is_extended(CursorSemantics::Block), true);
}

//#[test]
//fn direction_bar_semantics(){
//    assert_eq!(Selection::new(0, 0).direction(CursorSemantics::Bar), Direction::Forward);
//    assert_eq!(Selection::new(0, 1).direction(CursorSemantics::Bar), Direction::Forward);
//    assert_eq!(Selection::new(1, 0).direction(CursorSemantics::Bar), Direction::Backward);
//}
//
//#[test]
//fn direction_block_semantics(){
//    //assert_eq!(Selection::new(0, 0).direction(CursorSemantics::Block), Direction::Backward);    //state shouldn't be possible with block cursor semantics, so this failure is fine.
//    assert_eq!(Selection::new(0, 1).direction(CursorSemantics::Block), Direction::Forward);
//    assert_eq!(Selection::new(1, 0).direction(CursorSemantics::Block), Direction::Backward);
//    assert_eq!(Selection::new(1, 1).direction(CursorSemantics::Block), Direction::Backward); //state shouldn't be possible with block cursor semantics, but the result is still valid.
//}

//#[test]
//fn set_direction_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    assert_eq!(Selection::new(0, 0).set_direction(Direction::Forward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
//    assert_eq!(Selection::new(0, 0).set_direction(Direction::Backward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
//    assert_eq!(Selection::new(0, 5).set_direction(Direction::Backward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(5, 0, 0));
//    assert_eq!(Selection::new(5, 0).set_direction(Direction::Forward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 5, 1));
//}
//
//#[test]
//fn set_direction_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    assert_eq!(Selection::new(0, 1).set_direction(Direction::Backward, &text, CursorSemantics::Block), Selection::with_stored_line_position(1, 0, 0));
//    assert_eq!(Selection::new(1, 0).set_direction(Direction::Forward, &text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
//    assert_eq!(Selection::new(0, 5).set_direction(Direction::Backward, &text, CursorSemantics::Block), Selection::with_stored_line_position(5, 0, 0));
//    assert_eq!(Selection::new(5, 0).set_direction(Direction::Forward, &text, CursorSemantics::Block), Selection::with_stored_line_position(0, 5, 0));
//}

#[test]
fn overlaps_non_zero_width_selection(){
    //let text = Rope::from("idk\nsome\nshit\n");
    // non zero width selections, no overlap
    assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(3, 6)), false); //[idk]<\nso>me\nshit\n
    assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(6, 3)), false); //[idk]>\nso<me\nshit\n
    assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(3, 6)), false); //]idk[<\nso>me\nshit\n
    assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(6, 3)), false); //]idk[>\nso<me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(0, 3)), false); //<idk>[\nso]me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(3, 0)), false); //>idk<[\nso]me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(0, 3)), false); //<idk>]\nso[me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(3, 0)), false); //>idk<]\nso[me\nshit\n
    
    // non-zero-width selections, overlap.
    assert_eq!(Selection::new(0, 4).overlaps(&Selection::new(3, 6)), true);  //[idk<\n]so>me\nshit\n
    assert_eq!(Selection::new(0, 4).overlaps(&Selection::new(6, 3)), true);  //[idk>\n]so<me\nshit\n
    assert_eq!(Selection::new(4, 0).overlaps(&Selection::new(3, 6)), true);  //]idk<\n[so>me\nshit\n
    assert_eq!(Selection::new(4, 0).overlaps(&Selection::new(6, 3)), true);  //]idk>\n[so<me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(0, 4)), true);  //<idk[\n>so]me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(4, 0)), true);  //>idk[\n<so]me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(0, 4)), true);  //<idk]\n>so[me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(4, 0)), true);  //>idk]\n<so[me\nshit\n
}

#[test]
fn overlaps_zero_width_and_non_zero_width_selection(){
    //let text = Rope::from("idk\nsome\nshit\n");
    // Zero-width and non-zero-width selections, overlap.
    assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(3, 3)), true);  //[idk<>]\nsome\nshit\n
    assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(3, 3)), true);  //]idk<>[\nsome\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(0, 3)), true);  //<idk[]>\nsome\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(3, 0)), true);  //>idk[]<\nsome\nshit\n
    
    // Zero-width and non-zero-width selections, overlap.
    assert_eq!(Selection::new(1, 4).overlaps(&Selection::new(1, 1)), true);  //i[<>dk\n]some\nshit\n
    assert_eq!(Selection::new(4, 1).overlaps(&Selection::new(1, 1)), true);  //i]<>dk\n[some\nshit\n
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(1, 4)), true);  //i[<]dk\n>some\nshit\n
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(4, 1)), true);  //i[>]dk\n<some\nshit\n
    assert_eq!(Selection::new(1, 4).overlaps(&Selection::new(3, 3)), true);  //i[dk<>\n]some\nshit\n
    assert_eq!(Selection::new(4, 1).overlaps(&Selection::new(3, 3)), true);  //i]dk<>\n[some\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(1, 4)), true);  //i<dk[]\n>some\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(4, 1)), true);  //i>dk[]\n<some\nshit\n
}

#[test]
fn overlaps_zero_width_selection(){
    //let text = Rope::from("idk\nsome\nshit\n");
    // zero-width selections, no overlap.
    assert_eq!(Selection::new(0, 0).overlaps(&Selection::new(1, 1)), false); //[]i<>dk\nsome\nshit\n
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(0, 0)), false); //<>i[]dk\nsome\nshit\n
    
    // zero-width selections, overlap.
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(1, 1)), true);  //i[<>]dk\nsome\nshit\n
}

#[test]
fn contains(){
    assert!( Selection::new(0, 4).contains(3));
    assert!( Selection::new(4, 0).contains(3));
    assert!(!Selection::new(0, 4).contains(5));
    assert!(!Selection::new(4, 0).contains(5));
}

#[test]
fn intersection(){
    let first = Selection::new(0, 6);
    let second = Selection::new(3, 9);
    assert!(first.intersection(&second).is_ok());
    assert_eq!(Selection::new(3, 6), first.intersection(&second).unwrap());
    
    let first = Selection::new(1, 5);
    let second = Selection::new(2, 3);
    assert!(first.intersection(&second).is_ok());
    assert_eq!(Selection::new(2, 3), first.intersection(&second).unwrap());
}
#[test]
fn intersection_should_error_if_non_overlapping(){
    let first = Selection::new(0, 4);
    let second = Selection::new(5, 9);
    assert!(first.intersection(&second).is_err());
}

#[test]
fn merge(){
    let text = Rope::from("idk\nsome\nshit\n");
    // when self.anchor > self.head && other.anchor > other.head
    assert_eq!(Selection::new(4, 0).merge(&Selection::new(5, 1), &text), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(5, 1).merge(&Selection::new(4, 0), &text), Selection::with_stored_line_position(0, 5, 1));
    
    // when self.anchor < self.head && other.anchor < other.head
    assert_eq!(Selection::new(0, 4).merge(&Selection::new(1, 5), &text), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(1, 5).merge(&Selection::new(0, 4), &text), Selection::with_stored_line_position(0, 5, 1));
    
    // when self.anchor > self.head && other.anchor < other.head
    assert_eq!(Selection::new(4, 0).merge(&Selection::new(1, 5), &text), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(1, 5).merge(&Selection::new(4, 0), &text), Selection::with_stored_line_position(0, 5, 1));
    
    // when self.anchor < self.head && other.anchor > other.head
    assert_eq!(Selection::new(0, 4).merge(&Selection::new(5, 1), &text), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(5, 1).merge(&Selection::new(0, 4), &text), Selection::with_stored_line_position(0, 5, 1));
}

#[test]
fn merge_consecutive(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 1).merge(&Selection::new(1, 2), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(1, 0).merge(&Selection::new(1, 2), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(1, 0).merge(&Selection::new(2, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(0, 1).merge(&Selection::new(2, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(1, 2).merge(&Selection::new(0, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(2, 1).merge(&Selection::new(0, 1), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(2, 1).merge(&Selection::new(1, 0), &text), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(1, 2).merge(&Selection::new(1, 0), &text), Selection::with_stored_line_position(0, 2, 2));
}

#[test]
fn merge_overlapping(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(1, 4), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(1, 4), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(4, 1), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(4, 1), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(1, 4).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(4, 1).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(4, 1).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(1, 4).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 4, 0));
}

#[test]
fn merge_contained(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 6).merge(&Selection::new(2, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(6, 0).merge(&Selection::new(2, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(6, 0).merge(&Selection::new(4, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(0, 6).merge(&Selection::new(4, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(2, 4).merge(&Selection::new(0, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(4, 2).merge(&Selection::new(0, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(4, 2).merge(&Selection::new(6, 0), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(2, 4).merge(&Selection::new(6, 0), &text), Selection::with_stored_line_position(0, 6, 2));
}

#[test]
fn merge_disconnected(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(4, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(4, 6), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(2, 0).merge(&Selection::new(6, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(0, 2).merge(&Selection::new(6, 4), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(4, 6).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(6, 4).merge(&Selection::new(0, 2), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(6, 4).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 6, 2));
    assert_eq!(Selection::new(4, 6).merge(&Selection::new(2, 0), &text), Selection::with_stored_line_position(0, 6, 2));
}

#[test]
fn cursor(){
    //let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 0).cursor(CursorSemantics::Bar), 0);   //|>idk\nsome\nshit\n
    assert_eq!(Selection::new(1, 2).cursor(CursorSemantics::Block), 1); //i|:d>k\nsome\nshit\n
    assert_eq!(Selection::new(2, 1).cursor(CursorSemantics::Block), 1); //i:<d|k\nsome\nshit\n
    assert_eq!(Selection::new(2, 2).cursor(CursorSemantics::Block), 1); //i:d|>k\nsome\nshit\n  //though this state should be impossible with block cursor semantics
}

#[test]
fn put_cursor(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::new(0, 0).put_cursor(5, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(5, 5, 1));
    assert_eq!(Selection::new(5, 5).put_cursor(0, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    
    assert_eq!(Selection::new(0, 0).put_cursor(5, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(5, 5).put_cursor(0, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    
    assert_eq!(Selection::new(0, 1).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(5, 6, 1));
    assert_eq!(Selection::new(1, 0).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(5, 6, 1));
    assert_eq!(Selection::new(5, 6).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(6, 5).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    
    assert_eq!(Selection::new(0, 1).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    assert_eq!(Selection::new(1, 0).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    assert_eq!(Selection::new(5, 6).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(6, 5).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    
    // test putting cursor at end of text
    assert_eq!(Selection::new(0, 0).put_cursor(14, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(0, 0).put_cursor(14, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(0, 14, 0));
    assert_eq!(Selection::new(0, 1).put_cursor(14, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(14, 15, 0));
    assert_eq!(Selection::new(0, 1).put_cursor(14, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 15, 0));
}
#[test]
fn put_cursor_errors_if_to_out_of_text_bounds(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14
    assert!(Selection::new(0, 0).put_cursor(15, &text, Movement::Move, CursorSemantics::Bar, true).is_err());
    assert!(Selection::new(0, 1).put_cursor(15, &text, Movement::Move, CursorSemantics::Block, true).is_err());
    // TODO: test extend as well
}

#[test]
fn move_vertically(){
    let text = Rope::from("idk\nsomething\nelse\n");
    assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 0, 0));
    
    assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 5, 0));
    assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    
    // handles moving/extending to text bounds correctly
    assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 19, 0)); //idk\nsomething\nelse\n[]
    assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));    //[]idk\nsomething\nelse\n
    assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 19, 0));    //idk\nsomething\nelse\n[]
    assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 0, 0)); //[]idk\nsomething\nelse\n
    
    assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 20, 0));   //idk\nsomething\nelse\n|: >    //is this the desired functionality?...
    assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 20, 0));
    assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 0, 0));
}

#[test]
fn move_horizontally(){
    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
    assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
    assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 1, 1));
    assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 0, 0));
    
    assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
    assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));
    assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 0, 0));
    
    // handles moving/extending to text bounds correctly
    assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 19, 0));
    assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 19, 0));
    assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 0, 0));
    
    assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 20, 0));
    assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 20, 0));
    assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 0, 0)); //:<idk\nsomething\nelse\n|
}

#[test]
fn set_from_line_number(){
    let text = Rope::from("idk\nsomething\nelse\n");
    
    // normal use
    assert_eq!(Selection::new(0, 0).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(0, 1).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(14, 15, 0));
    assert_eq!(Selection::new(0, 0).set_from_line_number(2, &text, Movement::Extend, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 14, 0));
    assert_eq!(Selection::new(0, 1).set_from_line_number(2, &text, Movement::Extend, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 15, 0));
    
    // restricts cursor to line end when stored_line_position > line width
    assert_eq!(Selection::new(13, 13).set_from_line_number(0, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 9));
    assert_eq!(Selection::new(13, 14).set_from_line_number(0, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 9));
    assert_eq!(Selection::new(13, 13).set_from_line_number(0, &text, Movement::Extend, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(13, 3, 9));
    assert_eq!(Selection::new(13, 14).set_from_line_number(0, &text, Movement::Extend, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(13, 3, 9));    //if at end of line, sets anchor before newline char
    
    //from end of text
    assert_eq!(Selection::new(19, 19).set_from_line_number(1, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(19, 20).set_from_line_number(1, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    assert_eq!(Selection::new(19, 19).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(19, 20).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(14, 15, 0));
}
#[test]
fn set_from_line_number_should_error_when_goal_line_number_greater_than_len_lines(){
    let text = Rope::from("idk\nsomething\nelse\n");    //num lines 4
    assert!(Selection::new(0, 0).set_from_line_number(5, &text, Movement::Move, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).set_from_line_number(5, &text, Movement::Move, CursorSemantics::Block).is_err());
}

#[test]
fn collapse(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // head < anchor
    assert_eq!(Selection::new(4, 0).collapse(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));  //<idk\n|some\nshit\n   //<|idk\nsome\nshit\n
    assert_eq!(Selection::new(4, 0).collapse(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));    //:<idk\n|some\nshit\n  //|:i>dk\nsome\nshit\n
    
    // anchor < head
    assert_eq!(Selection::new(0, 4).collapse(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));  //|idk\n>some\nshit\n   //idk\n|>some\nshit\n
    assert_eq!(Selection::new(0, 4).collapse(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));    //|idk\n>some\nshit\n   //idk|:\n>some\nshit\n
    
    // test setting cursor to end of text
    assert_eq!(Selection::new(0, 14).collapse(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));   //|idk\nsome\nshit\n>   //idk\nsome\nshit\n|>
    assert_eq!(Selection::new(0, 14).collapse(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(13, 14, 4)); //|idk\nsome\nshit:\n>  //idk\nsome\nshit|:\n>
}
#[test]
fn collapse_errors_if_already_not_extended(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).collapse(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).collapse(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_right(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // normal use
    assert_eq!(Selection::new(0, 0).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
    assert_eq!(Selection::new(0, 1).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
    
    // new line resets stored line position
    assert_eq!(Selection::new(3, 3).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(3, 4).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    
    // with selection extended, collapses selection, then performs move
    assert_eq!(Selection::new(0, 3).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(3, 0).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
    assert_eq!(Selection::new(0, 3).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
    assert_eq!(Selection::new(3, 0).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
}
#[test]
fn move_right_errors_if_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 14).move_right(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).move_right(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_left(){
    let text = Rope::from("idk\nsomething\nelse\n");
    
    // normal use
    assert_eq!(Selection::new(1, 1).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(1, 2).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    
    // move to previous line resets stored line position
    assert_eq!(Selection::new(4, 4).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(4, 5).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
    
    // with selection extended, collapses selection, then performs move
    assert_eq!(Selection::new(1, 4).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(4, 1).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(1, 4).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 3, 2));   // i[d k:\n]s o m e t h i n g \n e l s e
                                                                                                                                // i d[k]\n s o m e t h i n g \n e l s e
    assert_eq!(Selection::new(4, 1).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));   // i]d k \n[s o m e t h i n g \n e l s e
                                                                                                                                //[i]d k \n s o m e t h i n g \n e l s e
}
#[test]
fn move_left_errors_if_at_doc_start(){
    let text = Rope::from("idk\nsomething\nelse\n");
    assert!(Selection::new(0, 0).move_left(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_left(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_up(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // to shorter line
    assert_eq!(Selection::new(13, 13).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 9));
    assert_eq!(Selection::new(13, 14).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 9));
    
    // to longer line
    assert_eq!(Selection::new(18, 18).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(8, 8, 4));
    assert_eq!(Selection::new(18, 19).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(8, 9, 4));
    
    // with selection extended, collapses selection, then performs move
    assert_eq!(Selection::new(14, 14).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(14, 4).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(4, 14).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 9));
    assert_eq!(Selection::new(14, 4).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
}
#[test]
fn move_up_errors_if_on_topmost_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(0, 0).move_up(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_up(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_down(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // to longer line
    assert_eq!(Selection::new(3, 3).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(7, 7, 3));
    assert_eq!(Selection::new(3, 4).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(7, 8, 3));
    
    // to shorter line
    assert_eq!(Selection::new(13, 13).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(18, 18, 9));
    assert_eq!(Selection::new(13, 14).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(18, 19, 9));
    
    // with selection extended, collapses selection, then performs move
    assert_eq!(Selection::new(0, 4).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(4, 0).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    //[i d k \n]s o m e \n s h i t \n
    // i d k \n s o m[e]\n s h i t \n
    assert_eq!(Selection::new(0, 4).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(7, 8, 3));
    assert_eq!(Selection::new(4, 0).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
}
#[test]
fn move_down_errors_if_on_bottommost_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(18, 18).move_down(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(18, 19).move_down(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_line_text_end(){
    let text = Rope::from("idk\nsomething\nelse\n");
    
    assert_eq!(Selection::new(0, 0).move_line_text_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(0, 1).move_line_text_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
    
    // with selection extended, collapse and move
    assert_eq!(Selection::new(0, 2).move_line_text_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(2, 0).move_line_text_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(0, 2).move_line_text_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
    assert_eq!(Selection::new(2, 0).move_line_text_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
}
#[test]
fn move_line_text_end_errors_if_already_at_line_text_end(){
    let text = Rope::from("idk\nsomething\nelse\n");
    assert!(Selection::new(3, 3).extend_line_text_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(2, 3).extend_line_text_end(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_home(){
    let text = Rope::from("    idk\n");
    
    // moves to text start when cursor past text start
    assert_eq!(Selection::new(6, 6).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(6, 7).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    
    // moves to line start when cursor at text start
    assert_eq!(Selection::new(4, 4).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(4, 5).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    
    // moves to text start when cursor before text start
    assert_eq!(Selection::new(1, 1).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(1, 2).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    
    // with selection extended, collapse and move
    assert_eq!(Selection::new(0, 5).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(0, 3).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(0, 4).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(5, 0).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(0, 6).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(0, 4).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(0, 5).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(5, 0).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
}
#[test]
fn move_home_errors_if_line_start_same_as_text_start_and_cursor_at_text_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).move_home(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_home(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(3, 3).move_line_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(3, 4).move_line_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
}
#[test]
fn move_line_start_errors_if_already_at_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).move_line_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_line_start(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_line_text_start(){
    let text = Rope::from("  idk\n");
    assert_eq!(Selection::new(0, 0).move_line_text_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 2, 2));
    assert_eq!(Selection::new(0, 1).move_line_text_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 3, 2));
}
#[test]
fn move_line_text_start_errors_if_already_at_line_text_start(){
    let text = Rope::from("    idk\nsome\nshit\n");
    assert!(Selection::new(4, 4).move_line_text_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(4, 5).move_line_text_start(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_page_up(){
    use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::new(6, 6).move_page_up(&text, &client_view, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 2, 2));
    assert_eq!(Selection::new(6, 7).move_page_up(&text, &client_view, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 3, 2));
}
#[test]
fn move_page_up_errors_if_already_on_topmost_line(){
    use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert!(Selection::new(0, 0).move_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_page_up(&text, &client_view, CursorSemantics::Block).is_err());
}

#[test]
fn move_page_down(){
    use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::new(0, 0).move_page_down(&text, &client_view, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(0, 1).move_page_down(&text, &client_view, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
}
#[test]
fn move_page_down_errors_if_already_on_bottommost_line(){
    use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert!(Selection::new(14, 14).move_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).move_page_down(&text, &client_view, CursorSemantics::Block).is_err());
}

#[test]
fn move_doc_start(){
    let text = Rope::from("idk\n");
    assert_eq!(Selection::new(4, 4).move_doc_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(4, 5).move_doc_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
}
#[test]
fn move_doc_start_errors_if_already_at_doc_start(){
    let text = Rope::from("idk\n");
    assert!(Selection::new(0, 0).move_doc_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_doc_start(&text, CursorSemantics::Block).is_err());
}

#[test]
fn move_doc_end(){
    let text = Rope::from("idk\nsome\nshit");
    assert_eq!(Selection::new(0, 0).move_doc_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(13, 13, 4));
    assert_eq!(Selection::new(0, 1).move_doc_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(13, 14, 4));
}
#[test]
fn move_doc_end_errors_if_already_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 14).move_doc_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).move_doc_end(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_right(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // normal use
    assert_eq!(Selection::new(0, 0).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 1, 1));
    assert_eq!(Selection::new(0, 1).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));
    
    // resets stored line position after new line
    assert_eq!(Selection::new(3, 3).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 4, 0));
    assert_eq!(Selection::new(3, 4).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 5, 0));
    
    // previously extended
    assert_eq!(Selection::new(0, 3).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(3, 0).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 1, 1));
    assert_eq!(Selection::new(0, 3).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 4, 3));
    assert_eq!(Selection::new(3, 0).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 1, 1));
}
#[test]
fn extend_right_errors_if_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 14).extend_right(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).extend_right(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_left(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // normal use
    assert_eq!(Selection::new(2, 2).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 1, 1));
    assert_eq!(Selection::new(2, 3).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 1, 1)); //id[:k]\nsomething\nelse   //i:]dk[\nsomething\nelse
    
    //updates stored line position on line change
    assert_eq!(Selection::new(4, 4).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 3, 3));
    assert_eq!(Selection::new(4, 5).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 3, 3)); //idk\n[s]omething\nelse    //idk:]\ns[omething\nelse
    
    //previously extended
    assert_eq!(Selection::new(0, 3).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 2, 2));
    assert_eq!(Selection::new(3, 1).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 0, 0));
    assert_eq!(Selection::new(0, 3).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1)); //[id:k]\nsomething\nelse   //[i:d]k\nsomething\nelse
    assert_eq!(Selection::new(3, 1).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 0, 0)); //i:]dk[\nsomething\nelse   //:]idk[\nsomething\nelse
}
#[test]
fn extend_left_errors_if_at_doc_start(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(0, 0).extend_left(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_left(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_up(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // to shorter line
    assert_eq!(Selection::with_stored_line_position(13, 3, 9), Selection::new(13, 13).extend_up(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(13, 3, 9), Selection::new(13, 14).extend_up(&text, CursorSemantics::Block).unwrap()); //if at end of line, sets anchor before newline char
    
    // to longer line
    assert_eq!(Selection::with_stored_line_position(18, 8, 4), Selection::new(18, 18).extend_up(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(18, 8, 4), Selection::new(18, 19).extend_up(&text, CursorSemantics::Block).unwrap()); //idk\nsomething\nelse[: ]   //idk\nsome:]thing\nelse[
}
#[test]
fn extend_up_errors_if_on_topmost_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(0, 0).extend_up(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_up(&text, CursorSemantics::Block).is_err());
}

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

#[test]
fn extend_line_text_end(){
    let text = Rope::from("idk\n");
    assert_eq!(Selection::with_stored_line_position(0, 3, 3), Selection::new(0, 0).extend_line_text_end(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_end(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_line_text_end_errors_if_already_at_text_end(){
    let text = Rope::from("idk\n");
    assert!(Selection::new(4, 4).extend_line_text_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(3, 4).extend_line_text_end(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_home(){
    let text = Rope::from("    idk\n");
    
    // extends selection to text start when head past text start
    assert_eq!(Selection::with_stored_line_position(6, 4, 4), Selection::new(6, 6).extend_home(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(7, 4, 4), Selection::new(6, 7).extend_home(&text, CursorSemantics::Block).unwrap());
    
    // extends selection to line start when head at text start
    assert_eq!(Selection::with_stored_line_position(4, 0, 0), Selection::new(4, 4).extend_home(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(5, 0, 0), Selection::new(4, 5).extend_home(&text, CursorSemantics::Block).unwrap());   //    [:i]dk\n  //:]    i[dk\n
    
    // extends selection to text start when head before text start
    assert_eq!(Selection::with_stored_line_position(1, 4, 4), Selection::new(1, 1).extend_home(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(1, 5, 4), Selection::new(1, 2).extend_home(&text, CursorSemantics::Block).unwrap()); // [: ]  idk\n  // [   :i]dk\n
}
#[test]
fn extend_home_errors_if_line_start_same_as_text_start_and_cursor_at_text_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).extend_home(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_home(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 3).extend_line_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 4).extend_line_start(&text, CursorSemantics::Block).unwrap());   //special case  //if at end of line, sets anchor before newline char
}
#[test]
fn extend_line_start_errors_if_already_at_line_start(){
    let text = Rope::from("idk\n");
    assert!(Selection::new(0, 0).extend_home(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_home(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_line_text_start(){
    let text = Rope::from("  idk\n");
    assert_eq!(Selection::with_stored_line_position(0, 2, 2), Selection::new(0, 0).extend_line_text_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_start(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_line_text_start_errors_if_already_at_text_start(){
    let text = Rope::from("    idk\n");
    assert!(Selection::new(4, 4).extend_line_text_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(4, 5).extend_line_text_start(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_page_up(){
    use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::with_stored_line_position(6, 2, 2), Selection::new(6, 6).extend_page_up(&text, &client_view, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(7, 2, 2), Selection::new(6, 7).extend_page_up(&text, &client_view, CursorSemantics::Block).unwrap());    //idk\nso[m]ething\nelse    //id:]k\nsom[ething\nelse
}
#[test]
fn extend_page_up_errors_if_on_topmost_line(){
    use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert!(Selection::new(3, 3).extend_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(3, 4).extend_page_up(&text, &client_view, CursorSemantics::Block).is_err());
}

#[test]
fn extend_page_down(){
    use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::with_stored_line_position(0, 4, 0), Selection::new(0, 0).extend_page_down(&text, &client_view, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 5, 0), Selection::new(0, 1).extend_page_down(&text, &client_view, CursorSemantics::Block).unwrap());  //[i]dk\nsomething\nelse    //[idk\n:s]omething\nelse
}
#[test]
fn extend_page_down_errors_if_on_bottommost_line(){
    use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert!(Selection::new(14, 14).extend_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).extend_page_down(&text, &client_view, CursorSemantics::Block).is_err());
}

#[test]
fn extend_doc_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::with_stored_line_position(9, 0, 0), Selection::new(9, 9).extend_doc_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(10, 0, 0), Selection::new(9, 10).extend_doc_start(&text, CursorSemantics::Block).unwrap());  //idk\nsome\n[s]hit\n   //:]idk\nsome\ns[hit\n
}
#[test]
fn extend_doc_start_errors_if_already_at_doc_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).extend_doc_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_doc_start(&text, CursorSemantics::Block).is_err());
}

#[test]
fn extend_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).extend_doc_end(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 15, 0), Selection::new(0, 1).extend_doc_end(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_doc_end_errors_if_already_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 14).extend_doc_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).extend_doc_end(&text, CursorSemantics::Block).is_err());
}

#[test]
fn select_all(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).select_all(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 15, 0), Selection::new(0, 1).select_all(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn select_all_errors_if_already_all_selected(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 14).select_all(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 15).select_all(&text, CursorSemantics::Block).is_err());
}

#[test]
fn shift_and_extend(){
    // TODO: impl tests
    //assert!(false);
}

#[test]
fn selection_to_selection2d(){
    use edit_core::selection::Selection2d;
    use edit_core::Position;

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









////////////////////////// Selections ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
use edit_core::selection::Selections;

#[test]
fn selections_new(){
    // sorts and merges overlapping
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![
        Selection::new(2, 4),    // i d[k \n]s o m e \n s h i t \n
        Selection::new(0, 5),    //[i d k \n s]o m e \n s h i t \n
        Selection::new(3, 6)     // i d k[\n s o]m e \n s h i t \n
    ], 0, &text);
    let expected_selections = Selections::new(vec![
        Selection::with_stored_line_position(0, 6, 2)     //[i d k \n s o]m e \n s h i t \n
    ], 0, &text);
    assert_eq!(expected_selections, selections);
}

#[test]
#[should_panic]
fn selections_new_should_panic_if_input_selections_empty(){
    let text = Rope::from("idk\nsome\nshit\n");
    Selections::new(vec![], 0, &text);  //panics
}

#[test]
fn pop(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0)], 0, &text), selections.pop());
    
    // always contains at least one selection
    let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0)], 0, &text), selections.pop());
}

#[test]
fn push_front(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text), selections.push_front(Selection::new(0, 0), true));
    
    let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push_front(Selection::new(0, 0), false));
}

#[test]
fn push(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text); //[]idk\nsome\nshit\n
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push(Selection::new(4, 4), true));
}

#[test]
fn increment_primary_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // increments
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text), selections.increment_primary_selection().unwrap());
    
    // wraps on last selection
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text), selections.increment_primary_selection().unwrap());
}
#[test]
fn increment_primary_selection_errors_if_only_one_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).increment_primary_selection().is_err());
}

#[test]
fn decrement_primary_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // decrements
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text), selections.decrement_primary_selection().unwrap());
    
    // wraps on first selection
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text), selections.decrement_primary_selection().unwrap());
}
#[test]
fn decrement_primary_selection_errors_if_only_one_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).decrement_primary_selection().is_err());
}

#[test]
fn sort(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![
        Selection::new(2, 4),
        Selection::new(0, 5),
        Selection::new(3, 6)
    ], 0, &text);
    let expected_selections = Selections::new(vec![
        Selection::new(0, 5),
        Selection::new(2, 4),
        Selection::new(3, 6)
    ], 1, &text);
    assert_eq!(expected_selections, selections.sort());
}

#[test]
fn merge_overlapping_selections(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    let mut selections = Selections::new(vec![
        Selection::new(0, 2),    //[i d]k \n s o m e \n s h i t \n
        Selection::new(1, 4),    // i[d k \n]s o m e \n s h i t \n
        Selection::new(5, 7),    // i d k \n s[o m]e \n s h i t \n
        Selection::new(8, 10),   // i d k \n s o m e[\n s]h i t \n
        Selection::new(9, 12)    // i d k \n s o m e \n[s h i]t \n
    ], 4, &text);
    let expected_selections = Selections::new(vec![
        Selection::with_stored_line_position(0, 4, 0),    //[i d k \n]s o m e \n s h i t \n
        Selection::new(5, 7),    // i d k \n s[o m]e \n s h i t \n
        Selection::with_stored_line_position(8, 12, 3)    // i d k \n s o m e[\n s h i]t \n
    ], 2, &text);
    assert_eq!(expected_selections, selections.merge_overlapping(&text));
}

#[test]
fn clear_non_primary_selections(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // normal use
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text);
    assert_eq!(Selections::new(vec![Selection::new(4, 4)], 0, &text), selections.clear_non_primary_selections().unwrap());
}
#[test]
fn clear_non_primary_selections_errors_if_only_one_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).clear_non_primary_selections().is_err());
}

#[test]
fn add_selection_above_with_no_selection_extension_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text)),
        selections.add_selection_above(&text, CursorSemantics::Bar)
    );
}
#[test]
fn add_selection_above_with_no_selection_extension_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 5)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 1, &text)),
        selections.add_selection_above(&text, CursorSemantics::Block)
    );

    // doesn't currently handle selections with Backwards Direction. not entirely sure this is necessary...
    //let selections = Selections::new(vec![Selection::new(5, 4)], 0, &text);
    //assert_eq!(
    //    Ok(Selections::new(vec![Selection::new(1, 0), Selection::new(5, 4)], 1, &text)),
    //    selections.add_selection_above(&text, CursorSemantics::Block)
    //);
}
#[test]
fn add_selection_above_with_selection_extension_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(5, 7)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 1, &text)),
        selections.add_selection_above(&text, CursorSemantics::Bar)
    );
}
#[test]
fn add_selection_above_with_selection_extension_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(5, 7)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 1, &text)),
        selections.add_selection_above(&text, CursorSemantics::Block)
    );
}
#[test]
fn add_selection_above_with_newline_bar_semantics(){
    let text = Rope::from("\n\nidk\n");
    let selections = Selections::new(vec![Selection::new(1, 1)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text)),
        selections.add_selection_above(&text, CursorSemantics::Bar)
    );
}
#[test]
fn add_selection_above_with_newline_block_semantics(){
    let text = Rope::from("\n\nidk\n");
    let selections = Selections::new(vec![Selection::new(1, 2)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(0, 1), Selection::new(1, 2)], 1, &text)),
        selections.add_selection_above(&text, CursorSemantics::Block)
    );
}
// TODO: add selection above with empty line
#[test]
fn add_selection_above_with_selection_extension_across_lines_of_multiple_widths_bar_semantics(){
    let text = Rope::from("idk\ni\nidk\n");
    let mut selections = Selections::new(vec![Selection::new(6, 9)], 0, &text);
    selections = selections.add_selection_above(&text, CursorSemantics::Bar).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(4, 5), Selection::new(6, 9)], 1, &text), selections);
    selections = selections.add_selection_above(&text, CursorSemantics::Bar).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 5), Selection::new(6, 9)], 2, &text), selections);
}
#[test]
fn add_selection_above_with_selection_extension_across_lines_of_multiple_widths_block_semantics(){
    let text = Rope::from("idk\ni\nidk\n");
    let mut selections = Selections::new(vec![Selection::new(6, 9)], 0, &text);
    selections = selections.add_selection_above(&text, CursorSemantics::Block).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(4, 6), Selection::new(6, 9)], 1, &text), selections);
    selections = selections.add_selection_above(&text, CursorSemantics::Block).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 6), Selection::new(6, 9)], 2, &text), selections);
}
#[test]
fn add_selection_above_should_error_when_selection_on_line_0(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(1, 3)], 0, &text);
    assert!(selections.add_selection_above(&text, CursorSemantics::Bar).is_err());
    assert!(selections.add_selection_above(&text, CursorSemantics::Block).is_err());
}
#[test]
fn add_selection_above_should_error_when_any_selection_is_multi_line(){ //may not require this later...
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 10)], 0, &text);
    assert!(selections.add_selection_above(&text, CursorSemantics::Bar).is_err());
    assert!(selections.add_selection_above(&text, CursorSemantics::Block).is_err());
}

#[test]
fn add_selection_below_with_no_selection_extension_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(4, 4), Selection::new(9, 9)], 0, &text)),
        selections.add_selection_below(&text, CursorSemantics::Bar)
    );
}
#[test]
fn add_selection_below_with_no_selection_extension_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 5)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(4, 5), Selection::new(9, 10)], 0, &text)),
        selections.add_selection_below(&text, CursorSemantics::Block)
    );
}
#[test]
fn add_selection_below_with_selection_extension_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(5, 7)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(5, 7), Selection::new(10, 12)], 0, &text)),
        selections.add_selection_below(&text, CursorSemantics::Bar)
    );
}
#[test]
fn add_selection_below_with_selection_extension_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(5, 7)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(5, 7), Selection::new(10, 12)], 0, &text)),
        selections.add_selection_below(&text, CursorSemantics::Block)
    );
}
#[test]
fn add_selection_below_with_newline_bar_semantics(){
    let text = Rope::from("\n\nidk\n");
    let selections = Selections::new(vec![Selection::new(1, 1)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(1, 1), Selection::new(2, 2)], 0, &text)),
        selections.add_selection_below(&text, CursorSemantics::Bar)
    );
}
#[test]
fn add_selection_below_with_newline_block_semantics(){
    let text = Rope::from("\n\nidk\n");
    let selections = Selections::new(vec![Selection::new(1, 2)], 0, &text);
    assert_eq!(
        Ok(Selections::new(vec![Selection::new(1, 2), Selection::new(2, 3)], 0, &text)),
        selections.add_selection_below(&text, CursorSemantics::Block)
    );
}
//// TODO: add selection below with empty line
#[test]
fn add_selection_below_with_selection_extension_across_lines_of_multiple_widths_bar_semantics(){
    let text = Rope::from("idk\ni\nidk\n");
    let mut selections = Selections::new(vec![Selection::new(0, 3)], 0, &text);
    selections = selections.add_selection_below(&text, CursorSemantics::Bar).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 5)], 0, &text), selections);
    selections = selections.add_selection_below(&text, CursorSemantics::Bar).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 5), Selection::new(6, 9)], 0, &text), selections);
}
#[test]
fn add_selection_below_with_selection_extension_across_lines_of_multiple_widths_block_semantics(){
    let text = Rope::from("idk\ni\nidk\n");
    let mut selections = Selections::new(vec![Selection::new(0, 3)], 0, &text);
    selections = selections.add_selection_below(&text, CursorSemantics::Block).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 6)], 0, &text), selections);
    selections = selections.add_selection_below(&text, CursorSemantics::Block).unwrap();
    assert_eq!(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 6), Selection::new(6, 9)], 0, &text), selections);
}
#[test]
fn add_selection_below_should_error_when_selection_on_last_line(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(14, 14)], 0, &text);
    assert!(selections.add_selection_below(&text, CursorSemantics::Bar).is_err());
    let selections = Selections::new(vec![Selection::new(14, 15)], 0, &text);
    assert!(selections.add_selection_below(&text, CursorSemantics::Block).is_err());
}
#[test]
fn add_selection_below_should_error_when_any_selection_is_multi_line(){ //may not require this later...
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 10)], 0, &text);
    assert!(selections.add_selection_below(&text, CursorSemantics::Bar).is_err());
    assert!(selections.add_selection_below(&text, CursorSemantics::Block).is_err());
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

