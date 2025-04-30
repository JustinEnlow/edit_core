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
        selection::{Selection, CursorSemantics},
    };
    use ropey::Rope;

    //fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    let result = collapse_selections_to_cursor::document_impl(&mut doc, semantics);
    //    assert!(!result.is_err());
    //    let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
    //    assert_eq!(expected_selections, doc.selections);
    //    assert!(!doc.is_modified());
    //}
    //fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    assert!(collapse_selections_to_cursor::document_impl(&mut doc, semantics).is_err());
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
        let result = collapse_selections_to_cursor::document_impl(&mut doc, semantics);
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
        assert!(collapse_selections_to_cursor::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //TODO: should these functions really result in selections with a stored line position?...
    
    #[test] fn collapses_to_cursor_with_multiple_selections_with_selection_forward(){
        //test(
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 3), Direction::Forward),
        //        Selection::new(Range::new(4, 8), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2),
        //        Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3)
        //    ], 0, 
        //    CursorSemantics::Block
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 3, None),
                (4, 8, None)
            ], 0, 
            vec![
                (2, 3, Some(2)),
                (7, 8, Some(3))
            ], 0
        );
    }
    #[test] fn collapses_to_cursor_with_multiple_selections_with_selection_backward(){
        //test(
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 3), Direction::Backward),
        //        Selection::new(Range::new(4, 8), Direction::Backward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
        //        Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0)
        //    ], 0, 
        //    CursorSemantics::Block
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 0, None),
                (8, 4, None)
            ], 0, 
            vec![
                (0, 1, Some(0)),
                (4, 5, Some(0))
            ], 0
        );
    }
    
    #[test] fn collapses_to_cursor_with_mixed_extension(){
        //test(
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 8), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3)
        //    ], 0, 
        //    CursorSemantics::Block
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),
                (4, 8, None)
            ], 0, 
            vec![
                (0, 1, None),
                (7, 8, Some(3))
            ], 0
        );
    }
    
    #[test] fn errors_if_already_collapsed(){
        //test_error(
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 5), Direction::Forward)
        //    ], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),
                (4, 5, None)
            ], 0
        );
    }
    //maybe test above with single selection too...idk
}
