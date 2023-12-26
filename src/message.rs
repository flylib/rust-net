pub struct Message {
    id: u32,
    sid: u64,
    body: Vec<u8>,
}

impl Message {
    pub fn new(sid: u64, msg_id: u32, body: Vec<u8>) -> Self {
        Self {
            id: msg_id,
            body,
            sid,
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_sid(&self) -> u64 {
        self.sid
    }
    pub fn get_body(&self) -> &[u8] {
        &self.body
    }
}