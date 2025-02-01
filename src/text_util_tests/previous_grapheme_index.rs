use ropey::Rope;
use crate::text_util;

#[test] fn with_ascii(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(0, text_util::previous_grapheme_index(1, &text));
}

#[test] fn with_ascii_if_at_doc_start_returns_current_index(){
    let text = Rope::from("idk\n");
    assert_eq!(0, text_util::previous_grapheme_index(0, &text));
}

#[ignore] #[test] fn with_utf8(){
    //assert!(false);
    unimplemented!()
}

#[ignore] #[test] fn with_utf8_if_at_doc_start_returns_current_index(){
    //assert!(false);
    unimplemented!()
}
