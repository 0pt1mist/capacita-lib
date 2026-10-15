use crate::engram::Engram;
//shitty code
pub struct Capability {
    engram_id: u32,
    keys: u8, 
}

impl Capability {
    pub fn new(new_engram_id: u32, new_keys: u8) -> Self {
        Capability { engram_id: new_engram_id, keys: new_keys }
    }

    pub fn rght_att(&mut self) {
        if self.keys == 0b11 {
            self.keys = 0b01u8;
        }
    }
    
    pub fn check<'a>(&self, engram: &'a Engram) -> Option<&'a [u128; 1024]> {
        if self.engram_id != engram.get_id() {
            return None;
        }

        if (self.keys & 0b01) != 0 {
            Some(engram.view())
        } else {
            None
        }
    }
}
