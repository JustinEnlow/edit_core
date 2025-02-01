use ropey::Rope;
use crate::document::Document;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::selections::Selections;

fn cut_test(selection: Selection, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
    let text = Rope::from("idk\nsome\nshit\n");
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
    let _ = doc.cut(semantics);
    println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
    doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection

    //TODO: ensure clipboard text is correct as well
}

#[test]
fn cut_with_selection_anchor_less_than_head(){
    //assert!(cut_test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
    assert!(cut_test(Selection::new(Range::new(4, 9), Direction::Forward), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0), CursorSemantics::Bar));
    //assert!(cut_test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    assert!(cut_test(Selection::new(Range::new(4, 9), Direction::Forward), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0), CursorSemantics::Block));
}

#[test]
fn cut_with_selection_anchor_greater_than_head(){
    //assert!(cut_test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
    assert!(cut_test(Selection::new(Range::new(4, 9), Direction::Backward), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0), CursorSemantics::Bar));
    //assert!(cut_test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    assert!(cut_test(Selection::new(Range::new(4, 9), Direction::Backward), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0), CursorSemantics::Block));
}

#[test]
fn cut_with_multiple_selections_returns_error(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let mut doc = Document::new(CursorSemantics::Bar).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 7)], 0, &text));
    let mut doc = Document::new(CursorSemantics::Bar)
        .with_selections(
            Selections::new(
                vec![
                    Selection::new(Range::new(0, 3), Direction::Forward),
                    Selection::new(Range::new(4, 7), Direction::Forward)
                ],
                0,
                &text
            )
        );
    assert!(doc.cut(CursorSemantics::Bar).is_err());
}
