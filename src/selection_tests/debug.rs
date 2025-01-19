use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn returns_proper_string_when_head_greater_than_anchor_bar_semantics(){
    let text = Rope::from("idk some shit\n");
    let selection = Selection::new(0, 5);
    assert_eq!("[|idk s>]ome shit\n".to_string(), selection.debug(&text, CursorSemantics::Bar));
}
#[test] fn returns_proper_string_when_head_greater_than_anchor_block_semantics(){
    let text = Rope::from("idk some shit\n");
    let selection = Selection::new(0, 5);
    assert_eq!("[|idk :s>]ome shit\n".to_string(), selection.debug(&text, CursorSemantics::Block));
}

#[test] fn returns_proper_string_when_head_less_than_anchor_bar_semantics(){
    let text = Rope::from("idk some shit\n");
    let selection = Selection::new(5, 0);
    assert_eq!("[<idk s|]ome shit\n".to_string(), selection.debug(&text, CursorSemantics::Bar));
}
#[test] fn returns_proper_string_when_head_less_than_anchor_block_semantics(){
    let text = Rope::from("idk some shit\n");
    let selection = Selection::new(5, 0);
    assert_eq!("[:<idk s|]ome shit\n".to_string(), selection.debug(&text, CursorSemantics::Block));
}

#[test] fn returns_proper_string_when_head_equals_anchor_bar_semantics(){
    let text = Rope::from("idk some shit\n");
    let selection = Selection::new(0, 0);
    assert_eq!("[|>]idk some shit\n".to_string(), selection.debug(&text, CursorSemantics::Bar));
}
//TODO: make this return an actual error, instead of a magic string
#[test] fn errors_when_head_equals_anchor_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(0, 0);
    assert_eq!("Selection head and anchor should not be equal using Block semantics.".to_string(), selection.debug(&text, CursorSemantics::Block));
}

#[test] fn returns_proper_string_when_cursor_past_end_of_text_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(14, 14);
    assert_eq!("idk\nsome\nshit\n[|>]".to_string(), selection.debug(&text, CursorSemantics::Bar));
}
#[test] fn returns_proper_string_when_cursor_past_end_of_text_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(14, 15);
    assert_eq!("idk\nsome\nshit\n[|:>]".to_string(), selection.debug(&text, CursorSemantics::Block));
}
