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

/// Returns a new instance of [`Selection`] with cursor moved left to the nearest word boundary.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if selection.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
    
    let goal_index = text_util::previous_word_boundary(selection.cursor(text, semantics), text);
    selection.put_cursor(goal_index, text, Movement::Move, semantics, true)
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_word_boundary_backward;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let result = move_cursor_word_boundary_backward::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(move_cursor_word_boundary_backward::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_valid_selections_bar_semantics(){
        //                    1                   2
        //0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // _ _ _ _ u s e _ e r r o r : : E r r o r ;
        test(
            CursorSemantics::Bar, 
            "    use error::Error;",    //len 21    text end: (21, 21)  doc end: (21, 21)
            vec![
                Selection::new(Range::new(4, 4), Direction::Forward),   //skips whitespace and moves to doc start if no other alphanumeric
                Selection::new(Range::new(8, 8), Direction::Forward),   //skips whitespace and moves to next starting word boundary
                Selection::new(Range::new(14, 14), Direction::Forward), //non alpha_numeric or whitespace jumps to previous non whitespace
                Selection::new(Range::new(15, 20), Direction::Backward),//extended collapses then moves normally
                Selection::new(Range::new(21, 21), Direction::Forward)  //common use
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(13, 13), Direction::Forward, 13),
                Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 14),
                Selection::with_stored_line_position(Range::new(20, 20), Direction::Forward, 20),
            ], 0
        );
    }
    #[test] fn with_multiple_valid_selections_block_semantics(){
        //                    1                   2
        //0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // _ _ _ _ u s e _ e r r o r : : E r r o r ;
        test(
            CursorSemantics::Block, 
            "    use error::Error;",    //len 21    text end: (20, 21)  doc end: (21, 22)
            vec![
                Selection::new(Range::new(4, 5), Direction::Forward),   //skips whitespace and moves to doc start if no other alphanumeric
                Selection::new(Range::new(8, 9), Direction::Forward),   //skips whitespace and moves to next starting word boundary
                Selection::new(Range::new(14, 15), Direction::Forward), //non alpha_numeric or whitespace jumps to previous non whitespace
                Selection::new(Range::new(15, 20), Direction::Backward),//extended collapses then moves normally
                Selection::new(Range::new(21, 22), Direction::Forward)  //common use
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 13),
                Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 14),
                Selection::with_stored_line_position(Range::new(20, 21), Direction::Forward, 20),
            ], 0
        );
    }

    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),   //invalid
                Selection::new(Range::new(9, 9), Direction::Forward)    //valid + line to line updates stored line position
            ], 0, 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),   //invalid
                Selection::new(Range::new(9, 10), Direction::Forward)    //valid + line to line updates stored line position
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0)
            ], 0
        );
    }

    #[test] fn errors_when_single_selection_at_doc_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0
        );
    }
    #[test] fn errors_when_single_selection_at_doc_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        );
    }
}
