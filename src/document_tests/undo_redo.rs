use ropey::Rope;
use crate::document::Document;
use crate::selection::{Selection, CursorSemantics};
use crate::selections::Selections;



#[test]
fn insert_single_char_with_multi_selection(){
    let text = Rope::from("some\nshit\n");
        
    let semantics = CursorSemantics::Block;
    let mut doc = Document::new(semantics).with_text(text.clone()).with_selections(Selections::new(vec![Selection::new(0, 1), Selection::new(5, 6)], 0, &text));
    let _ = doc.insert_string("x", semantics);
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
    let _ = doc.insert_string("x", semantics);
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
        let _ = doc.insert_string("idk\n", semantics);
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
        let _ = doc.insert_string("idk\n", semantics);
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
        let _ = doc.delete(semantics);
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
        let _ = doc.delete(semantics);
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
        let _ = doc.delete(semantics);
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
        let _ = doc.delete(semantics);
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
        let _ = doc.backspace(semantics);
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
        let _ = doc.backspace(semantics);
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
        let _ = doc.backspace(semantics);
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
        let _ = doc.backspace(semantics);
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
        let _ = doc.insert_string("wut", semantics);
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
        let _ = doc.insert_string("wut", semantics);
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
        let _ = doc.insert_string("shit", semantics);
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
        let _ = doc.insert_string("shit", semantics);
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
        let _ = doc.insert_string("x", semantics);
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
