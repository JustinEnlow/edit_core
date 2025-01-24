use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn sanity_check(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(14, text.len_chars());
}

#[test] fn extend_right_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_right_word_boundary(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 3, 3), Selection::new(0, 0).extend_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
}

// bar
    #[test] fn normal_use_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert_eq!(Selection::with_stored_line_position(0, 3, 3), Selection::new(0, 0).extend_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
    }
    #[test] fn extends_to_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert_eq!(Selection::with_stored_line_position(13, 14, 0), Selection::new(13, 13).extend_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
    }
    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert!(Selection::new(14, 14).extend_right_word_boundary(&text, CursorSemantics::Bar).is_err());
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert!(Selection::new(0, 14).extend_right_word_boundary(&text, CursorSemantics::Bar).is_err());
    }
    #[test] fn errors_if_already_extended_backward_at_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert!(Selection::new(14, 0).extend_right_word_boundary(&text, CursorSemantics::Bar).is_err());
    }

// block
    #[test] fn normal_use_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_right_word_boundary(&text, CursorSemantics::Block).unwrap());
    }
    #[test] fn extends_to_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert_eq!(Selection::with_stored_line_position(12, 14, 4), Selection::new(12, 13).extend_right_word_boundary(&text, CursorSemantics::Block).unwrap());
    }
    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert!(Selection::new(13, 14).extend_right_word_boundary(&text, CursorSemantics::Block).is_err());
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert!(Selection::new(0, 14).extend_right_word_boundary(&text, CursorSemantics::Block).is_err());
    }
    #[test] fn errors_if_already_extended_backward_at_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        assert!(Selection::new(14, 0).extend_right_word_boundary(&text, CursorSemantics::Block).is_err());
    }
