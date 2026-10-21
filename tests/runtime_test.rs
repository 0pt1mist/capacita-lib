use capacita_lib::actor::{Actor, Mailbox, Message};
use capacita_lib::capability::CNode;
use capacita_lib::storage::StorageManager;
use capacita_lib::kernel::{Kernel, ActorControlBlock};
use core::cell::UnsafeCell;

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

static REAL_STORAGE: SyncUnsafeCell<StorageManager> = SyncUnsafeCell(UnsafeCell::new(StorageManager::new()));
static KERNEL: SyncUnsafeCell<Option<Kernel>> = SyncUnsafeCell(UnsafeCell::new(None));

struct SpammerActor;

impl Actor for SpammerActor {
    fn receive(&mut self, _msg: Message) {}
}

struct VictimActor {
    received_count: u32,
}

impl Actor for VictimActor {
    fn receive(&mut self, _msg: Message) {
        self.received_count += 1;
    }
}

#[test]
fn test_two_actors_ipc_and_toxicity() {
    unsafe {
        let storage_ptr = REAL_STORAGE.0.get();
        let kernel_ptr = KERNEL.0.get();

        *kernel_ptr = Some(Kernel::new(storage_ptr));
    }

    let kernel_ptr = KERNEL.0.get();
    let kernel = unsafe { (*kernel_ptr).as_mut().unwrap() };

    let mut spammer = SpammerActor;
    let mut victim = VictimActor { received_count: 0 };

    let spammer_ref: &'static mut dyn Actor = unsafe {
        core::mem::transmute(&mut spammer as &mut dyn Actor)
    };
    let victim_ref: &'static mut dyn Actor = unsafe {
        core::mem::transmute(&mut victim as &mut dyn Actor)
    };

    kernel.actors[0] = Some(ActorControlBlock {
        id: 100,
        cnode: CNode { slots: [None; 16] },
        mailbox: Mailbox::new(),
        transaction: None,
        spam_penalty_ticks: 0,
        actor_impl: Some(spammer_ref),
    });

    kernel.actors[1] = Some(ActorControlBlock {
        id: 200,
        cnode: CNode { slots: [None; 16] },
        mailbox: Mailbox::new(),
        transaction: None,
        spam_penalty_ticks: 0,
        actor_impl: Some(victim_ref),
    });

    assert!(kernel.sys_send(0, 1, Message::Ping).is_ok());
    kernel.schedule();
    assert_eq!(victim.received_count, 1);

    for _ in 0..15 {
        assert!(kernel.sys_send(0, 1, Message::Ping).is_ok());
    }

    let spam_result = kernel.sys_send(0, 1, Message::Ping);
    assert!(spam_result.is_err());

    if let Some(ref sender_acb) = kernel.actors[0] {
        assert_eq!(sender_acb.spam_penalty_ticks, 10);
    }

    if let Some(ref target_acb) = kernel.actors[1] {
        assert_eq!(target_acb.spam_penalty_ticks, 0);
    }

    kernel.schedule();
    if let Some(ref sender_acb) = kernel.actors[0] {
        assert_eq!(sender_acb.spam_penalty_ticks, 9);
    }
}

// Tests is coded by ChatGPT, so please, if you have suggestion or fixes, or you found bug - tell me in Issues or Pull requests