use std::io::Error;

pub trait ISession {
    fn get_id(&self) -> u64;
    async fn send(&mut self, msg_id: u32, body: Vec<u8>) -> Result<(), Error>;
    async fn close(&mut self) -> Result<(), Error>;
}