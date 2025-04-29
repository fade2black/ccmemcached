#[derive(Debug)]
pub enum Command {
    Set(SetCommand),
    Get(GetCommand),
}

#[derive(Debug, Default)]
pub struct SetCommand {
    pub key: String,
    pub flags: u32,
    pub expire_time: i64,
    pub byte_count: usize,
    pub noreply: bool,
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct GetCommand {
    pub key: String,
}
