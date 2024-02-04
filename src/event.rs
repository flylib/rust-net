use crate::message::Message;
use crate::session::ISession;

//事件处理钩子
pub trait IEventHandler {
    async fn on_connect(&self, session: Box<dyn ISession>);
    async fn on_message(&self, msg: Message);
    async fn on_close(&self, sid: u64);
}