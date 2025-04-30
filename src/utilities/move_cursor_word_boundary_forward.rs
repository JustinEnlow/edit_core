use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
    text_util
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_potentially_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with cursor moved right to the nearest word boundary.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if selection.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
    
    let goal_index = text_util::next_word_boundary(selection.head(), text);
    match semantics{
        CursorSemantics::Bar => {
            selection.put_cursor(goal_index, text, Movement::Move, semantics, true)
        }
        CursorSemantics::Block => {
            if goal_index == text.len_chars(){
                selection.put_cursor(goal_index, text, Movement::Move, semantics, true)
            }else{
                selection.put_cursor(text_util::previous_grapheme_index(goal_index, text), text, Movement::Move, semantics, true)
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_word_boundary_forward;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics},
    };
    use ropey::Rope;

    //fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    let result = move_cursor_word_boundary_forward::document_impl(&mut doc, semantics);
    //    assert!(!result.is_err());
    //    let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
    //    assert_eq!(expected_selections, doc.selections);
    //    assert!(!doc.is_modified());
    //}
    //fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    assert!(move_cursor_word_boundary_forward::document_impl(&mut doc, semantics).is_err());
    //    assert!(!doc.is_modified());
    //}
    fn test(semantics: CursorSemantics, text: &str, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize, tuple_expected_selections: Vec<(usize, usize, Option<usize>)>, expected_primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections);
        let result = move_cursor_word_boundary_forward::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let mut vec_expected_selections = Vec::new();
        for tuple in tuple_expected_selections{
            vec_expected_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let expected_selections = Selections::new(vec_expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections);
        assert!(move_cursor_word_boundary_forward::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_valid_selections_bar_semantics(){
        //                    1                   2
        //0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // u s e _ e r r o r : : E r r o r ; _ _ _ _
        //test(
        //    CursorSemantics::Bar, 
        //    "use error::Error;    ",    //len 21    text end: (21, 21)    doc end: (21, 21)
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),   //common use
        //        Selection::new(Range::new(3, 3), Direction::Forward),   //skips whitespace and moves to next ending word boundary
        //        Selection::new(Range::new(9, 9), Direction::Forward),   //non alpha_numeric or whitespace jumps to next non whitespace
        //        Selection::new(Range::new(11, 16), Direction::Forward), //extended collapses then moves normally
        //        Selection::new(Range::new(17, 17), Direction::Forward)  //skips whitespace and moves to doc end if no other alphanumeric
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3),
        //        Selection::with_stored_line_position(Range::new(9, 9), Direction::Forward, 9),
        //        Selection::with_stored_line_position(Range::new(10, 10), Direction::Forward, 10),
        //        Selection::with_stored_line_position(Range::new(17, 17), Direction::Forward, 17),
        //        Selection::with_stored_line_position(Range::new(21, 21), Direction::Forward, 21),
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "use error::Error;    ",    //len 21    text end: (21, 21)    doc end: (21, 21)
            vec![
                (0, 0, None),   //common use
                (3, 3, None),   //skips whitespace and moves to next ending word boundary
                (9, 9, None),   //non alpha_numeric or whitespace jumps to next non whitespace
                (11, 16, None), //extended collapses then moves normally
                (17, 17, None)  //skips whitespace and moves to doc end if no other alphanumeric
            ], 0, 
            vec![
                (3, 3, Some(3)),
                (9, 9, Some(9)),
                (10, 10, Some(10)),
                (17, 17, Some(17)),
                (21, 21, Some(21))
            ], 0
        );
    }
    #[test] fn with_multiple_valid_selections_block_semantics(){
        //                    1                   2
        //0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // u s e _ e r r o r : : E r r o r ; _ _ _ _
        //test(
        //    CursorSemantics::Block, 
        //    "use error::Error;    ",    //len 21    text end: (20, 21)    doc end: (21, 22)
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),   //common use
        //        Selection::new(Range::new(2, 3), Direction::Forward),   //skips whitespace and moves to next ending word boundary
        //        Selection::new(Range::new(8, 9), Direction::Forward),   //non alpha_numeric or whitespace jumps to next non whitespace
        //        Selection::new(Range::new(11, 16), Direction::Forward), //extended collapses then moves normally
        //        Selection::new(Range::new(16, 17), Direction::Forward)  //skips whitespace and moves to doc end if no other alphanumeric
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2),
        //        Selection::with_stored_line_position(Range::new(8, 9), Direction::Forward, 8),
        //        Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 9),
        //        Selection::with_stored_line_position(Range::new(16, 17), Direction::Forward, 16),
        //        Selection::with_stored_line_position(Range::new(21, 22), Direction::Forward, 21),
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "use error::Error;    ",    //len 21    text end: (20, 21)    doc end: (21, 22)
            vec![
                (0, 1, None),   //common use
                (2, 3, None),   //skips whitespace and moves to next ending word boundary
                (8, 9, None),   //non alpha_numeric or whitespace jumps to next non whitespace
                (11, 16, None), //extended collapses then moves normally
                (16, 17, None)  //skips whitespace and moves to doc end if no other alphanumeric
            ], 0, 
            vec![
                (2, 3, Some(2)),
                (8, 9, Some(8)),
                (9, 10, Some(9)),
                (16, 17, Some(16)),
                (21, 22, Some(21))
            ], 0
        );
    }
    
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(3, 3), Direction::Forward),   //valid + line to line updates stored line position
        //        Selection::new(Range::new(14, 14), Direction::Forward)  //invalid
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(8, 8), Direction::Forward, 4),
        //        Selection::new(Range::new(14, 14), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 3, None),   //valid + line to line updates stored line position
                (14, 14, None)  //invalid
            ], 0, 
            vec![
                (8, 8, Some(4)),
                (14, 14, None)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(3, 4), Direction::Forward),   //valid + line to line updates stored line position
        //        Selection::new(Range::new(14, 15), Direction::Forward)  //invalid
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3),
        //        Selection::new(Range::new(14, 15), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 4, None),   //valid + line to line updates stored line position
                (14, 15, None)  //invalid
            ], 0, 
            vec![
                (7, 8, Some(3)),
                (14, 15, None)
            ], 0
        );
    }
    
    #[test] fn errors_when_single_selection_at_doc_end_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (14, 14, None)
            ], 0
        );
    }
    #[test] fn errors_when_single_selection_at_doc_end_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (14, 15, None)
            ], 0
        );
    }
}
