use ropey::Rope;
use edit_core::document::Document;
use edit_core::selection::{Selection, Selections, CursorSemantics};

/*
TODO:
    do all tests with block and bar cursor semantics

    test insert with hard tab
    test insert with soft tab

    cut/undo/redo
    copy
    paste/undo/redo
*/

#[test]
fn insert_single_char_with_multi_selection(){
    let text = Rope::from("some\nshit\n");
        
    let semantics = CursorSemantics::Block;
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
    doc.insert_string("x", semantics);
    assert_eq!("xsome\nxshit\n", doc.text());
    assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
    assert!(doc.is_modified());
    let _ = doc.undo(semantics);
    assert_eq!("some\nshit\n", doc.text());
    assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text), doc.selections());
    assert!(!doc.is_modified());
    let _ = doc.redo(semantics);
    assert_eq!("xsome\nxshit\n", doc.text());
    assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
    assert!(doc.is_modified());

    let semantics = CursorSemantics::Bar;
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
    doc.insert_string("x", semantics);
    assert_eq!("xsome\nxshit\n", doc.text());
    assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
    assert!(doc.is_modified());
    let _ = doc.undo(semantics);
    assert_eq!("some\nshit\n", doc.text());
    assert_eq!(&Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text), doc.selections());
    assert!(!doc.is_modified());
    let _ = doc.redo(semantics);
    assert_eq!("xsome\nxshit\n", doc.text());
    assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
    assert!(doc.is_modified());
}

    #[test]
    fn insert_multi_char_with_multi_selection(){
        let text = Rope::from("some\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
        doc.insert_string("idk\n", semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 0), Selection::with_stored_line_position(13, 14, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("some\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 0), Selection::with_stored_line_position(13, 14, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
        doc.insert_string("idk\n", semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 0), Selection::with_stored_line_position(13, 13, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("some\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("idk\nsome\nidk\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 0), Selection::with_stored_line_position(13, 13, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_forward_single_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(9, 10)], 0, &text));
        doc.delete(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 1), Selection::new(9, 10)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(9, 9)], 0, &text));
        doc.delete(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 0), Selection::new(9, 9)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_forward_multi_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.delete(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.delete(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_backward_single_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(1, 2), Selection::new(10, 11)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(1, 2), Selection::new(10, 11)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(8, 9, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(1, 1), Selection::new(10, 10)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(1, 1), Selection::new(10, 10)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("dk\nsome\nhit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(8, 8, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn delete_backward_multi_char_with_multi_selection(){
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 1, 0), Selection::with_stored_line_position(6, 7, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.backspace(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("\nsome\nt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(0, 0, 0), Selection::with_stored_line_position(6, 6, 0)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn replace_same_len_with_multi_selection(){
        // redo replace (multi selection with replacement string same len as selected)
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("wut", semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 4, 3), Selection::with_stored_line_position(12, 13, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 4, 3), Selection::with_stored_line_position(12, 13, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("wut", semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 3, 3), Selection::with_stored_line_position(12, 12, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("wut\nsome\nwutt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(3, 3, 3), Selection::with_stored_line_position(12, 12, 3)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn replace_more_chars_with_multi_selection(){
        // redo replace (multi selection with replacement string more chars than selected)
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("shit", semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 4), Selection::with_stored_line_position(14, 15, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 5, 4), Selection::with_stored_line_position(14, 15, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("shit", semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 4), Selection::with_stored_line_position(14, 14, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("shit\nsome\nshitt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(4, 4, 4), Selection::with_stored_line_position(14, 14, 4)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn replace_less_chars_with_multi_selection(){
        // redo replace (multi selection with replacement string less chars than selected)
        let text = Rope::from("idk\nsome\nshit\n");

        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("x\nsome\nxt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(8, 9, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
        let _ = doc.undo(semantics);
        assert_eq!("idk\nsome\nshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(9, 12)], 0, &text), doc.selections());
        assert!(!doc.is_modified());
        let _ = doc.redo(semantics);
        assert_eq!("x\nsome\nxt\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(8, 9, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }

    #[test]
    fn undo_with_nothing_on_stack_errors(){
        let mut doc = Document::new(CursorSemantics::Bar);
        assert!(doc.undo(CursorSemantics::Bar).is_err());
    }

    #[test]
    fn redo_with_nothing_on_stack_errors(){
        let mut doc = Document::new(CursorSemantics::Bar);
        assert!(doc.redo(CursorSemantics::Bar).is_err());
    }

    ////////////////////////////////////////////////////////////////////// Insert ///////////////////////////////////////////////////////////////////////////
    #[test]
    fn idk_insert_single_char_with_multi_selection(){
        let text = Rope::from("some\nshit\n");
        
        let semantics = CursorSemantics::Block;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 2, 1), Selection::with_stored_line_position(7, 8, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());

        let semantics = CursorSemantics::Bar;
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(5, 5)], 0, &text));
        doc.insert_string("x", semantics);
        assert_eq!("xsome\nxshit\n", doc.text());
        assert_eq!(&Selections::new(vec![Selection::with_stored_line_position(1, 1, 1), Selection::with_stored_line_position(7, 7, 1)], 0, &text), doc.selections());
        assert!(doc.is_modified());
    }
    ////////////////////////////////////////////////////////////////////// Insert ///////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////// Delete ///////////////////////////////////////////////////////////////////////////
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
        assert!(delete_test("test1", Selection::new(14, 14), Selection::new(14, 14), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test1", Selection::new(14, 15), Selection::new(14, 15), Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block)); //idk\nsome\nshit\n|: >
    }
    #[test]
    fn delete_with_no_selection(){
        assert!(delete_test("test2", Selection::new(0, 0), Selection::with_stored_line_position(0, 0, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test2", Selection::new(0, 1), Selection::with_stored_line_position(0, 1, 0), Rope::from("dk\nsome\nshit\n"), CursorSemantics::Block));    //|:i>dk\nsome\nshit\n
    }
    #[test]
    fn delete_with_selection_head_greater_than_anchor(){
        assert!(delete_test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 0, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test3", Selection::new(0, 2), Selection::with_stored_line_position(0, 1, 0), Rope::from("k\nsome\nshit\n"), CursorSemantics::Block)); //|i:d>k\nsome\nshit\n
    }
    #[test]
    fn delete_with_selection_head_less_than_anchor(){
        assert!(delete_test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 1, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Bar));
        assert!(delete_test("test4", Selection::new(3, 1), Selection::with_stored_line_position(1, 2, 1), Rope::from("i\nsome\nshit\n"), CursorSemantics::Block));    //i|d:k>\nsome\nshit\n
    }
    #[test]
    fn delete_with_whole_text_selected(){
        assert!(delete_test("test5", Selection::new(0, 13), Selection::with_stored_line_position(0, 0, 0), Rope::from("\n"), CursorSemantics::Bar));  //just verifying...
        assert!(delete_test("test5", Selection::new(0, 14), Selection::with_stored_line_position(0, 0, 0), Rope::from(""), CursorSemantics::Bar));
        assert!(delete_test("test5", Selection::new(0, 15), Selection::with_stored_line_position(0, 1, 0), Rope::from(""), CursorSemantics::Block));  //|idk\nsome\nshit\n: >
    }
    #[test]
    fn delete_at_1_less_doc_end(){
        assert!(delete_test("test6", Selection::new(13, 13), Selection::with_stored_line_position(13, 13, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Bar));
        assert!(delete_test("test6", Selection::new(13, 14), Selection::with_stored_line_position(13, 14, 4), Rope::from("idk\nsome\nshit"), CursorSemantics::Block));  //idk\nsome\nshit|:\n> //idk\nsome\nshit|: >
    }
    ////////////////////////////////////////////////////////////////////// Delete ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////// Backspace ///////////////////////////////////////////////////////////////////////////
    fn backspace_test(name: &str, selection: Selection, expected: Rope, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        doc.backspace(semantics);
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
        use edit_core::document::TAB_WIDTH;
        let mut spaces = String::new();
        for _ in 0..TAB_WIDTH{
            spaces.push(' ');
        }
        let text = Rope::from(format!("{}idk\nsome\nshit\n", spaces));
        let semantics = CursorSemantics::Block; //test Bar too
        let selection = Selection::new(TAB_WIDTH, match semantics{CursorSemantics::Bar => TAB_WIDTH, CursorSemantics::Block => TAB_WIDTH.saturating_add(1)});
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        doc.backspace(semantics);
        assert!(doc.text().clone() == Rope::from("idk\nsome\nshit\n"));
        assert!(doc.selections().primary().clone() == Selection::with_stored_line_position(0, match semantics{CursorSemantics::Bar => 0, CursorSemantics::Block => 1}, 0));
    }
    ////////////////////////////////////////////////////////////////////// Backspace ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////// Cut ///////////////////////////////////////////////////////////////////////////
    //# use ropey::Rope;
    //# use edit_core::document::Document;
    //# use edit_core::selection::{Selection, Selections, CursorSemantics};
    
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
        assert!(cut_test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
        assert!(cut_test(Selection::new(4, 9), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    }

    #[test]
    fn cut_with_selection_anchor_greater_than_head(){
        assert!(cut_test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 4, 0), CursorSemantics::Bar));
        assert!(cut_test(Selection::new(9, 4), Rope::from("idk\nshit\n"), Selection::with_stored_line_position(4, 5, 0), CursorSemantics::Block));
    }

    #[test]
    fn cut_with_multiple_selections_returns_error(){
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(CursorSemantics::Bar).with_selections(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 7)], 0, &text));
        assert!(doc.cut(CursorSemantics::Bar).is_err());
    }
    ////////////////////////////////////////////////////////////////////// Cut ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////// Copy ///////////////////////////////////////////////////////////////////////////
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    
    fn copy_test(selection: Selection, expected: &str, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text));
        let _ = doc.copy();
        println!("expected: {:#?}\ngot: {:#?}\n", expected, doc.clipboard());
        doc.clipboard() == expected
    }
    
    #[test]
    fn copy_with_selection_anchor_less_than_head(){
        assert!(copy_test(Selection::new(4, 9), "some\n", CursorSemantics::Bar));
        assert!(copy_test(Selection::new(4, 9), "some\n", CursorSemantics::Block));    //idk\n|some:\n>shit\n
    }

    #[test]
    fn copy_with_multiple_selections_should_error(){
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(CursorSemantics::Bar).with_selections(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text));
        assert!(doc.copy().is_err());
    }
    ////////////////////////////////////////////////////////////////////// Copy ///////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////// Paste ///////////////////////////////////////////////////////////////////////////
    /// # use ropey::Rope;
    /// # use edit_core::document::Document;
    /// # use edit_core::selection::{Selection, Selections, CursorSemantics};
    
    fn paste_test(selection: Selection, string: &str, expected: Rope, expected_selection: Selection, semantics: CursorSemantics) -> bool{
        let text = Rope::from("idk\nsome\nshit\n");
        let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![selection], 0, &text)).with_clipboard(string.to_string());
        doc.paste(semantics);
        println!("expected: {:#?}\ngot: {:#?}\nexpected_position: {:#?}\ngot: {:#?}\n", expected, doc.text().clone(), expected_selection, doc.selections().primary().clone());
        doc.text().clone() == expected && doc.selections().primary().clone() == expected_selection
    }
    
    #[test]
    fn paste(){
        assert!(paste_test(Selection::new(9, 9), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 15, 0), CursorSemantics::Bar));
        assert!(paste_test(Selection::new(9, 10), "other\n", Rope::from("idk\nsome\nother\nshit\n"), Selection::with_stored_line_position(15, 16, 0), CursorSemantics::Block));
    }
    ////////////////////////////////////////////////////////////////////// Paste ///////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////// Search ///////////////////////////////////////////////////////////////////////////
    #[test]
    fn search_works(){
        let text = Rope::from("idk\nsome\nshit\nidk\n");
        let mut doc = Document::new(CursorSemantics::Block).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1)], 0, &text));
        doc.search("idk", CursorSemantics::Block);
        assert_eq!(&Selections::new(vec![Selection::new(0, 3), Selection::new(14, 17)], 0, &text), doc.selections());
    }

    #[test]
    fn maintains_current_selections_if_no_matching_substring(){
        let text = Rope::from("idk\nsome\nshit\nidk\n");
        let mut doc = Document::new(CursorSemantics::Block).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1)], 0, &text));
        doc.search("fuck", CursorSemantics::Block);
        assert_eq!(&Selections::new(vec![Selection::new(0, 1)], 0, &text), doc.selections());
    }
    ////////////////////////////////////////////////////////////////////// Search ///////////////////////////////////////////////////////////////////////////
