use crate::actor::{Actor, Mailbox, Message};
use crate::capability::CNode;
use crate::storage::StorageManager;
use crate::engram::Engram;

pub const MAX_ACTORS: usize = 32;

pub enum TransactionStage { // Phases of transaction
    Allocating, //Allocates new data rn
    Freeing, //Succes, now - clearing not necessary blocks
}

pub struct TransactionRecord { // Name says it all
    pub engram_id: u32, // Engram id that we are changing rn
    pub stage: TransactionStage, // Stage from above
    pub temporary_chain_start: u32, // Global id of memory block
}

pub struct ActorControlBlock { // Only kernel struct, main descriptor
    pub id: u32, // Actor id
    pub cnode: CNode, // Individual capability table for this actor
    pub mailbox: Mailbox, // Async queue
    pub transaction: Option<TransactionRecord>, // Some or none
    pub spam_penalty_ticks: u32, // Toxicity value
    pub actor_impl: Option<&'static mut dyn Actor>, // Optional link to real object
}

pub struct Kernel {
    pub storage: *mut StorageManager,
    pub actors: [Option<ActorControlBlock>; MAX_ACTORS], // Only 32 actors for now
    pub current_actor_idx: usize, // Lact working actor uid
}

impl Kernel {
    pub const fn new(storage_manager: *mut StorageManager) -> Self { // Core starter
        const NONE_ACB: Option<ActorControlBlock> = None;
        Kernel {
            storage: storage_manager,
            actors: [NONE_ACB; MAX_ACTORS],
            current_actor_idx: 0,
        }
    }

    pub fn kill_and_reclaim(&mut self, actor_idx: usize) { // Let it crash, here we go
        if let Some(mut acb) = self.actors[actor_idx].take() {
            if let Some(tx) = acb.transaction {
                unsafe { (*self.storage).free_chain(tx.temporary_chain_start); }
            }

            let mut fresh_cnode = CNode { slots: [None; 16] };
            for i in 0..16 {
                if let Some(cap) = acb.cnode.slots[i] {
                    if cap.get_weight() == 255 {
                        fresh_cnode.insert(cap);
                    }
                }
            }

            self.actors[actor_idx] = Some(ActorControlBlock {
                id: acb.id,
                cnode: fresh_cnode,
                mailbox: Mailbox::new(),
                transaction: None,
                spam_penalty_ticks: 0,
                actor_impl: acb.actor_impl,
            });
        }
    }

    pub fn schedule(&mut self) { // Async scheduler
        for i in 0..MAX_ACTORS {
            let idx = (self.current_actor_idx + i) % MAX_ACTORS;
            
            let should_kill = if let Some(ref mut acb) = self.actors[idx] {
                if acb.spam_penalty_ticks > 0 {
                    acb.spam_penalty_ticks -= 1;
                    false
                } else if let Some(msg) = acb.mailbox.pop() {
                    if let Some(ref mut actor) = acb.actor_impl {
                        self.current_actor_idx = (idx + 1) % MAX_ACTORS;
                        actor.receive(msg);
                        false
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if should_kill {
                self.kill_and_reclaim(idx);
                break;
            }
        }
    }

    pub fn sys_send(&mut self, sender_idx: u32, target_idx: u32, msg: Message) -> Result<(), ()> { // Async non-blocking IPC O(1)
        let sender_idx = sender_idx as usize;
        let target_idx = target_idx as usize;

        if sender_idx >= MAX_ACTORS || target_idx >= MAX_ACTORS {
            return Err(());
        }

        let mut target_mailbox_full = false;

        if let Some(ref mut target_acb) = self.actors[target_idx] {
            if target_acb.mailbox.push(msg).is_err() {
                target_mailbox_full = true;
            }
        } else {
            return Err(());
        }

        if target_mailbox_full {
            if let Some(ref mut sender_acb) = self.actors[sender_idx] {
                sender_acb.spam_penalty_ticks = sender_acb.spam_penalty_ticks.saturating_add(10);
            }
            return Err(());
        }

        Ok(())
    }

    pub fn sys_read(&mut self, actor_idx: u32, selector: u32, engram: &Engram, buffer: &mut [u8]) -> Result<usize, ()> { // Read something from memory
        let idx = actor_idx as usize;
        if idx >= MAX_ACTORS {
            return Err(());
        }

        if let Some(ref mut acb) = self.actors[idx] {
            if !acb.cnode.check_and_use(selector, engram, 0b01) {
                return Err(());
            }
            
            let first_block = *engram.view();
            let total_size = engram.get_total_size() as usize;
            let read_bytes = unsafe { (*self.storage).read_chain(first_block, buffer, total_size) };
            return Ok(read_bytes);
        }
        Err(())
    }
}
