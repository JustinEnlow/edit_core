use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
    text_util
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_clearing_non_primary(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with [`Selection`] extended to encompass all text.
fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{  //TODO: ensure this can't extend past doc text end
    selection.assert_invariants(text, semantics);
    if selection.range.start == 0 
    && (
        selection.range.end == text.len_chars() || 
        selection.range.end == text.len_chars().saturating_add(1)
    ){return Err(SelectionError::ResultsInSameState);}
    
    let selection = selection.put_cursor(0, text, Movement::Move, semantics, true)?;
    //selection.put_cursor(text.len_chars(), text, Movement::Extend, semantics, true)
    selection.put_cursor(
        match semantics{
            CursorSemantics::Bar => text.len_chars(), 
            CursorSemantics::Block => text_util::previous_grapheme_index(text.len_chars(), text)
        }, 
        text, 
        Movement::Extend, 
        semantics, 
        true
    )
}

#[cfg(test)]
mod tests{
    use crate::utilities::select_all;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics));
        let result = select_all::document_impl(&mut doc, semantics);
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
        assert!(select_all::document_impl(&mut doc, semantics).is_err());
    }
    
    //TODO: should this really be returning a selection with stored_line_position?...
    
    #[test] fn selects_all_and_clears_non_primary_selections(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4)], 0);
    }
    #[test] fn ensure_cannot_past_text_len(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4)], 0
        );
    }
    
    #[test] fn errors_if_all_already_selected(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        );
    }

}
