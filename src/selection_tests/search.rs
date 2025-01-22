use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn sanity_check(){
    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
    let selection = Selection::new(0, 34);
    let selection_text = &text.to_string()[selection.range.start..selection.range.end];
    println!("{:#?}", selection_text);
    assert_eq!(text.to_string(), selection_text);
}

#[test] fn returns_matched_search_input(){
    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
    let selection = Selection::new(0, 34);
    for selection in selection.search("shit", &text){
        println!("{}\n", selection.debug(&text, CursorSemantics::Block));
    }
    assert_eq!(vec![Selection::new(9, 13), Selection::new(29, 33)], selection.search("shit", &text));
}

#[test] fn returns_empty_vec_if_no_match(){
    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
    let selection = Selection::new(0, 34);
    assert!(selection.search("fuck", &text).is_empty());
}
