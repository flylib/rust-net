use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};

use rust_net::context::Context;
use rust_net::event::IEventHandler;
use rust_net::message::Message;
use rust_net::session::ISession;
use rust_net::tcp::TcpServer;

#[tokio::main]
async fn main() {
    let ctx = Context::new(Box::new(EventHandler::new()));
    let mut server = TcpServer::new(ctx);
    server.listen("127.0.0.1:8089").await.unwrap();
}


struct EventHandler {
    sessions: DashMap<u64, Box<dyn ISession>>,
}


impl EventHandler {
    fn new() -> Self {
        Self {
            sessions: Default::default()
        }
    }
    pub fn add_session(&self, session: Box<dyn ISession>) {
        self.sessions.insert(session.get_id(), session);
    }
    pub fn remove_session(&self, id: u64) {
        self.sessions.remove(&id);
    }

    pub fn get_session(&self, id: u64) -> Option<Ref<u64, Box<dyn ISession>>> {
        self.sessions.get(&id)
    }
    pub fn get_session_mut(&self, id: u64) -> Option<RefMut<u64, Box<dyn ISession>>> {
        self.sessions.get_mut(&id)
    }
}


impl IEventHandler for EventHandler {
    fn on_connect(&self, session: Box<dyn ISession>) {
        println!("new session-{}", session.get_id());
        self.add_session(session);
    }

    fn on_message(&self, msg: Message) {
        println!("session-{} say: {}", msg.get_sid(), String::from_utf8_lossy(msg.get_body()))
    }

    fn on_close(&self, sid: u64) {
        eprintln!("session-{} close", sid);
        self.remove_session(sid);
    }
}