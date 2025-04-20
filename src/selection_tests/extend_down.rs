//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test] fn sanity_check(){
//    let text = Rope::from("idk\nsomething\nelse\n");
//    assert_eq!(19, text.len_chars());
//}
//
//// bar
//    #[test] fn to_shorter_line_bar_semantics(){
//        let text = Rope::from("idk\nsomething\nelse\n");
//        //assert_eq!(Selection::with_stored_line_position(13, 18, 9), Selection::new(13, 13).extend_down(&text, CursorSemantics::Bar).unwrap());
//        assert_eq!(Selection::with_stored_line_position(Range::new(13, 18), Direction::Forward, 9), Selection::new(Range::new(13, 13), Direction::Forward).extend_down(&text, CursorSemantics::Bar).unwrap());
//    }
//    #[test] fn to_longer_line_bar_semantics(){
//        let text = Rope::from("idk\nsomething\nelse\n");
//        //assert_eq!(Selection::with_stored_line_position(3, 7, 3), Selection::new(3, 3).extend_down(&text, CursorSemantics::Bar).unwrap());
//        assert_eq!(Selection::with_stored_line_position(Range::new(3, 7), Direction::Forward, 3), Selection::new(Range::new(3, 3), Direction::Forward).extend_down(&text, CursorSemantics::Bar).unwrap());
//    }
//    #[test] fn extends_to_doc_text_end_bar_semantics(){
//        let text = Rope::from("idk\nsomething\nelse\n");
//        //assert_eq!(Selection::with_stored_line_position(18, 19, 4/*because slp isn't updated on vertical movements*/), Selection::new(18, 18).extend_down(&text, CursorSemantics::Bar).unwrap());
//        assert_eq!(Selection::with_stored_line_position(Range::new(18, 19), Direction::Forward, 4/*because slp isn't updated on vertical movements*/), Selection::new(Range::new(18, 18), Direction::Forward).extend_down(&text, CursorSemantics::Bar).unwrap());
//    }
//
//    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_bar_semantics(){
//        let text = Rope::from("idk\nsome\nshit\n");
//        //assert!(Selection::new(14, 14).extend_down(&text, CursorSemantics::Bar).is_err());
//        assert!(Selection::new(Range::new(14, 14), Direction::Forward).extend_down(&text, CursorSemantics::Bar).is_err());
//    }
//    #[test] fn errors_if_already_extended_forward_at_doc_text_end_bar_semantics(){
//        let text = Rope::from("idk\nsome\nshit\n");
//        //assert!(Selection::new(0, 14).extend_down(&text, CursorSemantics::Bar).is_err());
//        assert!(Selection::new(Range::new(0, 14), Direction::Forward).extend_down(&text, CursorSemantics::Bar).is_err());
//    }
//    #[test] fn errors_if_already_extended_backward_at_doc_text_end_bar_semantics(){
//        let text = Rope::from("idk\nsome\nshit\n");
//        //assert!(Selection::new(14, 0).extend_down(&text, CursorSemantics::Bar).is_err());
//        assert!(Selection::new(Range::new(0, 14), Direction::Backward).extend_down(&text, CursorSemantics::Bar).is_err());
//    }
//
//// block
//    #[test] fn to_shorter_line_block_semantics(){
//        let text = Rope::from("idk\nsomething\nelse\n");
//        //assert_eq!(Selection::with_stored_line_position(13, 19, 9), Selection::new(13, 14).extend_down(&text, CursorSemantics::Block).unwrap());
//        assert_eq!(Selection::with_stored_line_position(Range::new(13, 19), Direction::Forward, 9), Selection::new(Range::new(13, 14), Direction::Forward).extend_down(&text, CursorSemantics::Block).unwrap());
//    }
//    #[test] fn to_longer_line_block_semantics(){
//        let text = Rope::from("idk\nsomething\nelse\n");
//        //assert_eq!(Selection::with_stored_line_position(3, 8, 3), Selection::new(3, 4).extend_down(&text, CursorSemantics::Block).unwrap());
//        assert_eq!(Selection::with_stored_line_position(Range::new(3, 8), Direction::Forward, 3), Selection::new(Range::new(3, 4), Direction::Forward).extend_down(&text, CursorSemantics::Block).unwrap());
//    }
//
//    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_block_semantics(){
//        let text = Rope::from("idk\nsome\nshit\n");
//        //assert!(Selection::new(14, 14).extend_down(&text, CursorSemantics::Block).is_err());
//        assert!(Selection::new(Range::new(13, 14), Direction::Forward).extend_down(&text, CursorSemantics::Block).is_err());
//        assert!(Selection::new(Range::new(14, 15), Direction::Forward).extend_down(&text, CursorSemantics::Block).is_err());
//    }
//    #[test] fn errors_if_already_extended_forward_at_doc_text_end_block_semantics(){
//        let text = Rope::from("idk\nsome\nshit\n");
//        //assert!(Selection::new(0, 14).extend_down(&text, CursorSemantics::Block).is_err());
//        assert!(Selection::new(Range::new(0, 14), Direction::Forward).extend_down(&text, CursorSemantics::Block).is_err());
//    }
//    #[test] fn errors_if_already_extended_backward_at_doc_text_end_block_semantics(){
//        let text = Rope::from("idk\nsome\nshit\n");
//        //assert!(Selection::new(14, 0).extend_down(&text, CursorSemantics::Block).is_err());
//        assert!(Selection::new(Range::new(0, 14), Direction::Backward).extend_down(&text, CursorSemantics::Block).is_err());
//    }
//