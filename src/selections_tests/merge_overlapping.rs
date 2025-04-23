use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use crate::selections::Selections;

#[test] fn merge_overlapping_selections(){
    let text = Rope::from("idk\nsome\nshit\n");

    let selections = Selections::new(vec![
        Selection::new(Range::new(0, 2), Direction::Forward),    //[i d]k \n s o m e \n s h i t \n
        Selection::new(Range::new(1, 4), Direction::Forward),    // i[d k \n]s o m e \n s h i t \n
        Selection::new(Range::new(5, 7), Direction::Forward),    // i d k \n s[o m]e \n s h i t \n
        Selection::new(Range::new(8, 10), Direction::Forward),   // i d k \n s o m e[\n s]h i t \n
        Selection::new(Range::new(9, 12), Direction::Forward)    // i d k \n s o m e \n[s h i]t \n
    ], 4, &text, CursorSemantics::Bar);
    let expected_selections = Selections::new(vec![
        Selection::with_stored_line_position(Range::new(0, 4), Direction::Forward, 0),    //[i d k \n]s o m e \n s h i t \n
        Selection::new(Range::new(5, 7), Direction::Forward),    // i d k \n s[o m]e \n s h i t \n
        Selection::with_stored_line_position(Range::new(8, 12), Direction::Forward, 3)    // i d k \n s o m e[\n s h i]t \n
    ], 2, &text, CursorSemantics::Bar);
    assert_eq!(expected_selections, selections.merge_overlapping(&text, CursorSemantics::Bar).unwrap());

    //assert_eq!(
    //    Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text, CursorSemantics::Block), 
    //    Selections::new(
    //        vec![
    //            Selection::new(Range::new(0, 1), Direction::Forward), 
    //            Selection::new(Range::new(0, 1), Direction::Forward)
    //        ], 0, &text, CursorSemantics::Block).merge_overlapping(&text, CursorSemantics::Block).unwrap()
    //);
    assert_eq!( //unverified needed because selections are automerged with new()
        Selections::unverified(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0),
        Selections::unverified(
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward), 
                Selection::new(Range::new(0, 1), Direction::Forward)
            ], 0).merge_overlapping(&text, CursorSemantics::Block).unwrap()
    );
    //assert_eq!(
    //    Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Backward, 0)], 0, &text, CursorSemantics::Block), 
    //    Selections::new(
    //        vec![
    //            Selection::new(Range::new(0, 1), Direction::Backward), 
    //            Selection::new(Range::new(0, 14), Direction::Backward)
    //        ], 0, &text, CursorSemantics::Block).merge_overlapping(&text, CursorSemantics::Block).unwrap()
    //);
    assert_eq!( //unverified needed because selections are automerged with new()
        Selections::unverified(vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Backward, 0)], 0),
        Selections::unverified(
            vec![
                Selection::new(Range::new(0, 1), Direction::Backward), 
                Selection::new(Range::new(0, 14), Direction::Backward)
            ], 0).merge_overlapping(&text, CursorSemantics::Block).unwrap()
    );
    //assert_eq!(
    //    Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4)], 0, &text, CursorSemantics::Block), 
    //    Selections::new(
    //        vec![
    //            Selection::new(Range::new(0, 1), Direction::Forward), 
    //            Selection::new(Range::new(0, 14), Direction::Forward)
    //        ], 0, &text, CursorSemantics::Block).merge_overlapping(&text, CursorSemantics::Block).unwrap()
    //);
    assert_eq!( //unverified needed because selections are automerged with new()
        Selections::unverified(vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4)], 0),
        Selections::unverified(
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward), 
                Selection::new(Range::new(0, 14), Direction::Forward)
            ], 0).merge_overlapping(&text, CursorSemantics::Block).unwrap()
    );
}

#[test] fn error_if_single_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text, CursorSemantics::Bar);
    assert!(selections.merge_overlapping(&text, CursorSemantics::Bar).is_err());
}

#[test] fn no_change_if_no_selections_overlap(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text, CursorSemantics::Block);
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text, CursorSemantics::Block), selections.merge_overlapping(&text, CursorSemantics::Block).unwrap());
}
