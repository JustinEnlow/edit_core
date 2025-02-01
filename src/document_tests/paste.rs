use ropey::Rope;
use crate::document::Document;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::selections::Selections;

fn paste_test(selection: Selection, string: &str, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
    let text = Rope::from("idk\nsome\nshit\n");
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text)).with_clipboard(string.to_string());
    let _ = doc.paste(semantics);
    println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
    doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection
}

#[test]
fn paste(){
    //assert!(paste_test(Selection::new(9, 9), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 15, 0), CursorSemantics::Bar));
    assert!(paste_test(Selection::new(Range::new(9, 9), Direction::Forward), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(Range::new(15, 15), Direction::Forward, 0), CursorSemantics::Bar));
    //assert!(paste_test(Selection::new(9, 10), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 16, 0), CursorSemantics::Block));
    assert!(paste_test(Selection::new(Range::new(9, 10), Direction::Forward), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(Range::new(15, 16), Direction::Forward, 0), CursorSemantics::Block));
}
