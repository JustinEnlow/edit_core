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

/// Returns a new instance of [`Selection`] with cursor moved up.
fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if text.char_to_line(selection.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
    selection.move_vertically(1, text, Movement::Move, Direction::Backward, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_up;
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
        let result = move_cursor_up::document_impl(&mut doc, semantics);
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
        assert!(move_cursor_up::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn to_shorter_line_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshits\n", 
            vec![
                Selection::new(Range::new(8, 8), Direction::Forward),
                Selection::new(Range::new(14, 14), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 4),  //notice this maintains stored line position of selection before operation
                Selection::with_stored_line_position(Range::new(8, 8), Direction::Forward, 5)   //notice this maintains stored line position of selection before operation
            ], 0
        );
    }
    #[test] fn to_shorter_line_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshits\n", 
            vec![
                Selection::new(Range::new(8, 9), Direction::Forward),
                Selection::new(Range::new(14, 15), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(8, 9), Direction::Forward, 5)
            ], 0
        );
    }
    
    #[test] fn to_line_with_equal_len_or_more_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idfk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(9, 9), Direction::Forward),
                Selection::new(Range::new(14, 14), Direction::Forward),
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(9, 9), Direction::Forward, 4)
            ], 0
        );
    }
    #[test] fn to_line_with_equal_len_or_more_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idfk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(9, 10), Direction::Forward),
                Selection::new(Range::new(14, 15), Direction::Forward),
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 4)
            ], 0
        );
    }

    //with mixed valid and invalid selections   //one on top line, one not
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::new(Range::new(9, 9), Direction::Forward)
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
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(9, 10), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0)
            ], 0
        );
    }

    //merges overlapping resultant selections   //one on top line, one on second
    #[test] fn merges_overlapping_resultant_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::new(Range::new(4, 4), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)
            ], 0
        );
    }
    #[test] fn merges_overlapping_resultant_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)
            ], 0
        );
    }
    
    //with extended selections collapses
    #[test] fn with_extended_selection_collapses_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(4, 8), Direction::Forward),
                Selection::new(Range::new(9, 13), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(8, 8), Direction::Forward, 4)
            ], 0
        );
    }
    #[test] fn with_extended_selection_collapses_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(4, 9), Direction::Forward),
                Selection::new(Range::new(9, 14), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 4),
                Selection::with_stored_line_position(Range::new(8, 9), Direction::Forward, 4)
            ], 0
        );
    }
    
    #[test] fn errors_if_single_selection_on_topmost_line_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0
        );
    }
    #[test] fn errors_if_single_selection_on_topmost_line_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        );
    }
}
