const EOF: u32 = u32::MAX;
const BLOCK_SIZE: usize = 512;
use core::cmp::min;

pub struct Block {
    pub data: [u8; BLOCK_SIZE],
}

pub struct StorageManager {
    block_pool: [Block; 1024],
    bitmap: [u32; 32], // Map, where every bit - like map, where 0 - Block is free, and 1 - Block is not free
    block_chain: [u32; 1024], // If something has size more than 512 B
}

impl StorageManager {
    pub const fn new() -> Self {
        const BLK: Block = Block { data: [0; BLOCK_SIZE] };
        StorageManager {
            block_pool: [BLK; 1024],
            bitmap: [0; 32],
            block_chain: [EOF; 1024],
        }
    }

    pub fn allocate_single_block(&mut self) -> Option<u32> { //Slightly changed alloc function
        for (bitmap_part_idx, bitmap_part) in self.bitmap.iter_mut().enumerate() {
            if *bitmap_part != u32::MAX {
                let bit_index = bitmap_part.trailing_zeros();
                *bitmap_part |= 1 << bit_index; // Switch 0 to 1
                let current_block_id = ((bitmap_part_idx * 32) + bit_index as usize) as u32; // Global block id
                self.block_chain[current_block_id as usize] = EOF;
                return Some(current_block_id);
            }
        }
        None // For times when storage is full
    }

    pub fn link_blocks(&mut self, prev_block: u32, next_block: u32) {
        self.block_chain[prev_block as usize] = next_block;
    }

    pub fn read_chain(&self, first_block_id: u32, buffer: &mut [u8], total_size: usize) -> usize {
        let mut bytes_left = total_size;
        let mut buffer_offset = 0;
        let mut current_block_id = first_block_id;

        while bytes_left > 0 {
            if current_block_id == EOF {
                if bytes_left > 0 {
                    // Err(());
                    // So, there is an EOF, but bytes_left > 0, data corruption, error
                    break; 
                }
            }

            let bytes_to_copy = min(BLOCK_SIZE, bytes_left);

            if buffer_offset + bytes_to_copy > buffer.len() {
                // Err(());
                // Error again, now its actors fault, it should know what buffer size it should have
                break;
            }

            // Actor's buffer slice
            let block_idx = current_block_id as usize;
            buffer[buffer_offset .. buffer_offset + bytes_to_copy]
                .copy_from_slice(&self.block_pool[block_idx].data[0..bytes_to_copy]);

            buffer_offset += bytes_to_copy;
            bytes_left -= bytes_to_copy;
            
            current_block_id = self.block_chain[block_idx];
        }

        buffer_offset
    }

    pub fn free_chain(&mut self, first_block_id: u32) {
        let mut current_block_id = first_block_id;
        while current_block_id != EOF {
            let block_idx = current_block_id as usize;

            let word_idx = block_idx / 32;
            let bit_idx = block_idx % 32;

            self.bitmap[word_idx] &= !(1 << bit_idx); // Reset bit to zero

            let next_block_id = self.block_chain[block_idx]; // Next block id

            self.block_chain[block_idx] = EOF; 
            current_block_id = next_block_id; // Next block
        }
    }
}
