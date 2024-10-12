//use std::{fs::File, path::PathBuf};

use edit_core::{
    id::{
        ClientID,
        ClientIDManager
    },
    editor::Editor, 
    ServerAction, 
    ServerResponse
};
use edit_core::selection::{CursorSemantics, Movement};



const CURSOR_SEMANTICS: CursorSemantics = CursorSemantics::Bar;



pub trait Server{
    fn listen(&mut self) -> ServerAction;
    fn respond(&mut self, response: ServerResponse);
    fn handle_new_connections(&mut self);
}

pub struct ServerOverNamedPipes{}
impl ServerOverNamedPipes{
    pub fn new() -> Self{
        Self{}
    }
}
impl Server for ServerOverNamedPipes{
    fn listen(&mut self) -> ServerAction{ServerAction::NoOp}
    fn respond(&mut self, _response: ServerResponse){}
    fn handle_new_connections(&mut self){}
}



fn main(){
    let mut editor = Editor::default();
    let mut server = ServerOverNamedPipes::new();
    loop{
        server.handle_new_connections();
        let request = server.listen();
        println!("server received: {:#?}", request);
        let id = ClientIDManager::default().assign_id();
        if let Some(response) = server_action_to_response(request, id, &mut editor){
            println!("server emitted: {:#?}", response);
            server.respond(response);
        }else{}
    }
}

/*TODO: in cases where requested action results in no state change, return ServerResponse::Acknowledge
ideally limiting the amount of data being transferred between client/server, and restricting logging
to actual state changes
*/
pub fn server_action_to_response(action: ServerAction, client_address: ClientID, editor: &mut Editor) -> Option<ServerResponse>{
    match action{
        ServerAction::NoOp => {Some(ServerResponse::Acknowledge)}
        ServerAction::Backspace => {
            if let Some(doc) = editor.document_mut(client_address){
                doc.backspace(CURSOR_SEMANTICS);
                
                let text = doc.text().clone();
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::CloseConnection => {
            if let Some(doc) = editor.document(client_address){
                println!("{}: closing {}", client_address, doc.file_name().unwrap());
            }
            editor.close_document(client_address);

            None
        },
        ServerAction::Delete => {
            if let Some(doc) = editor.document_mut(client_address){
                doc.delete(CURSOR_SEMANTICS);
                
                let text = doc.text().clone();
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::ExtendSelectionDown => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().extend_selections_down(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.extend_down(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::ExtendSelectionEnd => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().extend_selections_end(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.extend_line_text_end(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::ExtendSelectionHome => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().extend_selections_home(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.extend_home(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::ExtendSelectionLeft => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().extend_selections_left(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.extend_left(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::ExtendSelectionRight => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().extend_selections_right(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.extend_right(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::ExtendSelectionUp => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().extend_selections_up(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.extend_up(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::GoTo{line_number} => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                doc.selections_mut().clear_non_primary_selections();
                doc.selections_mut().first_mut().set_from_line_number(line_number, &text, Movement::Move, CURSOR_SEMANTICS);
                
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::OpenFile{file_path} => {
            match editor.open_document(&file_path, client_address){
                Ok(_) => {
                    if let Some(doc) = editor.document(client_address){
                        Some(ServerResponse::FileOpened {file_name: doc.file_name(), document_length: doc.len()})
                    }else{
                        Some(ServerResponse::Failed("no document open".to_string()))
                    }
                }
                Err(e) => {
                    Some(ServerResponse::Failed(format!("{}", e)))
                }
            }
        },
        ServerAction::UpdateClientViewSize{width, height} => {
            if let Some(doc) = editor.document_mut(client_address){
                doc.view_mut().set_size(width as usize, height as usize);
                
                let text = doc.text().clone();
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::ScrollClientViewDown{amount} => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                doc.view_mut().scroll_down(amount, &text);
                
                let selections = doc.selections().clone();
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::ScrollClientViewLeft{amount} => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                doc.view_mut().scroll_left(amount);
                
                let selections = doc.selections().clone();
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::ScrollClientViewRight{amount} => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                doc.view_mut().scroll_right(amount, &text);
                
                let selections = doc.selections();
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::ScrollClientViewUp{amount} => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                doc.view_mut().scroll_up(amount);
                
                let selections = doc.selections();
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorDocumentEnd => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_document_end(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_doc_end(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS), 
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::MoveCursorDocumentStart => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_document_start();
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_doc_start(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::MoveCursorDown => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_down(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_down(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorUp => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_up(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_up(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorRight => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_right(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_right(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorLeft => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_left(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_left(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorLineEnd => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_end(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_line_text_end(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorLineStart => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                
                //doc.selections_mut().move_cursors_home(&text);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_home(&text, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorPageDown => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                let view = doc.view().clone();
                
                //doc.selections_mut().move_cursors_page_down(&text, &view);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_page_down(&text, &view, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::MoveCursorPageUp => {
            if let Some(doc) = editor.document_mut(client_address){
                let text = doc.text().clone();
                let view = doc.view().clone();
                
                //doc.selections_mut().move_cursors_page_up(&text, &view);
                for selection in doc.selections_mut().iter_mut(){
                    selection.move_page_up(&text, &view, CURSOR_SEMANTICS);
                }
                
                let selections = doc.selections().clone();
                if doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS){
                    Some(ServerResponse::DisplayView{
                        content: doc.view().text(&text),
                        line_numbers: doc.view().line_numbers(&text),
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                        modified: doc.is_modified()
                    })
                }else{
                    Some(ServerResponse::CursorPosition{
                        client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                        document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    })
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        },
        ServerAction::InserChar(c) => {
            if let Some(doc) = editor.document_mut(client_address){
                doc.insert_char(c, CURSOR_SEMANTICS);
                
                let text = doc.text().clone();
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::InsertNewline => {
            if let Some(doc) = editor.document_mut(client_address){
                doc.enter(CURSOR_SEMANTICS);
                
                let text = doc.text().clone();
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::InsertTab => {
            if let Some(doc) = editor.document_mut(client_address){
                doc.tab(CURSOR_SEMANTICS);
                
                let text = doc.text().clone();
                let selections = doc.selections().clone();
                doc.view_mut().scroll_following_cursor(&selections, &text, CURSOR_SEMANTICS);
                
                Some(ServerResponse::DisplayView{
                    content: doc.view().text(&text),
                    line_numbers: doc.view().line_numbers(&text),
                    client_cursor_positions: doc.view().cursor_positions(&text, &selections, CURSOR_SEMANTICS),
                    document_cursor_position: selections.cursor_positions(&text, CURSOR_SEMANTICS),
                    modified: doc.is_modified()
                })
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
        ServerAction::Save => {
            if let Some(doc) = editor.document_mut(client_address){
                match doc.save(){
                    Ok(_) => {
                        let text = doc.text();
                        let selections = doc.selections();
                        Some(ServerResponse::DisplayView{
                            content: doc.view().text(text),
                            line_numbers: doc.view().line_numbers(text),
                            client_cursor_positions: doc.view().cursor_positions(text, selections, CURSOR_SEMANTICS),
                            document_cursor_position: selections.cursor_positions(text, CURSOR_SEMANTICS),
                            modified: doc.is_modified() //this is the important bit
                        })
                    }
                    Err(e) => {
                        Some(ServerResponse::Failed(format!("failed to save. error: {}", e)))
                    }
                }
            }else{
                Some(ServerResponse::Failed("no document open".to_string()))
            }
        }
    }
}
