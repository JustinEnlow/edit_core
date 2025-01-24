use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};
use crate::view::View;

#[test] fn sanity_check(){
    let text = Rope::from("idk\nsomething\nelse\n");
    assert_eq!(19, text.len_chars());
}

#[test]
fn extend_page_down(){
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::with_stored_line_position(0, 4, 0), Selection::new(0, 0).extend_page_down(&text, &client_view, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 5, 0), Selection::new(0, 1).extend_page_down(&text, &client_view, CursorSemantics::Block).unwrap());  //[i]dk\nsomething\nelse    //[idk\n:s]omething\nelse
}

// bar
    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        let client_view = View::new(0, 0, 2, 2);
        assert!(Selection::new(14, 14).extend_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        let client_view = View::new(0, 0, 2, 2);
        assert!(Selection::new(0, 14).extend_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    }
    #[test] fn errors_if_already_extended_backward_at_doc_text_end_bar_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        let client_view = View::new(0, 0, 2, 2);
        assert!(Selection::new(14, 0).extend_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    }

// block
    #[test] fn errors_if_cursor_at_doc_end_or_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        let client_view = View::new(0, 0, 2, 2);
        assert!(Selection::new(14, 14).extend_page_down(&text, &client_view, CursorSemantics::Block).is_err());
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        let client_view = View::new(0, 0, 2, 2);
        assert!(Selection::new(0, 14).extend_page_down(&text, &client_view, CursorSemantics::Block).is_err());
    }
    #[test] fn errors_if_already_extended_backward_at_doc_text_end_block_semantics(){
        let text = Rope::from("idk\nsome\nshit\n");
        let client_view = View::new(0, 0, 2, 2);
        assert!(Selection::new(14, 0).extend_page_down(&text, &client_view, CursorSemantics::Block).is_err());
    }
