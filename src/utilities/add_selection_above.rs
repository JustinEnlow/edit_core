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

//TODO: add selection above/below fns don't work as expected when multiple selections on same line. only adds primary selection range above/below

/// Adds a new [`Selection`] directly above the top-most [`Selection`], with the same start and end offsets from line start, if possible.
fn selections_impl(selections: &Selections, text: &Rope, semantics: CursorSemantics) -> Result<Selections, SelectionsError>{
    assert!(selections.count() > 0);  //ensure at least one selection in selections

    let top_selection = selections.first();
    let top_selection_line = text.char_to_line(top_selection.range.start);
    if top_selection_line == 0{return Err(SelectionsError::CannotAddSelectionAbove);}
    // should error if any selection spans multiple lines. //callee can determine appropriate response behavior in this case        //vscode behavior is to extend topmost selection up one line if any selection spans multiple lines
    for selection in &selections.selections{  //self.selections.iter(){   //change suggested by clippy lint
        if selection.spans_multiple_lines(text, semantics){return Err(SelectionsError::SpansMultipleLines);}
    }

    // using primary selection here, because that is the selection we want our added selection to emulate, if possible with the available text
    let start_offset = text_util::offset_from_line_start(selections.primary().range.start, text);
    let end_offset = start_offset.saturating_add(selections.primary().range.end.saturating_sub(selections.primary().range.start));  //start_offset + (end char index - start char index)
    let line_above = top_selection_line.saturating_sub(1);
    let line_start = text.line_to_char(line_above);
    let line_text = text.line(line_above);
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
        //Direction::Forward => Ok(self.push_front(Selection::new(start, end), false)),
        Direction::Forward => Ok(selections.push_front(Selection::new(Range::new(start, end), Direction::Forward), false)),
        //Direction::Backward => Ok(self.push_front(Selection::new(end, start), false))
        Direction::Backward => Ok(selections.push_front(Selection::new(Range::new(start, end), Direction::Backward), false))
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::add_selection_above;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let result = add_selection_above::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(add_selection_above::document_impl(&mut doc, semantics).is_err());
    }

    //to line with same len or more
        //non extended
            //bar
                //selection direction forward
                #[test] fn to_line_with_same_len_or_more_with_non_extended_selection_with_direction_forward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(9, 9), Direction::Forward)], 0, 
                        vec![
                            Selection::new(Range::new(4, 4), Direction::Forward),
                            Selection::new(Range::new(9, 9), Direction::Forward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn to_line_with_same_len_or_more_with_non_extended_selection_with_direction_backward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(9, 9), Direction::Backward)], 0, 
                        vec![
                            Selection::new(Range::new(4, 4), Direction::Backward),
                            Selection::new(Range::new(9, 9), Direction::Backward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn to_line_with_same_len_or_more_with_extended_selection_with_direction_forward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(9, 13), Direction::Forward)], 0, 
                        vec![
                            Selection::new(Range::new(4, 8), Direction::Forward),
                            Selection::new(Range::new(9, 13), Direction::Forward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn to_line_with_same_len_or_more_with_extended_selection_with_direction_backward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(9, 13), Direction::Backward)], 0, 
                        vec![
                            Selection::new(Range::new(4, 8), Direction::Backward),
                            Selection::new(Range::new(9, 13), Direction::Backward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
    //to shorter line
        //non extended
            //bar
                //selection direction forward
                #[test] fn to_shorter_line_with_non_extended_selection_with_direction_forward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0, 
                        vec![
                            Selection::new(Range::new(0, 0), Direction::Forward),
                            Selection::new(Range::new(4, 4), Direction::Forward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn to_shorter_line_with_non_extended_selection_with_direction_backward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(4, 4), Direction::Backward)], 0, 
                        vec![
                            Selection::new(Range::new(0, 0), Direction::Backward),
                            Selection::new(Range::new(4, 4), Direction::Backward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn to_shorter_line_with_extended_selection_with_direction_forward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(4, 8), Direction::Forward)], 0, 
                        vec![
                            Selection::new(Range::new(0, 4), Direction::Forward),
                            Selection::new(Range::new(4, 8), Direction::Forward)
                        ], 1, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn to_shorter_line_with_extended_selection_with_direction_backward_bar_semantics(){
                    test(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(4, 8), Direction::Backward)], 0, 
                        vec![
                            Selection::new(Range::new(0, 4), Direction::Backward),
                            Selection::new(Range::new(4, 8), Direction::Backward)
                        ], 1, 
                        CursorSemantics::Bar
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
    //should error if on top line
        //non extended
            //bar
                //selection direction forward
                #[test] fn should_error_if_on_top_line_with_non_extended_selection_with_direction_forward_bar_semantics(){
                    test_error(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn should_error_if_on_top_line_with_non_extended_selection_with_direction_backward_bar_semantics(){
                    test_error(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(0, 0), Direction::Backward)], 0, 
                        CursorSemantics::Bar
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
        //extended
            //bar
                //selection direction forward
                #[test] fn should_error_if_on_top_line_with_extended_selection_with_direction_forward_bar_semantics(){
                    test_error(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn should_error_if_on_top_line_with_extended_selection_with_direction_backward_bar_semantics(){
                    test_error(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(0, 3), Direction::Backward)], 0, 
                        CursorSemantics::Bar
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
                    test_error(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(0, 9), Direction::Forward)], 0, 
                        CursorSemantics::Bar
                    );
                }
                //selection direction backward
                #[test] fn should_error_if_any_selection_is_multiline_with_direction_backward_bar_semantics(){
                    test_error(
                        "idk\nsome\nshit\n", 
                        vec![Selection::new(Range::new(0, 9), Direction::Backward)], 0, 
                        CursorSemantics::Bar
                    );
                }
            //block
                //selection direction forward
                //selection direction backward
}
