use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
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

/// Returns a new instance of [`Selection`] with the [`Selection`] extended to the end of the current line.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{    //TODO: ensure this can't extend past doc text end
    selection.assert_invariants(text, semantics);
    let line_number = text.char_to_line(selection.cursor(text, semantics));
    let line = text.line(line_number);
    let line_width = text_util::line_width(line, false);    //doesn't include newline
    let line_start = text.line_to_char(line_number);
    let line_end = line_start.saturating_add(line_width);   //index at end of line text, not including newline  //nth_next_grapheme_index(line_start, line_width, text)?

    match semantics{
        CursorSemantics::Bar => {
            if selection.cursor(text, semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
            selection.put_cursor(line_end, text, Movement::Extend, semantics, true)
        }
        CursorSemantics::Block => {
            //if self.cursor(semantics) == line_end.saturating_sub(1)
            if selection.cursor(text, semantics) == text_util::previous_grapheme_index(line_end, text)
            || selection.cursor(text, semantics) == line_end{return Err(SelectionError::ResultsInSameState);}
            let start_line = text.char_to_line(selection.range.start);
            let end_line = text.char_to_line(selection.range.end);
            if selection.cursor(text, semantics) == selection.range.start && end_line > start_line{
                selection.put_cursor(line_end, text, Movement::Extend, semantics, true)  //put cursor over newline, if extending from a line below
            }else{
                //self.put_cursor(line_end.saturating_sub(1), text, Movement::Extend, semantics, true)
                selection.put_cursor(text_util::previous_grapheme_index(line_end, text), text, Movement::Extend, semantics, true)
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_line_end;
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
        let result = extend_selection_line_end::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_line_end::document_impl(&mut doc, semantics).is_err());
    }

    #[test] fn moves_to_line_text_end_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::new(Range::new(4, 4), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 3),
                Selection::with_stored_line_position(Range::new(4, 8), Direction::Forward, 4)
            ], 0
        );
    }
    #[test] fn moves_to_line_text_end_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2),
                Selection::with_stored_line_position(Range::new(4, 8), Direction::Forward, 3)
            ], 0
        );
    }
    
    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(3, 3), Direction::Forward),
                Selection::new(Range::new(5, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(3, 3), Direction::Forward),
                Selection::with_stored_line_position(Range::new(5, 8), Direction::Forward, 4)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(2, 3), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(2, 3), Direction::Forward),
                Selection::with_stored_line_position(Range::new(4, 8), Direction::Forward, 3)
            ], 0
        );
    }
    
    #[test] fn errors_if_already_at_line_text_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(3, 3), Direction::Forward),
                Selection::new(Range::new(8, 8), Direction::Forward)
            ], 0
        );
    }
    #[test] fn errors_if_already_at_line_text_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(2, 3), Direction::Forward),
                Selection::new(Range::new(7, 8), Direction::Forward)
            ], 0
        );
    }

    //Only applies to block cursor semantics
    #[test] fn error_if_already_at_line_end(){  //with cursor over newline char
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(3, 4), Direction::Forward),
                Selection::new(Range::new(8, 9), Direction::Forward)
            ], 0
        );
    }
}
