use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use ropey::Rope;
    
// if selection not extended, should always be false
#[test] fn selection_not_extended_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(false, Selection::new(Range::new(14, 14), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Bar));    // i d k \n s o m e \n s h i t \n|>
}
#[test] fn selection_not_extended_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(false, Selection::new(Range::new(13, 14), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Block));  // i d k \n s o m e \n s h i t|\n>
    assert_eq!(false, Selection::new(Range::new(13, 14), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Block));  // i d k \n s o m e \n s h i t<\n|
}

// if selection extended on same line, should always be false
#[test] fn selection_extended_on_same_line_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(false, Selection::new(Range::new(0, 3), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Bar));      //|i d k>\n s o m e \n s h i t \n
    assert_eq!(false, Selection::new(Range::new(0, 3), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Bar));      //<i d k|\n s o m e \n s h i t \n
}
#[test] fn selection_extended_on_same_line_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(false, Selection::new(Range::new(0, 3), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Block));    //|i d:k>\n s o m e \n s h i t \n
    assert_eq!(false, Selection::new(Range::new(0, 3), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Block));    //<i d k|\n s o m e \n s h i t \n
}

// if selection extended to line end and difference between lines is 1, should always be false
#[test] fn selection_extended_on_same_line_to_newline_char_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(false, Selection::new(Range::new(0, 4), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Bar));      //|i d k \n>s o m e \n s h i t \n
    assert_eq!(false, Selection::new(Range::new(0, 4), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Bar));      //<i d k \n|s o m e \n s h i t \n
}
#[test] fn selection_extended_on_same_line_to_newline_char_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(false, Selection::new(Range::new(0, 4), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Block));    //|i d k:\n>s o m e \n s h i t \n
    assert_eq!(false, Selection::new(Range::new(0, 4), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Block));    //<i d k \n|s o m e \n s h i t \n
}

// all other cases should be true
#[test] fn selection_extended_to_other_lines_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(true, Selection::new(Range::new(0, 5), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Bar));       //|i d k \n s>o m e \n s h i t \n
    assert_eq!(true, Selection::new(Range::new(0, 5), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Bar));       //|i d k \n s>o m e \n s h i t \n
}
#[test] fn selection_extended_to_other_lines_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n"); //len 14        //max bar = 14, max block = 15
    assert_eq!(true, Selection::new(Range::new(0, 5), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Block));     //|i d k \n:s>o m e \n s h i t \n
    assert_eq!(true, Selection::new(Range::new(0, 5), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Block));     //|i d k \n:s>o m e \n s h i t \n
}

//selection shouldn't be able to extend past doc end, but cursor can move there
#[test] fn verify_cursor_past_doc_text(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(false, Selection::new(Range::new(14, 15), Direction::Forward).spans_multiple_lines(&text, CursorSemantics::Block)); // i d k \n s o m e \n s h i t \n|: >
    assert_eq!(false, Selection::new(Range::new(14, 15), Direction::Backward).spans_multiple_lines(&text, CursorSemantics::Block)); // i d k \n s o m e \n s h i t \n< |
}
