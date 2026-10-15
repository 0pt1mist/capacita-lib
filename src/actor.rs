use crate::capability::Capability;

pub enum Message {
    Ping,
    Command(u32),
    TransferCapability(Capability),
}

pub trait Actor {
    fn receive(&mut self, msg: Message);
}
