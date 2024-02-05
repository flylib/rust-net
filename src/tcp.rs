use std::io::Error;

use serde_json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::context::Context;
use crate::message::Message;
use crate::session::ISession;

pub struct TcpSession {
    id: u64,
    pub tx: Sender<Vec<u8>>,
}

impl TcpSession {
    pub fn new(id: u64, tx: Sender<Message>) -> Self {
        let (tx, mut rx) = mpsc::channel(5);
        Self {
            id,
            tx,
        }
    }
}

impl ISession for TcpSession {
    fn get_id(&self) -> u64 {
        self.id
    }

    async fn send<T>(&mut self, msg_id: u32, body: &T) -> Result<(), Error> {
        let json_data = serde_json::to_vec(body)?;
        let bytes: [u8; 4] = msg_id.to_le_bytes();
        self.tx.send(json_data).await?;
        Ok(())
    }

    async fn close(&mut self) {
        self.tx.closed().await;
    }
}


pub struct TcpServer {
    ctx: Context,
}


impl TcpServer {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
        }
    }
}

impl TcpServer {
    pub async fn listen(&mut self, addr: &str) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(addr).await?;

        println!("===server listen on {} ===", addr);
        loop {
            let (socket, remote_addr) = listener.accept().await?;
            let sid = self.ctx.get_sequence();
            let (read_ch, write_ch) = socket.into_split();


            let sender = self.ctx.tx.clone();
            TcpServer::handle_reading(sid, read_ch, sender);
            let tx = TcpServer::handle_sending(write_ch);
            let session = TcpSession::new(sid, tx);
            let guard = self.ctx.event_handler.lock().await;
            guard.on_connect(Box::new(session)).await;
            println!("get the connection from {} session id={}", remote_addr, sid);
        }
    }

    fn handle_sending<'a>(mut write_ch: OwnedWriteHalf) -> Sender<&'a Vec<u8>> {
        let (tx:Sender<Vec<u8>>, mut rx:Receiver<Vec<u8>>) = mpsc::channel(5);
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                write_ch.write_all(data).await
            }
        });

        println!("Sender dropped, exiting receiver task");

        tx
    }


    fn handle_reading(sid: u64, mut read_ch: OwnedReadHalf, tx: Sender<Message>) {
        tokio::spawn(async move {
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
        });
    }
}