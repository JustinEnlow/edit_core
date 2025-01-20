use crate::text_util;
use ropey::RopeSlice;

#[test] fn with_line_containing_only_spaces(){
    let text = RopeSlice::from("    ");
    assert!(text_util::slice_is_all_spaces(text));
}

#[test] fn with_line_containing_non_space_chars(){
    let text = RopeSlice::from(" idk ");
    assert!(!text_util::slice_is_all_spaces(text));
}
