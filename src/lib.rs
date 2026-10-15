#![no_std]

pub struct Engram {
    tags: [u32;32],
    data: [u128;1024]
}

impl Engram {
    pub fn create(new_tags: [u32;32], new_data: [u128;1024]) -> Self {
        Engram {tags:new_tags, data:new_data}
    }

    pub fn view(&self) -> &[u128;1024] {
        &self.data
    }

    // Deprecated shit lower, I warned you
    //pub fn delete(&self) {
    //    drop(Engram); 
    //}

    pub fn give_tag(&mut self, new_tag: u32) {
        for element in &mut self.tags   {
            if *element == 0 {
                *element = new_tag;
                break;
            }
        }
    }

    pub fn view_tags(&self) -> &[u32;32] {
        &self.tags
    }

    pub fn delete_tag(&mut self, target_tag: u32) {
        for element in &mut self.tags   {
            if *element == target_tag {
                *element = 0;
                break;
            }
        }
    }
}