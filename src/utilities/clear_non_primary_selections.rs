use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError},
};

pub fn document_impl(document: &mut Document) -> Result<(), DocumentError>{
    match selections_impl(&document.selections){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Removes all [`Selection`]s except [`Selection`] at `primary_selection_index`.
/// Errors if [`Selections`] has only 1 [`Selection`].
pub fn selections_impl(selections: &Selections) -> Result<Selections, SelectionsError>{ //left this as public, because it is used elsewhere in codebase...
    if selections.count() < 2{return Err(SelectionsError::SingleSelection);}
    
    let primary_as_vec = vec![selections.primary().clone()];
    assert!(primary_as_vec.len() == 1);
    
    Ok(Selections{
        selections: primary_as_vec,
        primary_selection_index: 0
    })
}

#[cfg(test)]
mod tests{
    use crate::utilities::clear_non_primary_selections;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics},
    };
    use ropey::Rope;

    //fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    let result = clear_non_primary_selections::document_impl(&mut doc);
    //    assert!(!result.is_err());
    //    let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
    //    assert_eq!(expected_selections, doc.selections);
    //    assert!(!doc.is_modified());
    //}
    //fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    assert!(clear_non_primary_selections::document_impl(&mut doc).is_err());
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
        let result = clear_non_primary_selections::document_impl(&mut doc);
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
        assert!(clear_non_primary_selections::document_impl(&mut doc).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn clears_non_primary_with_multiple_selections(){
        //test(
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 5), Direction::Forward)
        //    ], 0, 
        //    vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),
                (4, 5, None)
            ], 0, 
            vec![
                (0, 1, None)
            ], 0
        );
    }
    
    #[test] fn errors_if_single_selection(){
        //test_error(
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None)
            ], 0
        );
    }
}
