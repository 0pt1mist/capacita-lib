use crate::engram::Engram;
pub const MAX_CAPABILITIES: usize = 16;

#[derive(Clone, Copy)]
//shitty code
pub struct Capability {
    engram_id: u32,
    keys: u8, // 0b00-None 0b01-View 0b10-Write 0b11-View and Write
    weight: u8, // 0-255
    max_ttl: u16, // How much times we can use this again, 0 - System
}

pub struct CNode {
    pub slots: [Option<Capability>; MAX_CAPABILITIES]
}

impl Capability {
    pub fn new(new_engram_id: u32, new_keys: u8) -> Self { // We can create new capability for already known engram with exactle the keys we want
        Capability { 
            engram_id: new_engram_id, 
            keys: new_keys,
            weight: 128,
            max_ttl: 0,
        }
    }

    pub fn rght_att(&mut self) {
        if self.keys == 0b11 {
            self.keys = 0b01u8; // Right there, as capability, we gave ourselves less rights than it was before. 
            // idk if thats should be there, but why not
        }
    }
    
    pub fn check(&self, engram: &Engram, required_key: u8) -> bool {
        if self.engram_id != engram.get_id() {
            return false;
        }
        (self.keys & required_key) != 0 // Bit "AND" operation
    }
}

impl CNode {
    pub fn insert(&mut self, new_cap: Capability) -> Option<u32> {
        for i in 0..MAX_CAPABILITIES {
            if let Some(ref mut existing_cap) = self.slots[i] {
                if existing_cap.engram_id == new_cap.engram_id {
                    existing_cap.keys |= new_cap.keys;
                    existing_cap.weight = existing_cap.weight.saturating_add(50); // Deduptlication, if cap already exists, we update its rights
                    return Some(i as u32);
                }
            }
        }

        for i in 0..MAX_CAPABILITIES {
            if self.slots[i].is_none() {
                self.slots[i] = Some(new_cap);
                return Some(i as u32); // New cap goes here, in emptty space
            }
        }

        let mut victim_idx = 0;
        let mut min_weight = u8::MAX;

        for i in 0..MAX_CAPABILITIES {
            if let Some(cap) = &self.slots[i] {
                if cap.weight < min_weight {
                    min_weight = cap.weight;
                    victim_idx = i; // the most unused cap is gonna die
                }
            }
        }

        self.slots[victim_idx] = Some(new_cap); // dead now
        Some(victim_idx as u32)
    }

    pub fn check_and_use(&mut self, selector: u32, engram: &Engram, required_key: u8) -> bool { // New func to verify rights and use capability
        let idx = selector as usize;
        if idx >= MAX_CAPABILITIES { // Defense against... Index 99 for example, instead of 15 max
            return false;
        }

        let mut cap = match self.slots[idx] {
            Some(c) => c, // Capability copies inside "cap"
            None => return false, // Cap is already dead
        };

        if !cap.check(engram, required_key) {
            return false; // Is it right key ro right engram?
        }

        if cap.max_ttl > 0 { // Not system capability
            cap.max_ttl -= 1;
            if cap.max_ttl == 0 {
                self.slots[idx] = None; // Zeroed slot
                return false;
            }
        }

        cap.weight = cap.weight.saturating_add(5); // If it was used, than its live a little more
        self.slots[idx] = Some(cap);
        
        true
    }
}
