use crate::capability::Capability;
pub const MAILBOX_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub enum Message {
    Ping, // Ping what? There should be ping-pong, or not? I mean, should we support ping pong command
    // Inside enum Message if we can do program for that? On the other hand ping-pong can give us ability to know if our messages reach destination
    // On the other hand I still think there is a situations where ping-pong program would be more convinient <- Still dont know, 19.10.26
    Command(u32),
    TransferCapability(Capability),
}

pub trait Actor {
    fn receive(&mut self, msg: Message); // Method to recieve messages, shouldnt there be a send function? <- No, kernel will do it
}

pub struct Mailbox {
    ring: [Option<Message>; MAILBOX_SIZE],
    head: usize, // The oldest "Message"
    tail: usize, // free one
}

impl Mailbox {
    pub const fn new() -> Self {
        Mailbox {
            ring: [None; MAILBOX_SIZE],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, msg: Message) -> Result<(), ()> {
        let next_tail = (self.tail + 1) % MAILBOX_SIZE;
        if next_tail == self.head { // So, if mailbox is full, we have error now
            return Err(());
        }
        self.ring[self.tail] = Some(msg); // If there is a free space, we are saving in in tail
        self.tail = next_tail;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Message> {
        if self.head == self.tail {
            return None;
        }
        let msg = self.ring[self.head].take();
        self.head = (self.head + 1) % MAILBOX_SIZE;
        msg // If there is somthing, we are sending it to actor
    }
}
