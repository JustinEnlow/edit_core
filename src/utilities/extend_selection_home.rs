use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics},
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

/// Returns a new instance of [`Selection`] with the [`Selection`] extended to absolute start of line, or line text start, depending on [`Selection`] `head` position.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    use crate::utilities::extend_selection_line_text_start;
    use crate::utilities::extend_selection_line_start;
    
    selection.assert_invariants(text, semantics);
    let line_number = text.char_to_line(selection.cursor(text, semantics));
    let line_start = text.line_to_char(line_number);
    let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
    let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

    //if text_start == line_start && self.cursor(semantics) == line_start{return Err(());}    //would result in same state
    //if selection.cursor(text, semantics) == text_start{selection.extend_line_start(text, semantics)}
    if selection.cursor(text, semantics) == text_start{extend_selection_line_start::selection_impl(selection, text, semantics)}
    //else{selection.extend_line_text_start(text, semantics)}
    else{extend_selection_line_text_start::selection_impl(selection, text, semantics)}
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_home;
    use ropey::Rope;
    use crate::document::Document;
    use crate::selection::{Selection, CursorSemantics};
    use crate::selections::Selections;

    //fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    let result = extend_selection_home::document_impl(&mut doc, semantics);
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
    //    assert!(extend_selection_home::document_impl(&mut doc, semantics).is_err());
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
        let result = extend_selection_home::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_home::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn when_cursor_past_line_text_start_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "    idk\n    something\n", 
        //    vec![
        //        Selection::new(Range::new(6, 6), Direction::Forward),
        //        Selection::new(Range::new(16, 16), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(4, 6), Direction::Backward, 4),
        //        Selection::with_stored_line_position(Range::new(12, 16), Direction::Backward, 4)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "    idk\n    something\n", 
            vec![
                (6, 6, None),
                (16, 16, None)
            ], 0, 
            vec![
                (6, 4, Some(4)),
                (16, 12, Some(4))
            ], 0
        );
    }
    #[test] fn when_cursor_past_line_text_start_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "    idk\n    something\n", 
        //    vec![
        //        Selection::new(Range::new(6, 7), Direction::Forward),
        //        Selection::new(Range::new(16, 17), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(4, 7), Direction::Backward, 4),
        //        Selection::with_stored_line_position(Range::new(12, 17), Direction::Backward, 4)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "    idk\n    something\n", 
            vec![
                (6, 7, None),
                (16, 17, None)
            ], 0, 
            vec![
                (7, 4, Some(4)),
                (17, 12, Some(4))
            ], 0
        );
    }
    #[test] fn when_cursor_at_line_text_start_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "    idk\n    something\n", 
        //    vec![
        //        Selection::new(Range::new(4, 4), Direction::Forward),
        //        Selection::new(Range::new(12, 12), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0),
        //        Selection::with_stored_line_position(Range::new(8, 12), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "    idk\n    something\n", 
            vec![
                (4, 4, None),
                (12, 12, None)
            ], 0, 
            vec![
                (4, 0, Some(0)),
                (12, 8, Some(0))
            ], 0
        );
    }
    #[test] fn when_cursor_at_line_text_start_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "    idk\n    something\n", 
        //    vec![
        //        Selection::new(Range::new(4, 5), Direction::Forward),
        //        Selection::new(Range::new(12, 13), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0),
        //        Selection::with_stored_line_position(Range::new(8, 13), Direction::Backward, 0)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "    idk\n    something\n", 
            vec![
                (4, 5, None),
                (12, 13, None)
            ], 0, 
            vec![
                (5, 0, Some(0)),
                (13, 8, Some(0))
            ], 0
        );
    }
    #[test] fn when_cursor_before_line_text_start_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "    idk\n    something\n", 
        //    vec![
        //        Selection::new(Range::new(2, 2), Direction::Forward),
        //        Selection::new(Range::new(10, 10), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(2, 4), Direction::Forward, 4),
        //        Selection::with_stored_line_position(Range::new(10, 12), Direction::Forward, 4)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "    idk\n    something\n", 
            vec![
                (2, 2, None),
                (10, 10, None)
            ], 0, 
            vec![
                (2, 4, Some(4)),
                (10, 12, Some(4))
            ], 0
        );
    }
    #[test] fn when_cursor_before_line_text_start_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "    idk\n    something\n", 
        //    vec![
        //        Selection::new(Range::new(2, 3), Direction::Forward),
        //        Selection::new(Range::new(10, 11), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::with_stored_line_position(Range::new(2, 5), Direction::Forward, 4),
        //        Selection::with_stored_line_position(Range::new(10, 13), Direction::Forward, 4)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "    idk\n    something\n", 
            vec![
                (2, 3, None),
                (10, 11, None)
            ], 0, 
            vec![
                (2, 5, Some(4)),
                (10, 13, Some(4))
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
    
    #[test] fn errors_when_line_start_and_line_text_start_and_cursor_position_all_equal_bar_semantics(){
        //test_error(
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
    #[test] fn errors_when_line_start_and_line_text_start_and_cursor_position_all_equal_block_semantics(){
        //test_error(
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
