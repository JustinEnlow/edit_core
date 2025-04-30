use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Direction, Movement},
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_potentially_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with cursor moved down.
fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if text.char_to_line(selection.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
    selection.move_vertically(1, text, Movement::Move, Direction::Forward, semantics)
}



#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_down;
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
    //    let result = move_cursor_down::document_impl(&mut doc, semantics);
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
    //    assert!(move_cursor_down::document_impl(&mut doc, semantics).is_err());
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
        let result = move_cursor_down::document_impl(&mut doc, semantics);
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
        assert!(move_cursor_down::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //to shorter line
    #[test] fn to_shorter_line_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "shits\nsome\nidk", 
        //    vec![
        //        Selection::new(Range::new(5, 5), Direction::Forward),
        //        Selection::new(Range::new(10, 10), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(10, 10), Direction::Forward, 5),  //notice this maintains stored line position of selection before operation
        //        Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 4)   //notice this maintains stored line position of selection before operation
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "shits\nsome\nidk", 
            vec![
                (5, 5, None),
                (10, 10, None)
            ], 0, 
            vec![
                (10, 10, Some(5)),  //notice this maintains stored line position of selection before operation
                (14, 14, Some(4))   //notice this maintains stored line position of selection before operation
            ], 0
        );
    }
    #[test] fn to_shorter_line_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "shits\nsome\nidk", 
        //    vec![
        //        Selection::new(Range::new(5, 6), Direction::Forward),
        //        Selection::new(Range::new(10, 11), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(10, 11), Direction::Forward, 5),  //notice this maintains stored line position of selection before operation
        //        Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 4)   //notice this maintains stored line position of selection before operation
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "shits\nsome\nidk", 
            vec![
                (5, 6, None),
                (10, 11, None)
            ], 0, 
            vec![
                (10, 11, Some(5)),  //notice this maintains stored line position of selection before operation
                (14, 15, Some(4))   //notice this maintains stored line position of selection before operation
            ], 0
        );
    }

    //to line with equal len or more
    #[test] fn to_line_with_equal_len_or_more_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "some\nshit\nidfk\n", 
        //    vec![
        //        Selection::new(Range::new(4, 4), Direction::Forward),
        //        Selection::new(Range::new(9, 9), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(9, 9), Direction::Forward, 4),
        //        Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 4)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "some\nshit\nidfk\n", 
            vec![
                (4, 4, None),
                (9, 9, None)
            ], 0, 
            vec![
                (9, 9, Some(4)),
                (14, 14, Some(4))
            ], 0
        );
    }
    #[test] fn to_line_with_equal_len_or_more_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "some\nshit\nidfk\n", 
        //    vec![
        //        Selection::new(Range::new(4, 5), Direction::Forward),
        //        Selection::new(Range::new(9, 10), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 4),
        //        Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 4)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "some\nshit\nidfk\n", 
            vec![
                (4, 5, None),
                (9, 10, None)
            ], 0, 
            vec![
                (9, 10, Some(4)),
                (14, 15, Some(4))
            ], 0
        );
    }
    
    //with mixed valid and invalid selections   //one on bottom line, one not
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(4, 4), Direction::Forward),
        //        Selection::new(Range::new(14, 14), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(9, 9), Direction::Forward, 0),
        //        Selection::new(Range::new(14, 14), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (4, 4, None),
                (14, 14, None)
            ], 0, 
            vec![
                (9, 9, Some(0)),
                (14, 14, None)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(4, 5), Direction::Forward),
        //        Selection::new(Range::new(14, 15), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 0),
        //        Selection::new(Range::new(14, 15), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (4, 5, None),
                (14, 15, None)
            ], 0, 
            vec![
                (9, 10, Some(0)),
                (14, 15, None)
            ], 0
        );
    }
    
    //merges overlapping resultant selections   //one on bottom line, one on second
    #[test] fn merges_overlapping_resultant_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(9, 9), Direction::Forward),
        //        Selection::new(Range::new(14, 14), Direction::Forward)
        //    ], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (9, 9, None),
                (14, 14, None)
            ], 0, 
            vec![
                (14, 14, Some(0))
            ], 0
        );
    }
    #[test] fn merges_overlapping_resultant_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(9, 10), Direction::Forward),
        //        Selection::new(Range::new(14, 15), Direction::Forward)
        //    ], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (9, 10, None),
                (14, 15, None)
            ], 0, 
            vec![
                (14, 15, Some(0))
            ], 0
        );
    }
    
    //with extended selections collapses
    #[test] fn with_extended_selection_collapses_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 3), Direction::Forward),
        //        Selection::new(Range::new(4, 8), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(7, 7), Direction::Forward, 3),
        //        Selection::with_stored_line_position(Range::new(13, 13), Direction::Forward, 4),
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 3, None),
                (4, 8, None)
            ], 0, 
            vec![
                (7, 7, Some(3)),
                (13, 13, Some(4))
            ], 0
        );
    }
    #[test] fn with_extended_selection_collapses_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 4), Direction::Forward),
        //        Selection::new(Range::new(4, 9), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3),
        //        Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 4),
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 4, None),
                (4, 9, None)
            ], 0, 
            vec![
                (7, 8, Some(3)),
                (13, 14, Some(4))
            ], 0
        );
    }
    
    //errors if single selection on bottom-most line
    #[test] fn errors_if_single_selection_on_bottommost_line_bar_semantics(){
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
    #[test] fn errors_if_single_selection_on_bottommost_line_block_semantics(){
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
