/// Type alias for usize
/// ```
/// # use edit_core::id::ClientIDManager;
/// 
/// let mut id_manager = ClientIDManager::default();
/// let _ = id_manager.assign_id();
/// id_manager.release_id(0);
/// assert!(id_manager.assign_id() == 0);
/// ```
pub type ClientID = usize;  // only an alias. doesn't enforce passed value is a ClientID and not just a usize.

/// Generates an ID for each client(individual front end instance or separate tabs 
/// inside an instance) connected to the editor.
/// # Example
/// ```
/// # use edit_core::id::ClientIDManager;
/// 
/// let mut manager = ClientIDManager::default();
/// let id0 = manager.assign_id();
/// let id1 = manager.assign_id();
/// manager.release_id(id0);
/// let id3 = manager.assign_id();
/// assert!(id1 == 1);
/// assert!(id3 == 0);
/// ```
#[derive(Default)]
pub struct ClientIDManager{
    next_id: ClientID,
    available_ids: Vec<ClientID>,  //should this be newtyped as ClientID(usize)? or type aliased?
}
impl ClientIDManager{
    pub fn assign_id(&mut self) -> ClientID{
        if let Some(id) = self.available_ids.first().cloned(){
            self.available_ids.remove(0);
            id
        }else{
            let id = self.next_id;
            self.next_id = self.next_id + 1;
            id
        }
    }

    pub fn release_id(&mut self, id: ClientID){
        self.available_ids.push(id);
        self.available_ids.sort();
    }
}
