use crate::{
    document::{Document, DocumentError},
    selection::{Selection, CursorSemantics},
    view::{View, ViewError}
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match view_impl(&document.client_view, document.selections.primary(), &document.text, semantics){
        Ok(view) => {document.client_view = view}
        Err(e) => {
            match e{
                ViewError::InvalidInput => {return Err(DocumentError::InvalidInput);}
                ViewError::ResultsInSameState => {return Err(DocumentError::InvalidInput);} //need error same state in document...
            }
        }
    }

    Ok(())
}

/// Returns an instance of [`View`] vertically centered around specified cursor.
/// # Errors
///     //if function output would return a `View` with the same state.
/// # Panics
///     //if `selection` is invalid.
///     //if `text` is invalid.
fn view_impl(view: &View, selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<View, ViewError>{
    assert!(selection.cursor(text, semantics) <= text.len_chars());    //ensure selection is valid
    assert!(text.len_lines() > 0);  //ensure text is not empty
        
    let current_line = text.char_to_line(selection.cursor(text, semantics));
    //let view_is_even_numbered = self.height % 2 == 0;
    let half_view_height = view.height / 2; //current impl will be biased towards the bottom of the view, if view is even numbered

    //TODO: consider how even numbered view heights should be handled...
    // maybe < half_view_height.saturating_sub(1)
    if current_line <= half_view_height{return Err(ViewError::ResultsInSameState);} //maybe return error cursor before doc_start + half the view height
    if current_line >= text.len_lines().saturating_sub(half_view_height){return Err(ViewError::ResultsInSameState);}    //maybe return error cursor after doc_end - half the view height

    // Calculate the new vertical start position
    let new_vertical_start = if current_line > half_view_height{
        current_line.saturating_sub(half_view_height)
    }else{
        0
    }.min(text.len_lines().saturating_sub(view.height));    //should self.height be half_view_height?

    // if view_is_even_numbered && (current_line == new_vertical_start || current_line == new_vertical_start.saturating_sub(1)){return Err(ViewError::ResultsInSameState);}
    //if current_line == new_vertical_start{return Err(ViewError::ResultsInSameState);}   //maybe return error already centered   //TODO: and test
    //

    let new_view = View::new(view.horizontal_start, new_vertical_start, view.width, view.height);    
    if new_view == view.clone(){return Err(ViewError::ResultsInSameState);} //can we catch this condition any earlier?...
    Ok(new_view)
}

#[cfg(test)]
mod tests{
    use crate::utilities::center_view_vertically_around_cursor;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics},
        view::View
    };
    use ropey::Rope;

    //fn test(text: &str, view: View, selections: Vec<Selection>, primary: usize, expected_text: &str, expected_view: View, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics))
    //        .with_view(view);
    //    let result = center_view_vertically_around_cursor::document_impl(&mut doc, semantics);
    //    assert!(!result.is_err());
    //    assert_eq!(expected_text.to_string(), doc.client_view.text(&text));
    //    assert_eq!(expected_view, doc.client_view);
    //}
    //fn test_error(text: &str, view: View, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics))
    //        .with_view(view);
    //    assert!(center_view_vertically_around_cursor::document_impl(&mut doc, semantics).is_err());
    //}
    fn test(semantics: CursorSemantics, text: &str, view: View, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize, expected_text: &str, expected_view: View){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections.clone())
            .with_view(view);
        let result = center_view_vertically_around_cursor::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        assert_eq!(expected_text.to_string(), doc.client_view.text(&text));
        assert_eq!(expected_view, doc.client_view);
        //is it necessary to assert selections haven't changed?...
        assert_eq!(selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, view: View, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections)
            .with_view(view);
        assert!(center_view_vertically_around_cursor::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }
    
    #[test] fn works_when_cursor_in_valid_position_before_center(){
        // i d k                                        // i d k
        // y e t                                        //|y e t|
        //|s o m|e      //<-- primary cursor here -->   //|s o m|e
        //|m o r|e                                      //|m o r|e
        //|o t h|e r                                    // o t h e r
        // r a n d o m                                  // r a n d o m
        // s h i t                                      // s h i t
        //test(
        //    "idk\nyet\nsome\nmore\nother\nrandom\nshit\n", 
        //    View::new(0, 2, 3, 3), 
        //    vec![Selection::new(Range::new(8, 9), Direction::Forward)], 0, 
        //    "yet\nsom\nmor\n", 
        //    View::new(0, 1, 3, 3), 
        //    CursorSemantics::Block
        //);
        test(
            CursorSemantics::Block, 
            "idk\nyet\nsome\nmore\nother\nrandom\nshit\n", 
            View::new(0, 2, 3, 3), 
            vec![
                (8, 9, None)
            ], 0, 
            "yet\nsom\nmor\n", 
            View::new(0, 1, 3, 3)
        );
    }
    #[test] fn works_when_cursor_in_valid_position_after_center(){
        // i d k                                        // i d k
        // y e t                                        // y e t
        //|s o m|e                                      // s o m e
        //|m o r|e                                      //|m o r|e
        //|o t h|e r    //<-- primary cursor here -->   //|o t h|e r
        // r a n d o m                                  //|r a n|d o m
        // s h i t                                      // s h i t
        //test(
        //    "idk\nyet\nsome\nmore\nother\nrandom\nshit\n", 
        //    View::new(0, 2, 3, 3), 
        //    vec![Selection::new(Range::new(18, 19), Direction::Forward)], 0, 
        //    "mor\noth\nran\n", 
        //    View::new(0, 3, 3, 3), 
        //    CursorSemantics::Block
        //);
        test(
            CursorSemantics::Block, 
            "idk\nyet\nsome\nmore\nother\nrandom\nshit\n", 
            View::new(0, 2, 3, 3), 
            vec![
                (18, 19, None)
            ], 0, 
            "mor\noth\nran\n", 
            View::new(0, 3, 3, 3)
        );
    }

    #[test] fn errors_when_cursor_before_half_view_height(){
        //|i d k|       //<-- primary cursor here -->   //|i d k|
        //|s o m|e                                      //|s o m|e
        //|m o r|e                                      //|m o r|e
        // o t h e r                                    // o t h e r
        // s h i t                                      // s h i t
        //test_error(
        //    "idk\nsome\nmore\nother\nshit\n", 
        //    View::new(0, 0, 3, 3), 
        //    vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nmore\nother\nshit\n", 
            View::new(0, 0, 3, 3), 
            vec![
                (0, 1, None)
            ], 0
        );
    }
    
    #[test] fn errors_when_cursor_after_doc_end_minus_half_view_height(){
        // i d k                                        // i d k
        // s o m e                                      // s o m e
        //|m o r|e                                      //|m o r|e
        //|o t h|e r                                    //|o t h|e r
        //|s h i|t      //<-- primary cursor here -->   //|s h i|t
        //test_error(
        //    "idk\nsome\nmore\nother\nshit\n", 
        //    View::new(0, 2, 3, 3), 
        //    vec![Selection::new(Range::new(25, 26), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nmore\nother\nshit\n", 
            View::new(0, 2, 3, 3), 
            vec![
                (25, 26, None)
            ], 0
        );
    }
    
    #[test] fn errors_when_cursor_already_centered_with_odd_num_lines(){
        // i d k                                        // i d k
        //|s o m|e                                      //|s o m|e
        //|m o r|e      //<-- primary cursor here -->   //|m o r|e
        //|o t h|e r                                    //|o t h|e r
        // s h i t                                      // s h i t
        //test_error(
        //    "idk\nsome\nmore\nother\nshit\n", 
        //    View::new(0, 1, 3, 3), 
        //    vec![Selection::new(Range::new(9, 10), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nmore\nother\nshit\n", 
            View::new(0, 1, 3, 3), 
            vec![
                (9, 10, None)
            ], 0
        );
    }
    #[test] fn errors_when_cursor_on_first_middle_line_with_even_num_lines(){
        // i d k                                        // i d k
        //|y e t|                                       //|y e t|
        //|s o m|e      //<-- primary cursor here -->   //|s o m|e
        //|m o r|e                                      //|m o r|e
        //|o t h|e r                                    //|o t h|e r
        // s h i t                                      // s h i t
        //test_error(
        //    "idk\nyet\nsome\nmore\nother\nshit\n", 
        //    View::new(0, 1, 3, 4), 
        //    vec![Selection::new(Range::new(8, 9), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nyet\nsome\nmore\nother\nshit\n", 
            View::new(0, 1, 3, 4), 
            vec![
                (8, 9, None)
            ], 0
        );
    }
    #[test] fn errors_when_cursor_on_other_middle_line_with_even_num_lines(){
        // i d k                                        // i d k
        //|y e t|                                       //|y e t|
        //|s o m|e                                      //|s o m|e
        //|m o r|e      //<-- primary cursor here -->   //|m o r|e
        //|o t h|e r                                    //|o t h|e r
        // s h i t                                      // s h i t
        //test_error(
        //    "idk\nyet\nsome\nmore\nother\nshit\n", 
        //    View::new(0, 1, 3, 4), 
        //    vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0, 
        //    CursorSemantics::Block
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nyet\nsome\nmore\nother\nshit\n", 
            View::new(0, 1, 3, 4), 
            vec![
                (13, 14, None)
            ], 0
        );
    }
}
