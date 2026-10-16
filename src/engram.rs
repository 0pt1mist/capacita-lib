const NO_BLOCK: u32 = 0;
const NO_SIZE: u32 = 0;

pub struct Engram {
    id: u32,
    tags: [u32;32], // 128 byte mf
    first_block_id: u32,
    total_size: u32,
}

impl Engram {
    pub fn get_id(&self) -> u32 { //If something have capability for that engram, it can check its id.
        self.id
    }

    pub fn create(new_id: u32, new_tags: [u32;32]) -> Self { // Our core can create new engrams, with new id, and zero tags and no first block and no size
        Engram {id: new_id, tags: new_tags, first_block_id: NO_BLOCK, total_size: NO_SIZE}
    }

    pub fn view(&self) -> &u32 { // View self first block, just for now, without memory manager
        &self.first_block_id
    }

    // Deprecated shit lower, I warned you
    //pub fn delete(&self) {
    //    drop(Engram); <<<<<<<< Wont work btw
    //} Maybe I would need it sometime, like when engram dies

    pub fn give_tag(&mut self, new_tag: u32) {
        for element in &mut self.tags   {
            if *element == 0 { // If we spot empty space inside tags array, we are adding there new tag
                *element = new_tag;
                break;
            }
        }
    }

    pub fn view_tags(&self) -> &[u32;32] { // View self tags
        &self.tags
    }

    pub fn delete_tag(&mut self, target_tag: u32) {
        for element in &mut self.tags   {
            if *element == target_tag { // If we spot exactly the element we are looking for, we are deleting it, or, technically, making it 0
                *element = 0;
                break;
            }
        }
    }
}