use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError}
};

pub fn document_impl(document: &mut Document) -> Result<(), DocumentError>{
    match selections_impl(&document.selections){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Decrements the primary selection index.
fn selections_impl(selections: &Selections) -> Result<Selections, SelectionsError>{
    if selections.count() < 2{return Err(SelectionsError::SingleSelection);}
    if selections.primary_selection_index > 0{
        Ok(Selections{selections: selections.selections.clone(), primary_selection_index: selections.primary_selection_index - 1})
    }else{
        Ok(Selections{selections: selections.selections.clone(), primary_selection_index: selections.count().saturating_sub(1)})
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::decrement_primary_selection;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics));
        let result = decrement_primary_selection::document_impl(&mut doc);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics));
        assert!(decrement_primary_selection::document_impl(&mut doc).is_err());
    }

    #[test] fn with_multiple_selections(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 1, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            CursorSemantics::Block
        );
    }
    #[test] fn wraps_if_primary_is_first(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 1, 
            CursorSemantics::Block
        );
    }

    #[test] fn errors_if_single_selection(){
        test_error(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
            CursorSemantics::Block
        );
    }
}
