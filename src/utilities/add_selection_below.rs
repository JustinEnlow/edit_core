use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError},
    selection::{Selection, CursorSemantics, Direction},
    range::Range,
    text_util
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match selections_impl(&document.selections, &document.text, semantics){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

// TODO: selection added below at text end is not rendering on last line(this is a frontend issue though)
/// Adds a new [`Selection`] directly below bottom-most [`Selection`], with the same start and end offsets from line start, if possible.
fn selections_impl(selections: &Selections, text: &Rope, semantics: CursorSemantics) -> Result<Selections, SelectionsError>{
    assert!(selections.count() > 0);  //ensure at least one selection in selections

    let bottom_selection = selections.last();
    let bottom_selection_line = text.char_to_line(bottom_selection.range.start);
    //bottom_selection_line must be zero based, and text.len_lines() one based...   //TODO: verify
    if bottom_selection_line >= text.len_lines().saturating_sub(1){return Err(SelectionsError::CannotAddSelectionBelow);}
    // should error if any selection spans multiple lines. //callee can determine appropriate response behavior in this case        //vscode behavior is to extend topmost selection down one line if any selection spans multiple lines
    for selection in &selections.selections{  //self.selections.iter(){   //change suggested by clippy lint
        if selection.spans_multiple_lines(text, semantics){return Err(SelectionsError::SpansMultipleLines);}
    }

    // using primary selection here, because that is the selection we want our added selection to emulate, if possible with the available text
    let start_offset = text_util::offset_from_line_start(selections.primary().range.start, text);
    let end_offset = start_offset.saturating_add(selections.primary().range.end.saturating_sub(selections.primary().range.start));  //start_offset + (end char index - start char index)
    let line_below = bottom_selection_line.saturating_add(1);
    let line_start = text.line_to_char(line_below);
    let line_text = text.line(line_below);
    let line_width = text_util::line_width(line_text, false);
    let line_width_including_newline = text_util::line_width(line_text, true);
    let (start, end) = if line_text.to_string().is_empty() || line_text == "\n"{    //should be impossible for the text in the line above first selection to be empty. is_empty() check is redundant here...
        match semantics{
            CursorSemantics::Bar => (line_start, line_start),
            CursorSemantics::Block => (line_start, text_util::next_grapheme_index(line_start, text))
        }
    }
    else if selections.primary().is_extended(semantics){
        if start_offset < line_width{   //should we exclusively handle start_offset < line_width && end_offset < line_width as well?
            (line_start.saturating_add(start_offset), line_start.saturating_add(end_offset.min(line_width_including_newline))) //start offset already verified within line text bounds
        }
        else{
            // currently same as non extended. this might change...
            match semantics{    //ensure adding the offsets doesn't make this go past line width
                CursorSemantics::Bar => (line_start.saturating_add(start_offset.min(line_width)), line_start.saturating_add(start_offset.min(line_width))),
                CursorSemantics::Block => (line_start.saturating_add(start_offset.min(line_width)), text_util::next_grapheme_index(line_start.saturating_add(start_offset.min(line_width)), text))
            }
        }
    }
    else{  //not extended
        match semantics{    //ensure adding the offsets doesn't make this go past line width
            CursorSemantics::Bar => (line_start.saturating_add(start_offset.min(line_width)), line_start.saturating_add(start_offset.min(line_width))),
            CursorSemantics::Block => (line_start.saturating_add(start_offset.min(line_width)), text_util::next_grapheme_index(line_start.saturating_add(start_offset.min(line_width)), text))
        }
    };

    match selections.primary().direction{
        //Direction::Forward => Ok(self.push(Selection::new(start, end), false)),
        Direction::Forward => Ok(selections.push(Selection::new(Range::new(start, end), Direction::Forward), false)),
        //Direction::Backward => Ok(self.push(Selection::new(end, start), false))
        //Direction::Backward => Ok(selections.push(Selection::new(Range::new(end, start), Direction::Backward), false))
        Direction::Backward => Ok(selections.push(Selection::new(Range::new(start, end), Direction::Backward), false))
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::add_selection_below;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics/*, Direction*/},
        //range::Range,
    };
    use ropey::Rope;

    //fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    let result = add_selection_below::document_impl(&mut doc, semantics);
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
    //    assert!(add_selection_below::document_impl(&mut doc, semantics).is_err());
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
        let result = add_selection_below::document_impl(&mut doc, semantics);
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
        assert!(add_selection_below::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //to line with same len or more
        //non extended
            //bar
                //selection direction forward
                #[test] fn to_line_with_same_len_or_more_with_non_extended_selection_with_direction_forward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(0, 0), Direction::Forward),
                    //        Selection::new(Range::new(4, 4), Direction::Forward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (0, 0, None)
                        ], 0, 
                        vec![
                            (0, 0, None),
                            (4, 4, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn to_line_with_same_len_or_more_with_non_extended_selection_with_direction_backward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(0, 0), Direction::Backward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(0, 0), Direction::Backward),
                    //        Selection::new(Range::new(4, 4), Direction::Backward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (0, 0, None)
                        ], 0, 
                        vec![
                            (0, 0, None),
                            (4, 4, None)
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn to_line_with_same_len_or_more_with_extended_selection_with_direction_forward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(0, 3), Direction::Forward),
                    //        Selection::new(Range::new(4, 7), Direction::Forward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (0, 3, None)
                        ], 0, 
                        vec![
                            (0, 3, None),
                            (4, 7, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn to_line_with_same_len_or_more_with_extended_selection_with_direction_backward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(0, 3), Direction::Backward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(0, 3), Direction::Backward),
                    //        Selection::new(Range::new(4, 7), Direction::Backward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (3, 0, None)
                        ], 0, 
                        vec![
                            (3, 0, None),
                            (7, 4, None)
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
    //to shorter line
        //non extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
    //to empty line
        //non extended
            //bar
                //selection direction forward
                #[test] fn to_empty_line_with_non_extended_selection_with_direction_forward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(9, 9), Direction::Forward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(9, 9), Direction::Forward),
                    //        Selection::new(Range::new(14, 14), Direction::Forward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (9, 9, None)
                        ], 0, 
                        vec![
                            (9, 9, None),
                            (14, 14, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn to_empty_line_with_non_extended_selection_with_direction_backward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(9, 9), Direction::Backward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(9, 9), Direction::Backward),
                    //        Selection::new(Range::new(14, 14), Direction::Backward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (9, 9, None)
                        ], 0, 
                        vec![
                            (9, 9, None),
                            (14, 14, None)
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn to_empty_line_with_extended_selection_with_direction_forward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(9, 13), Direction::Forward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(9, 13), Direction::Forward),
                    //        Selection::new(Range::new(14, 14), Direction::Forward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (9, 13, None)
                        ], 0, 
                        vec![
                            (9, 13, None),
                            (14, 14, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn to_empty_line_with_extended_selection_with_direction_backward_bar_semantics(){
                    //test(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(9, 13), Direction::Backward)], 0, 
                    //    vec![
                    //        Selection::new(Range::new(9, 13), Direction::Backward),
                    //        Selection::new(Range::new(14, 14), Direction::Backward)
                    //    ], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (13, 9, None)   //backwards
                        ], 0, 
                        vec![
                            (13, 9, None),  //backwards
                            (14, 14, None)  //returning selection backward, but should be selection forward. or Direction::None, if we add that...
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
    //to line with only newline char
        //non extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
    //with multiple selections on same line (should merge overlapping if needed)
        //non extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
    //should error if on bottom line
        //non extended
            //bar
                //selection direction forward
                #[test] fn should_error_if_non_extended_selection_with_forward_direction_on_bottom_line_bar_semantics(){
                    //test_error(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test_error(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (14, 14, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn should_error_if_non_extended_selection_with_backward_direction_on_bottom_line_bar_semantics(){
                    //test_error(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(14, 14), Direction::Backward)], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test_error(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (14, 14, None)
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn should_error_if_extended_selection_with_forward_direction_on_bottom_line_bar_semantics(){
                    //test_error(
                    //    "idk\nsome\nshit", 
                    //    vec![Selection::new(Range::new(9, 9), Direction::Forward)], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test_error(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit", 
                        vec![
                            (9, 11, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn should_error_if_extended_selection_with_backward_direction_on_bottom_line_bar_semantics(){
                    //test_error(
                    //    "idk\nsome\nshit", 
                    //    vec![Selection::new(Range::new(9, 9), Direction::Backward)], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test_error(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit", 
                        vec![
                            (11, 9, None)
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
    //should error if any selection is multiline
        //non extended
            //bar
                //selection direction forward
                //selection direction backward
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn should_error_if_any_selection_is_multiline_with_direction_forward_bar_semantics(){
                    //test_error(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(0, 9), Direction::Forward)], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test_error(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (0, 9, None)
                        ], 0
                    );
                }
                //selection direction backward
                #[test] fn should_error_if_any_selection_is_multiline_with_direction_backward_bar_semantics(){
                    //test_error(
                    //    "idk\nsome\nshit\n", 
                    //    vec![Selection::new(Range::new(0, 9), Direction::Backward)], 0, 
                    //    CursorSemantics::Bar
                    //);
                    test_error(
                        CursorSemantics::Bar, 
                        "idk\nsome\nshit\n", 
                        vec![
                            (9, 0, None)
                        ], 0
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
}
