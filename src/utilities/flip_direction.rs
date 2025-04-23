use crate::{
    document::{Document, DocumentError},
    selection::{Selection, CursorSemantics, Direction, SelectionError},
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_non_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if !selection.is_extended(semantics){return Err(SelectionError::ResultsInSameState)}
    Ok(
        //Selection{
        //    range: selection.range.clone(), 
        //    direction: match selection.direction{
        //        Direction::Forward => {Direction::Backward}
        //        Direction::Backward => {Direction::Forward}
        //    },
        //    stored_line_position: None
        //}
        Selection::new(
            selection.range.clone(), 
            match selection.direction{
                Direction::Forward => {Direction::Backward}
                Direction::Backward => {Direction::Forward}
            }
        )
    )
}

#[cfg(test)]
mod tests{
    use crate::utilities::flip_direction;
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
        let result = flip_direction::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    //fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text));
    //    assert!(flip_direction::document_impl(&mut doc, semantics).is_err());
    //    assert!(!doc.is_modified());
    //}

    #[test] fn forward_selections_flip_backwards_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 8), Direction::Forward),
            ], 0, 
            vec![
                Selection::new(Range::new(0, 3), Direction::Backward),
                Selection::new(Range::new(4, 8), Direction::Backward),
            ], 0
        );
    }
    #[test] fn forward_selections_flip_backwards_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 4), Direction::Forward),
                Selection::new(Range::new(4, 9), Direction::Forward),
            ], 0, 
            vec![
                Selection::new(Range::new(0, 4), Direction::Backward),
                Selection::new(Range::new(4, 9), Direction::Backward),
            ], 0
        );
    }

    #[test] fn backward_selections_flip_forwards_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Backward),
                Selection::new(Range::new(4, 8), Direction::Backward),
            ], 0, 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 8), Direction::Forward),
            ], 0
        );
    }
    #[test] fn backward_selections_flip_forwards_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 4), Direction::Backward),
                Selection::new(Range::new(4, 9), Direction::Backward),
            ], 0, 
            vec![
                Selection::new(Range::new(0, 4), Direction::Forward),
                Selection::new(Range::new(4, 9), Direction::Forward),
            ], 0
        );
    }

    //TODO: what about mixed directions? should they even be allowed?...
}
