use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_potentially_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with the cursor moved to the end of the document.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if selection.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
    selection.put_cursor(text.len_chars(), text, Movement::Move, semantics, true)
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_document_end;
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
        let result = move_cursor_document_end::document_impl(&mut doc, semantics);
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
        assert!(move_cursor_document_end::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_valid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::new(Range::new(4, 4), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)
            ], 0
        );
    }
    #[test] fn with_multiple_valid_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)
            ], 0
        );
    }
    
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),   //valid
                Selection::new(Range::new(14, 14), Direction::Forward)  //invalid
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),   //valid
                Selection::new(Range::new(14, 15), Direction::Forward)  //invalid
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)
            ], 0
        );
    }
    
    #[test] fn errors_when_single_selection_at_doc_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0
        );
    }
    #[test] fn errors_when_single_selection_at_doc_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0
        );
    }
}
