pub struct Engram {
    id: u32,
    tags: [u32;32], // 128 byte mf
    data: [u128;1024] //16 kb file
    // 16 512+u32 byte for one file. Is it too much? I mean I cant change it, its locked, I can have thousands of files with 1 byte data and thats bad I think?
}

impl Engram {
    pub fn get_id(&self) -> u32 { //If something have capability for that engram, it can check its id.
        self.id
    }

    pub fn create(new_id: u32, new_tags: [u32;32], new_data: [u128;1024]) -> Self { // Our core can create new engrams, with new id, and zero tags and data
        Engram {id: new_id, tags: new_tags, data: new_data,}
    }

    pub fn view(&self) -> &[u128;1024] { // View self data
        &self.data
    }

    // Deprecated shit lower, I warned you
    //pub fn delete(&self) {
    //    drop(Engram); <<<<<<<< Wont work btw
    //}

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
            if *element == target_tag { // If we spot exactle the element we are looking for, we are deleting it, or, technically, making it 0
                *element = 0;
                break;
            }
        }
    }
}