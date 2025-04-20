use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
    text_util,
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_potentially_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with cursor extended left to the nearest word boundary.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if selection.cursor(text, semantics) == 0{return Err(SelectionError::ResultsInSameState);}
    
    let goal_index = text_util::previous_word_boundary(selection.cursor(text, semantics), text);
    selection.put_cursor(goal_index, text, Movement::Extend, semantics, true)
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_word_boundary_backward;
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
        let result = extend_selection_word_boundary_backward::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_word_boundary_backward::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_valid_selections(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(2, 3), Direction::Forward),
                Selection::new(Range::new(7, 8), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0),
                Selection::with_stored_line_position(Range::new(4, 8), Direction::Backward, 0)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(7, 8), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 8), Direction::Backward, 0)
            ], 0
        );
    }
    
    #[test] fn extends_to_doc_start_if_no_other_word_boundaries(){
        test(
            CursorSemantics::Block, 
            "    idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0)], 0
        );
    }
    
    #[test] fn shrinks_previously_forward_extended_selection(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 10), Direction::Forward, 0)], 0
        );
    }
    
    #[test] fn errors_if_single_selection_at_doc_start(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        );
    }
    #[test] fn errors_if_already_extended_backwards_to_doc_start(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Backward)], 0
        );
    }

    //#[test] fn extend_left_word_boundary(){
    //    let text = Rope::from("use std::error::Error;");
    //    assert_eq!(Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0), Selection::new(Range::new(3, 4), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Block).unwrap());
    //    assert_eq!(Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0), Selection::new(Range::new(3, 3), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Bar).unwrap());
    //}
    //#[test] fn extend_left_word_boundary_error(){
    //    let text = Rope::from("idk\nsome\nshit\n");
    //    assert!(Selection::new(Range::new(0, 1), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Block).is_err());
    //    assert!(Selection::new(Range::new(0, 0), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Bar).is_err());
    //}
}