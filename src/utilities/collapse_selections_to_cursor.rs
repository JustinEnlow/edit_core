use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
};
use ropey::Rope;

//TODO: rename to collapse_selections_to_cursor
pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_non_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

//TODO: we should allow collapsing to anchor, or collapse to anchor collapse(&self, text: &Rope, semantics: CursorSemantics, collapse_target: Anchor)
/// Returns a new instance of [`Selection`] with `anchor` aligned with cursor.
fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    //selection.assert_invariants(text, semantics);
    if !selection.is_extended(semantics){return Err(SelectionError::ResultsInSameState);}
    selection.put_cursor(selection.cursor(text, semantics), text, Movement::Move, semantics, true)
    //if we want collapse to anchor:
    //self.put_cursor(self.anchor, text, Movement::Move, semantics, true)
}

#[cfg(test)]
mod tests{
    use crate::utilities::collapse_selections_to_cursor;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let result = collapse_selections_to_cursor::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(collapse_selections_to_cursor::document_impl(&mut doc, semantics).is_err());
    }

    //TODO: should these functions really result in selections with a stored line position?...
    
    #[test] fn collapses_to_cursor_with_multiple_selections_with_selection_forward(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 8), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2),
                Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3)
            ], 0, 
            CursorSemantics::Block
        );
    }
    #[test] fn collapses_to_cursor_with_multiple_selections_with_selection_backward(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Backward),
                Selection::new(Range::new(4, 8), Direction::Backward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0)
            ], 0, 
            CursorSemantics::Block
        );
    }
    
    #[test] fn collapses_to_cursor_with_mixed_extension(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 8), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3)
            ], 0, 
            CursorSemantics::Block
        );
    }
    
    #[test] fn errors_if_already_collapsed(){
        test_error(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            CursorSemantics::Block
        );
    }
    //maybe test above with single selection too...idk
}
