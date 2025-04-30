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

/// Returns a new instance of [`Selection`] with the [`Selection`] extended to the right.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{    //TODO: ensure this can't extend past doc text end
    selection.assert_invariants(text, semantics);
    if selection.range.start == text.len_chars()
    || selection.range.end == text.len_chars()
    || selection.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}

    selection.move_horizontally(1, text, Movement::Extend, Direction::Forward, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_right;
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
    //    let result = extend_selection_right::document_impl(&mut doc, semantics);
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
    //    assert!(extend_selection_right::document_impl(&mut doc, semantics).is_err());
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
        let result = extend_selection_right::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_right::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //use ropey::Rope;
    //use crate::range::Range;
    //use crate::selection::{Selection, CursorSemantics, Direction};

    //#[test] fn sanity_check(){
    //    let text = Rope::from("idk\nsome\nshit\n");
    //    assert_eq!(14, text.len_chars());
    //}

// bar
    #[test] fn normal_use_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 1)], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None)
            ], 0, 
            vec![
                (0, 1, Some(1))
            ], 0
        );
    }
    #[test] fn normal_use_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(0, 2), Direction::Forward, 1)], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None)
            ], 0, 
            vec![
                (0, 2, Some(1))
            ], 0
        );
    }

    #[test] fn extends_to_doc_text_end_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(13, 13), Direction::Forward)], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 0)], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (13, 13, None)
            ], 0, vec![
                (13, 14, Some(0))
            ], 0
        );
    }
    #[test] fn extends_to_doc_text_end_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(12, 13), Direction::Forward)], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(12, 14), Direction::Forward, 4)], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (12, 13, None)
            ], 0, 
            vec![
                (12, 14, Some(4))
            ], 0
        );
    }

    //TODO:
    //#[test] fn with_previously_backward_extended_selection(){
    //    test(
    //        CursorSemantics::Bar, 
    //        "idk\nsome\nshit\n", 
    //        vec![Selection::new(Range::new(0, 14), Direction::Backward)], 0,
    //        vec![Selection::with_stored_line_position(Range::new(1, 14), Direction::Backward, 1)], 0
    //    );
    //}

    #[test] fn errors_if_cursor_at_doc_text_end_bar_semantics(){
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
    #[test] fn errors_if_cursor_at_doc_text_end_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (13, 14, None)
            ], 0
        );
    }

    #[test] fn errors_if_already_extended_forward_at_doc_text_end_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 14, None)
            ], 0
        );
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 14, None)
            ], 0
        );
    }
}
