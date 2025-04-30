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

/// Returns a new instance of [`Selection`] with cursor moved left.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if !selection.is_extended(semantics) && selection.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
    selection.move_horizontally(1, text, Movement::Move, Direction::Backward, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_left;
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
    //    let result = move_cursor_left::document_impl(&mut doc, semantics);
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
    //    assert!(move_cursor_left::document_impl(&mut doc, semantics).is_err());
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
        let result = move_cursor_left::document_impl(&mut doc, semantics);
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
        assert!(move_cursor_left::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_valid_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(1, 1), Direction::Forward),   //common use
        //        Selection::new(Range::new(4, 4), Direction::Forward),   //line to line updates stored line position
        //        Selection::new(Range::new(10, 13), Direction::Forward), //extended selection collapses to cursor then does regular move
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0),
        //        Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3),
        //        Selection::with_stored_line_position(Range::new(12, 12), Direction::Forward, 3),
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (1, 1, None),   //common use
                (4, 4, None),   //line to line updates stored line position
                (10, 13, None)  //extended selection collapses to cursor then does regular move
            ], 0, 
            vec![
                (0, 0, Some(0)),
                (3, 3, Some(3)),
                (12, 12, Some(3))
            ], 0
        );
    }
    #[test] fn with_multiple_valid_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(1, 2), Direction::Forward),   //common use
        //        Selection::new(Range::new(4, 5), Direction::Forward),   //line to line updates stored line position
        //        Selection::new(Range::new(10, 13), Direction::Forward), //extended selection collapses to cursor then does regular move
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
        //        Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3),
        //        Selection::with_stored_line_position(Range::new(11, 12), Direction::Forward, 2),
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (1, 2, None),   //common use
                (4, 5, None),   //line to line updates stored line position
                (10, 13, None)  //extended selection collapses to cursor then does regular move
            ], 0, 
            vec![
                (0, 1, Some(0)),
                (3, 4, Some(3)),
                (11, 12, Some(2))
            ], 0
        );
    }
    
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),   //invalid
        //        Selection::new(Range::new(4, 4), Direction::Forward),   //valid
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3),
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None),   //invalid
                (4, 4, None)    //valid
            ], 0, 
            vec![
                (0, 0, None),
                (3, 3, Some(3))
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),   //invalid
        //        Selection::new(Range::new(4, 5), Direction::Forward),   //valid
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3),
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),   //invalid
                (4, 5, None)    //valid
            ], 0, 
            vec![
                (0, 1, None),
                (3, 4, Some(3))
            ], 0
        );
    }
    
    #[test] fn errors_if_single_selection_at_doc_start_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None)
            ], 0
        );
    }
    #[test] fn errors_if_single_selection_at_doc_start_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None)
            ], 0
        );
    }
}
