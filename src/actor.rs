use crate::capability::Capability;

pub enum Message {
    Ping, // Ping what? There should be ping-pong, or not? I mean, should we support ping pong command
    // Inside enum Message if we can do program for that? On the other hand ping-pong can give us ability to know if our messages reach destination
    // On the other hand I still think there is a situations where ping-pong program would be more convinient
    Command(u32),
    TransferCapability(Capability),
}

pub trait Actor {
    fn receive(&mut self, msg: Message); // Method to recieve messages, shouldnt there be a send function?
}
