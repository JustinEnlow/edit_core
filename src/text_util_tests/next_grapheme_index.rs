use ropey::Rope;
use crate::text_util;

#[test] fn with_ascii(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(1, text_util::next_grapheme_index(0, &text));
}

#[test] fn with_ascii_if_at_doc_end_returns_current_index(){
    let text = Rope::from("idk\n"); //+ 1 for cursor on last line
    assert_eq!(5, text_util::next_grapheme_index(5, &text));
}

#[ignore] #[test] fn with_utf8(){
    //assert!(false);
    unimplemented!()
}

#[ignore] #[test] fn with_utf8_if_at_doc_end_returns_current_index(){
    //assert!(false);
    unimplemented!()
}
