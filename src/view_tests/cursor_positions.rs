use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::selections::Selections;
use crate::view::View;
use crate::document::Document;
use crate::position::Position;

fn test(text: &Rope, selection: Selection, expected: Vec<Position>, view: View, semantics: CursorSemantics) -> bool{
    //let text = Rope::from("idk\nsome\nshit\n");
    let /*mut */doc = Document::new(semantics)
        .with_text(text.clone())
        .with_selections(Selections::new(vec![selection], 0, &text, semantics))
        .with_view(view);
    println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.client_view.cursor_positions(&text, &doc.selections, semantics));
    doc.client_view.cursor_positions(&text, &doc.selections, semantics) == expected
}

#[test] fn cursor_positions(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(test(Selection::new(0, 0), vec![Position::new(0, 0)], View::new(0, 0, 2, 2), CursorSemantics::Bar));
    assert!(test(&text, Selection::new(Range::new(0, 0), Direction::Forward), vec![Position::new(0, 0)], View::new(0, 0, 2, 2), CursorSemantics::Bar));
    //assert!(test(Selection::new(0, 1), vec![Position::new(0, 0)], View::new(0, 0, 2, 2), CursorSemantics::Block));
    assert!(test(&text, Selection::new(Range::new(0, 1), Direction::Forward), vec![Position::new(0, 0)], View::new(0, 0, 2, 2), CursorSemantics::Block));
    //assert!(test(Selection::new(0, 0), Vec::new(), View::new(1, 0, 2, 2), CursorSemantics::Bar));
    assert!(test(&text, Selection::new(Range::new(0, 0), Direction::Forward), Vec::new(), View::new(1, 0, 2, 2), CursorSemantics::Bar));
    //assert!(test(Selection::new(0, 1), Vec::new(), View::new(1, 0, 2, 2), CursorSemantics::Block));
    assert!(test(&text, Selection::new(Range::new(0, 1), Direction::Forward), Vec::new(), View::new(1, 0, 2, 2), CursorSemantics::Block));
    //assert!(test(Selection::new(0, 0), Vec::new(), View::new(1, 1, 2, 2), CursorSemantics::Bar));
    assert!(test(&text, Selection::new(Range::new(0, 0), Direction::Forward), Vec::new(), View::new(1, 1, 2, 2), CursorSemantics::Bar));
    //assert!(test(Selection::new(0, 1), Vec::new(), View::new(1, 1, 2, 2), CursorSemantics::Block));
    assert!(test(&text, Selection::new(Range::new(0, 1), Direction::Forward), Vec::new(), View::new(1, 1, 2, 2), CursorSemantics::Block));
}
