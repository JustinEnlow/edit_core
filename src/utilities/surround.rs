use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError},
    selection::{Selection, Direction, CursorSemantics},
    range::Range,
    text_util
};
use ropey::Rope;

//TODO: take CursorSemantics as arg, and handle

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match selections_impl(&document.selections, &document.text, semantics){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e));}
    }
    Ok(())
}

pub fn selections_impl(selections: &Selections, text: &Rope, semantics: CursorSemantics) -> Result<Selections, SelectionsError>{
    let mut new_selections = Vec::with_capacity(2 * selections.count());
    let mut num_pushed: usize = 0;
    let primary_selection = selections.primary();
    let mut primary_selection_index = selections.primary_selection_index;
    for selection in &selections.selections{
        let surrounds = selection_impl(selection, text);
        //if selection == primary_selection{
        //    primary_selection_index = num_pushed;//.saturating_sub(1);
        //}
        //for surround in surrounds{
        //    new_selections.push(surround);
        //    num_pushed = num_pushed + 1;
        //}
        if surrounds.is_empty(){    //needed to handle mixed valid and invalid selections
            if selections.count() == 1{return Err(SelectionsError::ResultsInSameState);}
            if selection == primary_selection{
                primary_selection_index = num_pushed;//.saturating_sub(1);
            }
            new_selections.push(selection.clone());
            num_pushed = num_pushed + 1;
        }
        else{
            if selection == primary_selection{
                primary_selection_index = num_pushed;//.saturating_sub(1);
            }
            for surround in surrounds{
                new_selections.push(surround);
                num_pushed = num_pushed + 1;
            }
        }
    }
    assert!(!new_selections.is_empty());
    //if new_selections.is_empty(){Err(SelectionsError::ResultsInSameState)} //TODO: create better error?...
    //else{
        Ok(Selections::new(new_selections, primary_selection_index, text, semantics))
    //}
}

#[must_use] pub fn selection_impl(selection: &Selection, text: &Rope) -> Vec<Selection>{
    //TODO: selection.assert_invariants(text, semantics);
    let mut surround_selections = Vec::new();
    if selection.range.start == text.len_chars(){return surround_selections;}
    let first_selection = Selection::new(Range::new(selection.range.start, text_util::next_grapheme_index(selection.range.start, text)), Direction::Forward);
    let second_selection = Selection::new(Range::new(selection.range.end, text_util::next_grapheme_index(selection.range.end, text)), Direction::Forward);
    surround_selections.push(first_selection);
    surround_selections.push(second_selection);
    surround_selections
}

#[cfg(test)]
mod tests{
    use crate::utilities::surround;
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
        let result = surround::document_impl(&mut doc, semantics);
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
        assert!(surround::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_non_extended_selection(){   //also ensures primary updates properly
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward),
            ], 1, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(1, 2), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward),
                Selection::new(Range::new(5, 6), Direction::Forward),
            ], 2
        );
    }
    //TODO: need to handle bar semantics
    //#[test] fn with_non_extended_selection_bar_semantics(){   //also ensures primary updates properly
    //    test(
    //        CursorSemantics::Bar, 
    //        "idk\nsome\nshit\n", 
    //        vec![
    //            Selection::new(Range::new(0, 0), Direction::Forward),
    //            Selection::new(Range::new(4, 4), Direction::Forward),
    //        ], 1, 
    //        vec![
    //            Selection::new(Range::new(0, 0), Direction::Forward),
    //            Selection::new(Range::new(1, 1), Direction::Forward),
    //            Selection::new(Range::new(4, 4), Direction::Forward),
    //            Selection::new(Range::new(5, 5), Direction::Forward),
    //        ], 2
    //    );
    //}
    
    #[test] fn with_extended_selection(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 8), Direction::Forward)
            ], 0, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(3, 4), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward),
                Selection::new(Range::new(8, 9), Direction::Forward)
            ], 0
        );
    }

    //mixed valid and invalid selections  //one at doc end, one not
    #[test] fn mixed_valid_and_invalid_selections(){    //also ensures primary updates properly
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(14, 15), Direction::Forward)
            ], 1, 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(1, 2), Direction::Forward),
                Selection::new(Range::new(14, 15), Direction::Forward)
            ], 2
        );
    }
    
    #[test] fn errors_if_single_selection_at_doc_end(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0
        );
    }
}
