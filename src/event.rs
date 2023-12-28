use crate::message::Message;
use crate::session::ISession;

//事件处理钩子
pub trait IEventHandler: Send {
    fn on_connect(&self, session: Box<dyn ISession>);
    fn on_message(&self, msg: Message);
    fn on_close(&self, sid: u64);
}