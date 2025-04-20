//use ropey::Rope;
//use crate::document::Document;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//use crate::selections::Selections;
//
//fn copy_test(selection: Selection, expected: &str, semantics: CursorSemantics) -> bool{
//    let text = Rope::from("idk\nsome\nshit\n");
//    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
//    let _ = doc.copy();
//    println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.clipboard());
//    doc.clipboard() == expected
//}
//
//#[test]
//fn copy_with_selection_anchor_less_than_head(){
//    //assert!(copy_test(Selection::new(4, 9), "some\n", CursorSemantics::Bar));
//    assert!(copy_test(Selection::new(Range::new(4, 9), Direction::Forward), "some\n", CursorSemantics::Bar));
//    //assert!(copy_test(Selection::new(4, 9), "some\n", CursorSemantics::Block));    //idk\n|some:\n>shit\n
//    assert!(copy_test(Selection::new(Range::new(4, 9), Direction::Forward), "some\n", CursorSemantics::Block));    //idk\n|some:\n>shit\n
//}
//
//#[test]
//fn copy_with_multiple_selections_should_error(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //let mut doc = Document::new(CursorSemantics::Bar).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text));
//    let mut doc = Document::new(CursorSemantics::Bar)
//        .with_selections(
//            Selections::new(
//                vec![
//                    Selection::new(Range::new(0, 0), Direction::Forward), 
//                    Selection::new(Range::new(4, 4), Direction::Forward)
//                    ], 
//                0, 
//                &text
//            )
//        );
//    assert!(doc.copy().is_err());
//}
//