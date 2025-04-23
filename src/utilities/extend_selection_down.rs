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

/// Returns a new instance of [`Selection`] with the [`Selection`] extended down.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{ //TODO: ensure this can't extend past doc text end
    selection.assert_invariants(text, semantics);
    //if text.char_to_line(self.cursor(text, semantics)) == text.len_lines().saturating_sub(1){return Err(SelectionError::ResultsInSameState);}
    let last_line = text.len_lines().saturating_sub(1);
    if text.char_to_line(selection.range.start) == last_line
    || text.char_to_line(selection.range.end) == last_line
    || text.char_to_line(selection.cursor(text, semantics)) == last_line{return Err(SelectionError::ResultsInSameState);}

    selection.move_vertically(1, text, Movement::Extend, Direction::Forward, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_down;
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
        let result = extend_selection_down::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_down::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_valid_selections_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "some\nshit\nidk\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),  //common use
                Selection::new(Range::new(9, 9), Direction::Forward)   //to shorter line
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 5), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(9, 13), Direction::Forward, 4),
            ], 0
        );  
    }
    #[test] fn with_multiple_valid_selections_block_semantics(){
        test(
            CursorSemantics::Block, 
            "some\nshit\nidk\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),  //common use
                Selection::new(Range::new(9, 10), Direction::Forward)   //to shorter line
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 6), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(9, 14), Direction::Forward, 4),
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
                Selection::with_stored_line_position(Range::new(0, 4), Direction::Forward, 0),
                Selection::new(Range::new(14, 14), Direction::Forward)
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
                Selection::with_stored_line_position(Range::new(0, 5), Direction::Forward, 0),
                Selection::new(Range::new(14, 15), Direction::Forward)
            ], 0
        );
    }

    #[test] fn errors_when_single_selection_on_bottom_line_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0
        );
    }
    #[test] fn errors_when_single_selection_on_bottom_line_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0
        );
    }
}
