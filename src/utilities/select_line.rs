use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Direction},
    range::Range,
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

/// Returns a new instance of [`Selection`] encompassing the current line.
//TODO: make pub fn select_line //should this include newline at end of line? //should this include indentation at start of line? //vscode includes both, as does kakoune
//TODO: if called on empty last line, this moves the selection to second to last line end, instead it should error
fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    //vs code selects all spanned lines...  maybe caller can make that determination...
    if selection.spans_multiple_lines(text, semantics){return Err(SelectionError::SpansMultipleLines);}    //make specific error. SpansMultipleLines or something...
    if text.char_to_line(selection.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}

    let line = text.char_to_line(selection.range.start);
    let line_start = text.line_to_char(line);
    let line_end = line_start + text_util::line_width(text.line(line), true);

    if selection.range.start == line_start && selection.range.end == line_end{Err(SelectionError::ResultsInSameState)}
    else{
        //Ok(Selection::new(line_start, line_end))
        Ok(Selection::new(Range::new(line_start, line_end), Direction::Forward))
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::select_line;
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
    //    let result = select_line::document_impl(&mut doc, semantics);
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
    //    assert!(select_line::document_impl(&mut doc, semantics).is_err());
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
        let result = select_line::document_impl(&mut doc, semantics);
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
        assert!(select_line::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn normal_use_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::new(Range::new(4, 4), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 4), Direction::Forward),
        //        Selection::new(Range::new(4, 9), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None),
                (4, 4, None)
            ], 0, 
            vec![
                (0, 4, None),
                (4, 9, None)
            ], 0
        );
    }
    #[test] fn normal_use_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 5), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 4), Direction::Forward),
        //        Selection::new(Range::new(4, 9), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),
                (4, 5, None)
            ], 0, 
            vec![
                (0, 4, None),
                (4, 9, None)
            ], 0
        );
    }
    #[test] fn should_succeed_if_mixed_selection_spanning_multiple_lines_and_valid_selection(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 12), Direction::Forward)
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 4), Direction::Forward),
        //        Selection::new(Range::new(4, 12), Direction::Forward)
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),
                (4, 12, None)
            ], 0, 
            vec![
                (0, 4, None),
                (4, 12, None)
            ], 0
        );
    }

    #[test] fn errors_if_selection_spans_multiple_lines_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(4, 12), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (4, 12, None)
            ], 0
        );
    }
    #[test] fn errors_if_selection_spans_multiple_lines_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(4, 12), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (4, 12, None)
            ], 0
        );
    }

    //TODO: have test with mixed new state and same state selections. should succeed...
    #[test] fn errors_if_results_in_same_state_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 4), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 4, None)
            ], 0
        );
    }
    #[test] fn errors_if_results_in_same_state_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 4), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 4, None)
            ], 0
        );
    }

    //TODO: have test with mixed valid selection and selection at doc end and line empty. should succeed...
    #[test] fn errors_if_at_doc_end_and_line_empty_bar_semantics(){
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
    #[test] fn errors_if_at_doc_end_and_line_empty_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (14, 15, None)
            ], 0
        );
    }
}
