pub struct Capability {
    engram_id: u32, // unused for now, will be in actor.rs
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
    
    pub fn check(&self) -> bool {
        if (self.keys & 0b01) != 0 {
            true
        } else {
            false
        }
    }
}
