use ropey::Rope;
use crate::document::Document;
use crate::selection::{Selection, CursorSemantics};
use crate::selections::Selections;

fn backspace_test(name: &str, selection: Selection, expected: Rope, semantics: CursorSemantics) -> bool{
    let text = Rope::from("idk\nsome\nshit\n");
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    let _ = doc.backspace(semantics);
    println!("{:#?}\n{:#?}\nexpected: {:#?}\ngot: {:#?}\n", name, semantics, expected, doc.text().clone());
    doc.text().clone() == expected
}

#[test]
fn backspace_does_nothing_at_doc_start(){
    assert!(backspace_test("test0", Selection::new(0, 0), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(backspace_test("test0", Selection::new(0, 1), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block));
}
#[test]
fn backspace_without_selection_deletes_previous_char(){
    assert!(backspace_test("test1", Selection::new(1, 1), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(backspace_test("test1", Selection::new(1, 2), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));   //i|:d>k\nsome\nshit\n
}
#[test]
fn backspace_at_line_start_appends_current_line_to_end_of_previous_line(){
    assert!(backspace_test("test2", Selection::new(4, 4), Rope::from("idksome\nshit\n"), CursorSemantics::Bar));
    assert!(backspace_test("test2", Selection::new(4, 5), Rope::from("idksome\nshit\n"), CursorSemantics::Block));
}
#[test]
fn backspace_with_selection_head_greater_than_anchor(){
    assert!(backspace_test("test3", Selection::new(0, 2), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(backspace_test("test3", Selection::new(0, 2), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block));
}
#[test]
fn backspace_with_selection_head_less_than_anchor(){
    assert!(backspace_test("test4", Selection::new(2, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
    assert!(backspace_test("test4", Selection::new(2, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block));
}
#[test]
fn backspace_at_text_end(){
    assert!(backspace_test("test5", Selection::new(14, 14), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
    assert!(backspace_test("test5", Selection::new(14, 15), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit\n|: > //idk\nsome\nshit|: >
}
#[test]
fn backspace_removes_previous_tab(){
    use crate::document::TAB_WIDTH;
    let mut spaces = String::new();
    for _ in 0..TAB_WIDTH{
        spaces.push(' ');
    }
    let text = Rope::from(format!("{}idk\nsome\nshit\n", spaces));
    let semantics = CursorSemantics::Block; //test Bar too
    let selection = Selection::new(TAB_WIDTH, match semantics{CursorSemantics::Bar => TAB_WIDTH, CursorSemantics::Block => TAB_WIDTH.saturating_add(1)});
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    let _ = doc.backspace(semantics);
    assert!(doc.text().clone() == Rope::from("idk\nsome\nshit\n"));
    assert!(doc.selections().primary().clone() == Selection::with_stored_line_position(0, match semantics{CursorSemantics::Bar => 0, CursorSemantics::Block => 1}, 0));
}
