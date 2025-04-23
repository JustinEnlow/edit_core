use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics},
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

/// Returns a new instance of [`Selection`] with cursor moved to absolute start of line, or start of line text, depending on current cursor position.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    let line_number = text.char_to_line(selection.cursor(text, semantics));
    let line_start = text.line_to_char(line_number);
    let text_start_offset = text_util::first_non_whitespace_character_offset(text.line(line_number));
    let text_start = line_start.saturating_add(text_start_offset);  //nth_next_grapheme_index(line_start, text_start_offset, text)?

    //if text_start == line_start && self.cursor(semantics) == line_start{return Err(());}    //would result in same state    //TODO: test
    //if selection.cursor(text, semantics) == text_start{selection.move_line_start(text, semantics)}
    if selection.cursor(text, semantics) == text_start{crate::utilities::move_cursor_line_start::selection_impl(selection, text, semantics)}
    //else{selection.move_line_text_start(text, semantics)}
    else{crate::utilities::move_cursor_line_text_start::selection_impl(selection, text, semantics)}
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_cursor_home;
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
            .with_selections(Selections::new(selections, primary, &text, semantics));
        let result = move_cursor_home::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics));
        assert!(move_cursor_home::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //TODO: test switching between line start and line text start

    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),   //invalid
                Selection::new(Range::new(6, 6), Direction::Forward),   //from middle of line
                Selection::new(Range::new(13, 13), Direction::Forward)    //from end of line
            ], 0, 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(9, 9), Direction::Forward, 0),
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),   //invalid
                Selection::new(Range::new(6, 7), Direction::Forward),   //from middle of line
                Selection::new(Range::new(13, 14), Direction::Forward)    //from end of line
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 0),
            ], 0
        );
    }

    #[test] fn errors_when_single_selection_at_line_start_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0
        );
    }
    #[test] fn errors_when_single_selection_at_line_start_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        );
    }
}
