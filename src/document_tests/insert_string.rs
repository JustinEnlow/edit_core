//use ropey::Rope;
//use crate::document::Document;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//use crate::selections::Selections;
//
//#[test] fn idk_insert_single_char_with_multi_selection(){
//    let text = Rope::from("some\nshit\n");
//        
//    let semantics = CursorSemantics::Block;
//    //let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
//    let mut doc = Document::new(semantics)
//        .with_text(text.clone())
//        .with_selections(
//            Selections::new(
//                vec![
//                    Selection::new(Range::new(0, 1), Direction::Forward),
//                    Selection::new(Range::new(5, 6), Direction::Forward)
//                ], 
//                0, 
//                &text
//            )
//        );
//    let _ = doc.insert_string("x", semantics);
//    assert_eq!("xsome\nxshit\n", doc.text());
//    //assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
//    assert_eq!(
//        &Selections::new(
//            vec![
//                Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1),
//                Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 1)
//            ], 
//            0, 
//            &text
//        ),
//        doc.selections()
//    );
//    assert!(doc.is_modified());
//
//    let semantics = CursorSemantics::Bar;
//    //let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
//    let mut doc = Document::new(semantics)
//        .with_text(text.clone())
//        .with_selections(
//            Selections::new(
//                vec![
//                    Selection::new(Range::new(0, 0), Direction::Forward),
//                    Selection::new(Range::new(5, 5), Direction::Forward)
//                ],
//                0,
//                &text
//            )
//        );
//    let _ = doc.insert_string("x", semantics);
//    assert_eq!("xsome\nxshit\n", doc.text());
//    //assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
//    assert_eq!(
//        &Selections::new(
//            vec![
//                Selection::with_stored_line_position(Range::new(1, 1), Direction::Forward, 1),
//                Selection::with_stored_line_position(Range::new(7, 7), Direction::Forward, 1)
//            ],
//            0,
//            &text
//        ),
//        doc.selections()
//    );
//    assert!(doc.is_modified());
//}
//
//// TODO: insert multi-char with multi selection
//
//#[test] fn errors_if_empty_insert_string(){
//    let text = Rope::from("some\nshit\n");
//        
//    let semantics = CursorSemantics::Block;
//    //let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
//    let mut doc = Document::new(semantics)
//        .with_text(text.clone())
//        .with_selections(
//            Selections::new(
//                vec![
//                    Selection::new(Range::new(0, 1), Direction::Forward),
//                    Selection::new(Range::new(5, 6), Direction::Forward)
//                ],
//                0,
//                &text
//            )
//        );
//    assert!(doc.insert_string("", semantics).is_err());
//
//    let semantics = CursorSemantics::Bar;
//    //let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
//    let mut doc = Document::new(semantics)
//        .with_text(text.clone())
//        .with_selections(
//            Selections::new(
//                vec![
//                    Selection::new(Range::new(0, 1), Direction::Forward),
//                    Selection::new(Range::new(5, 6), Direction::Forward)
//                ], 
//                0, 
//                &text
//            )
//        );
//    assert!(doc.insert_string("", semantics).is_err());
//}
//