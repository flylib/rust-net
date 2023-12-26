

pub trait ISession{
    fn get_id()->u64;
    fn send<T>(msg_id:u32,v:T);
}