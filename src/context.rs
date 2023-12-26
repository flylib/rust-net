use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::event::IEventHandler;
use crate::message::Message;

pub struct Context {
    incr: AtomicU64,
    event_handler: Arc<Box<dyn IEventHandler>>,
    pub(crate) tx: Sender<Message>,
}


impl Context {
    pub fn new(event_handler: Box<dyn IEventHandler>) -> Self {
        //global message channel
        let (tx, mut rx) = mpsc::channel(32);

        let arc_handler = Arc::new(event_handler);
        let handler_clone = arc_handler.clone();
        //run
        tokio::spawn(async move {
            // 接收并打印消息
            while let Some(msg) = rx.recv().await {
                // println!("Received: {}", msg);
                handler_clone.on_message(msg);
            }
        });

        Self {
            incr: Default::default(),
            event_handler: arc_handler,
            tx,
        }
    }
    ///流水号,从0开始
    pub fn get_sequence_num(&self) -> u64 {
        self.incr.fetch_add(1, Ordering::SeqCst)
    }
}
