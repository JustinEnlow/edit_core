use ropey::Rope;
use crate::document::Document;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::selections::Selections;

fn delete_test(name: &str, selection: Selection, expected_selection: Selection, expected_text: Rope, semantics: CursorSemantics) -> bool{
    let text = Rope::from("idk\nsome\nshit\n");
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    let _ = doc.delete(semantics);
    println!("{:#?}\n{:#?}\nexpected_text {:#?}\ngot: {:#?}\nexpected_selection: {:#?}\ngot: {:#?}\n", name, semantics, expected_text, doc.text().clone(), expected_selection, doc.selections().primary().clone());
    doc.text().clone() == expected_text &&
    doc.selections().primary().clone() == expected_selection
}

#[test]
fn delete_will_not_delete_past_end_of_doc(){
    //assert!(delete_test("test1", Selection::new(14, 14), Selection::new(14, 14), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(delete_test("test1", Selection::new(Range::new(14, 14), Direction::Forward), Selection::new(Range::new(14, 14), Direction::Forward), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
    //assert!(delete_test("test1", Selection::new(14, 15), Selection::new(14, 15), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block)); //idk\nsome\nshit\n|: >
    assert!(delete_test("test1", Selection::new(Range::new(14, 15), Direction::Forward), Selection::new(Range::new(14, 15), Direction::Forward), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block)); //idk\nsome\nshit\n|: >
}
#[test]
fn delete_with_no_selection(){
    //assert!(delete_test("test2", Selection::new(0, 0), Selection::with_stored_line_position(0, 0, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(delete_test("test2", Selection::new(Range::new(0, 0), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
    //assert!(delete_test("test2", Selection::new(0, 1), Selection::with_stored_line_position(0, 1, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));    //|:i>dk\nsome\nshit\n
    assert!(delete_test("test2", Selection::new(Range::new(0, 1), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));    //|:i>dk\nsome\nshit\n
}
#[test]
fn delete_with_selection_head_greater_than_anchor(){
    //assert!(delete_test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 0, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(delete_test("test3", Selection::new(Range::new(0, 2), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    //assert!(delete_test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 1, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block)); //|i:d>k\nsome\nshit\n
    assert!(delete_test("test3", Selection::new(Range::new(0, 2), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block)); //|i:d>k\nsome\nshit\n
}
#[test]
fn delete_with_selection_head_less_than_anchor(){
    //assert!(delete_test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 1, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(delete_test("test4", Selection::new(Range::new(1, 3), Direction::Backward), Selection::with_stored_line_position(Range::new(1, 1), Direction::Forward, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Bar));
    //assert!(delete_test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 2, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Block));    //i|d:k>\nsome\nshit\n
    assert!(delete_test("test4", Selection::new(Range::new(1, 3), Direction::Backward), Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Block));    //i|d:k>\nsome\nshit\n
}
#[test]
fn delete_with_whole_text_selected(){
    //assert!(delete_test("test5", Selection::new(0, 13), Selection::with_stored_line_position(0, 0, 0), Rope::from("\n"), CursorSemantics::Bar));  //just verifying...
    assert!(delete_test("test5", Selection::new(Range::new(0, 13), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0), Rope::from("\n"), CursorSemantics::Bar));  //just verifying...
    //assert!(delete_test("test5", Selection::new(0, 14), Selection::with_stored_line_position(0, 0, 0), Rope::from(""), CursorSemantics::Bar));
    assert!(delete_test("test5", Selection::new(Range::new(0, 14), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0), Rope::from(""), CursorSemantics::Bar));
    //assert!(delete_test("test5", Selection::new(0, 15), Selection::with_stored_line_position(0, 1, 0), Rope::from(""), CursorSemantics::Block));  //|idk\nsome\nshit\n: >
    assert!(delete_test("test5", Selection::new(Range::new(0, 15), Direction::Forward), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0), Rope::from(""), CursorSemantics::Block));  //|idk\nsome\nshit\n: >
}
#[test]
fn delete_at_1_less_doc_end(){
    //assert!(delete_test("test6", Selection::new(13, 13), Selection::with_stored_line_position(13, 13, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
    assert!(delete_test("test6", Selection::new(Range::new(13, 13), Direction::Forward), Selection::with_stored_line_position(Range::new(13, 13), Direction::Forward, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
    //assert!(delete_test("test6", Selection::new(13, 14), Selection::with_stored_line_position(13, 14, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit|:\n> //idk\nsome\nshit|: >
    assert!(delete_test("test6", Selection::new(Range::new(13, 14), Direction::Forward), Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit|:\n> //idk\nsome\nshit|: >
}
