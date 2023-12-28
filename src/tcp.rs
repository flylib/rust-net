use std::io::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;

use crate::context::Context;
use crate::message::Message;
use crate::session::ISession;

pub struct Session {
    id: u64,
    write_ch: OwnedWriteHalf,
}

impl Session {
    pub fn new(id: u64, write_ch: OwnedWriteHalf) -> Self {
        Self {
            id,
            write_ch,
        }
    }
}

impl ISession for Session {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn send(&mut self, msg_id: u32, body: Vec<u8>) -> Result<(), Error> {
        let _ = self.write_ch.write_all(&body);
        Ok(())
    }

    fn close(&mut self) -> Result<(), Error> {
        let _ = self.write_ch.shutdown();
        Ok(())
    }
}


pub struct Server {
    ctx: Context,
}


impl Server {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
        }
    }
}

impl Server {
    pub async fn listen(&mut self, addr: &str) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(addr).await?;

        println!("===server listen on {} ===", addr);
        loop {
            let (socket, remote_addr) = listener.accept().await?;
            let sid = self.ctx.get_sequence();
            let (read, write) = socket.into_split();
            let session = Session::new(sid, write);
            let guard = self.ctx.event_handler.lock().await;
            guard.on_connect(Box::new(session));
            println!("get the connection from {} session id={}", remote_addr, sid);

            let sender = self.ctx.tx.clone();
            tokio::spawn(async move {
                Server::handle_connection(sid, read, sender).await;
            });
        }
    }


    async fn handle_connection(sid: u64, mut read_ch: OwnedReadHalf, tx: Sender<Message>) {
        let mut buffer = [0; 1024];
        loop {
            let _ = match read_ch.read(&mut buffer).await {
                Ok(0) => {
                    eprintln!("socket-{} close!!!", sid);
                    break;
                }
                Ok(n) => {
                    /// handle the message
                    tx.send(Message::new(sid, 1, buffer[..n].to_vec()))
                }
                Err(e) => {
                    eprintln!("Error reading from socket-{}: {}", sid, e);
                    break;
                }
            }.await;
        }
        println!("Connection-{} closed done", sid);
    }
}