use crate::engram::Engram;
//shitty code
pub struct Capability {
    engram_id: u32,
    keys: u8, // 0b00-None 0b01-View 0b10-Write 0b11-View and Write
}

impl Capability {
    pub fn new(new_engram_id: u32, new_keys: u8) -> Self { // We can create new capability for already known engram with exactle the keys we want
        Capability { engram_id: new_engram_id, keys: new_keys }
    }

    pub fn rght_att(&mut self) {
        if self.keys == 0b11 {
            self.keys = 0b01u8; // Right there, as capability, we gave ourselves less rights than it was before. 
            // idk if thats should be there, but why not
        }
    }
    
    pub fn check<'a>(&self, engram: &'a Engram) -> Option<&'a [u128; 1024]> {
        if self.engram_id != engram.get_id() {
            return None; //If capability have other engram_id than the one inside engram - no access
        }

        if (self.keys & 0b01) != 0 {
            Some(engram.view()) // If capability have view rights, we are using engram.view()
        } else {
            None
        }
    }
}
