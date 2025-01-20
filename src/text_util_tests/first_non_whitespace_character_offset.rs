use ropey::Rope;
use crate::text_util;

#[test] fn with_leading_whitespace(){
    let text = Rope::from("   idk\n");
    assert!(text_util::first_non_whitespace_character_offset(text.slice(..)) == 3);
}

#[test] fn with_empty_line(){
    let text = Rope::from("");
    assert!(text_util::first_non_whitespace_character_offset(text.slice(..)) == 0);
}

#[test] fn with_line_containing_only_whitespace(){
    let text = Rope::from("   ");
    assert!(text_util::first_non_whitespace_character_offset(text.slice(..)) == 0);
}
