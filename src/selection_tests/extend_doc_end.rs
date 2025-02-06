use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test] fn sanity_check(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(14, text.len_chars());
}

// bar
    #[test] fn extends_to_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).extend_doc_end(&text, CursorSemantics::Bar).unwrap());
        assert_eq!(Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 0), Selection::new(Range::new(0, 0), Direction::Forward).extend_doc_end(&text, CursorSemantics::Bar).unwrap());
    }

    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert!(Selection::new(14, 14).extend_doc_end(&text, CursorSemantics::Bar).is_err());
        assert!(Selection::new(Range::new(14, 14), Direction::Forward).extend_doc_end(&text, CursorSemantics::Bar).is_err());
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert!(Selection::new(0, 14).extend_doc_end(&text, CursorSemantics::Bar).is_err());
        assert!(Selection::new(Range::new(0, 14), Direction::Forward).extend_doc_end(&text, CursorSemantics::Bar).is_err());
    }
    #[test] fn errors_if_already_extended_backward_at_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert!(Selection::new(14, 0).extend_doc_end(&text, CursorSemantics::Bar).is_err());
        assert!(Selection::new(Range::new(0, 14), Direction::Backward).extend_doc_end(&text, CursorSemantics::Bar).is_err());
    }

// block
    #[test] fn extends_to_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert_eq!(Selection::with_stored_line_position(0, 14, 4), Selection::new(0, 0).extend_doc_end(&text, CursorSemantics::Block).unwrap());
        assert_eq!(Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4), Selection::new(Range::new(0, 1), Direction::Forward).extend_doc_end(&text, CursorSemantics::Block).unwrap());
    }
    
    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert!(Selection::new(13, 14).extend_doc_end(&text, CursorSemantics::Block).is_err());
        assert!(Selection::new(Range::new(13, 14), Direction::Forward).extend_doc_end(&text, CursorSemantics::Block).is_err());
        //assert!(Selection::new(14, 13).extend_doc_end(&text, CursorSemantics::Block).is_err());
        assert!(Selection::new(Range::new(13, 14), Direction::Backward).extend_doc_end(&text, CursorSemantics::Block).is_err());
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert!(Selection::new(0, 14).extend_doc_end(&text, CursorSemantics::Block).is_err());
        assert!(Selection::new(Range::new(0, 14), Direction::Forward).extend_doc_end(&text, CursorSemantics::Block).is_err());
    }
    #[test] fn errors_if_already_extended_backward_at_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        //assert!(Selection::new(14, 0).extend_doc_end(&text, CursorSemantics::Block).is_err());
        assert!(Selection::new(Range::new(0, 14), Direction::Backward).extend_doc_end(&text, CursorSemantics::Block).is_err());
    }
