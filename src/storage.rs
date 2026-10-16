
const EOF: u32 = u32::MAX;
const BLOCK_SIZE: usize = 512;

pub struct Block {
    pub data: [u8; BLOCK_SIZE],
}

pub struct StorageManager {
    block_pool: [Block; 1024],
    bitmap: [u32; 32], // Map, where every bit - like map, where 0 - Block is free, and 1 - Block is not free
    block_chain: [u32; 1024] // If something has size more than 512 B
}

impl StorageManager {
    fn allocate(&mut self, total_size: usize) { 
        let hm_blocks:usize = total_size / BLOCK_SIZE;
        for _ in 0..hm_blocks { // goddamit
            
        }
    }

    fn read_chain() {

    }

    fn free_chain() {

    }
}

