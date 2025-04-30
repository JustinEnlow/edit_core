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

/// Returns a new instance of [`Selection`] with the [`Selection`] extended to the start of the text on the current line.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    let line_number = text.char_to_line(selection.cursor(text, semantics));
    let line_start = text.line_to_char(line_number);
    let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
    let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

    if selection.cursor(text, semantics) == text_start{return Err(SelectionError::ResultsInSameState);}
    selection.put_cursor(text_start, text, Movement::Extend, semantics, true)
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_line_text_start;
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
    //    let result = extend_selection_line_text_start::document_impl(&mut doc, semantics);
    //    assert!(!result.is_err());
    //    let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
    //    assert_eq!(expected_selections, doc.selections);
    //    assert!(!doc.is_modified());
    //}
    //fn test_errr(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    assert!(extend_selection_line_text_start::document_impl(&mut doc, semantics).is_err());
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
        let result = extend_selection_line_text_start::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_line_text_start::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn extend_line_text_start_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(3, 3), Direction::Forward),
        //        Selection::new(Range::new(8, 8), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0),
        //        Selection::with_stored_line_position(Range::new(4, 8), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 3, None),
                (8, 8, None)
            ], 0, 
            vec![
                (3, 0, Some(0)),
                (8, 4, Some(0))
            ], 0
        );
    }
    #[test] fn extend_line_text_start_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(2, 3), Direction::Forward),
        //        Selection::new(Range::new(7, 8), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0),
        //        Selection::with_stored_line_position(Range::new(4, 8), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (2, 3, None),
                (7, 8, None)
            ], 0, 
            vec![
                (3, 0, Some(0)),
                (8, 4, Some(0))
            ], 0
        );
    }
    
    //This applies to block semantics only...
    #[test] fn when_extending_from_line_end_clips_new_selection_before_newline_char(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(3, 4), Direction::Forward),
        //        Selection::new(Range::new(8, 9), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0),
        //        Selection::with_stored_line_position(Range::new(4, 8), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 4, None),
                (8, 9, None)
            ], 0, 
            vec![
                (3, 0, Some(0)),
                (8, 4, Some(0))
            ], 0
        );
    }
    
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::new(Range::new(6, 6), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(4, 6), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None),
                (6, 6, None)
            ], 0, 
            vec![
                (0, 0, None),
                (6, 4, Some(0))
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(6, 7), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(4, 7), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),
                (6, 7, None)
            ], 0, 
            vec![
                (0, 1, None),
                (7, 4, Some(0))
            ], 0
        );
    }
    
    #[test] fn errors_if_already_at_line_text_start_bar_semantics(){
        //test_errr(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::new(Range::new(4, 4), Direction::Forward)
        //    ], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None),
                (4, 4, None)
            ], 0
        );
    }
    #[test] fn errors_if_already_at_line_text_start_block_semantics(){
        //test_errr(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 5), Direction::Forward)
        //    ], 0
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
}
