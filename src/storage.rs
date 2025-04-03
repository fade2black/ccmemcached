use crate::Result;
use crate::command::SetCommand;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record {
    key: String,
    flags: u32,
    expire_time: u32,
    byte_count: usize,
    noreply: bool,
    pub data: Vec<u8>,
}

impl Record {
    pub fn to_string(&self) -> Result<String> {
        let data = std::str::from_utf8(&self.data)?;
        Ok(format!(
            "VALUE {} {} {}\r\n{}\r\n",
            self.key, self.flags, self.byte_count, data
        ))
    }
}

impl From<SetCommand> for Record {
    fn from(cmd: SetCommand) -> Self {
        Record {
            key: cmd.key,
            flags: cmd.flags,
            expire_time: cmd.expire_time,
            byte_count: cmd.byte_count,
            noreply: cmd.noreply,
            data: cmd.data,
        }
    }
}

pub trait Storage {
    fn store(&mut self, key: String, record: Record);
    fn find(&self, key: &String) -> Option<&Record>;
}
