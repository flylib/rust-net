use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;

use crate::event::IEventHandler;
use crate::message::Message;

pub struct Context {
    incr: AtomicU64,
    pub event_handler: Arc<Mutex<Box<dyn IEventHandler>>>,
    pub tx: Sender<Message>,
}


impl Context {
    pub fn new(event_handler: Box<dyn IEventHandler>) -> Self {
        //global message channel
        let (tx, mut rx) = mpsc::channel(32);

        let arc_handler = Arc::new(Mutex::new(event_handler));
        let clone_handler = arc_handler.clone();
        //run
        tokio::spawn(async move {
            // 接收并打印消息
            while let Some(msg) = rx.recv().await {
                // println!("Received: {}", msg);
                let guard = clone_handler.lock().await;
                guard.on_message(msg);
            }
        });

        Self {
            incr: Default::default(),
            event_handler: arc_handler,
            tx,
        }
    }
    ///流水号,从0开始
    pub fn get_sequence(&self) -> u64 {
        self.incr.fetch_add(1, Ordering::SeqCst)
    }
}
