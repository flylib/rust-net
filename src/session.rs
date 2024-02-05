use std::io::Error;

pub trait ISession {
    fn get_id(&self) -> u64;
    async fn send<T>(&mut self, msg_id: u32, body: &T) -> Result<(), Error>;
    async fn close(&mut self);
}