use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test] fn sanity_check(){
    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
    //let selection = Selection::new(0, 34);
    let selection = Selection::new(Range::new(0, 34), Direction::Forward);
    let selection_text = &text.to_string()[selection.range.start..selection.range.end];
    println!("{:#?}", selection_text);
    assert_eq!(text.to_string(), selection_text);
}

#[test] fn returns_matched_search_input(){
    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
    //let selection = Selection::new(0, 34);
    let selection = Selection::new(Range::new(0, 34), Direction::Forward);
    for selection in selection.search("shit", &text){
        println!("{}\n", selection.debug(&text, CursorSemantics::Block));
    }
    //assert_eq!(vec![Selection::new(9, 13), Selection::new(29, 33)], selection.search("shit", &text));
    assert_eq!(vec![Selection::new(Range::new(9, 13), Direction::Forward), Selection::new(Range::new(29, 33), Direction::Forward)], selection.search("shit", &text));
}

#[test] fn returns_empty_vec_if_no_match(){
    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
    //let selection = Selection::new(0, 34);
    let selection = Selection::new(Range::new(0, 34), Direction::Forward);
    assert!(selection.search("fuck", &text).is_empty());
}
