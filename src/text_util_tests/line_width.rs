use ropey::Rope;
use crate::text_util;

#[test] fn idk(){
    let text = Rope::from("idk\n");
    assert!(text_util::line_width(text.slice(..), false) == 3);
    assert!(text_util::line_width(text.slice(..), true) == 4);
}

//let text = Rope::from("idk\n\n");
//assert!(text_util::line_width(text.slice(..), false) == 3);
//assert!(text_util::line_width(text.slice(..), true) == 4);
