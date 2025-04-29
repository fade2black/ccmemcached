use crate::Result;
use crate::command::SetCommand;
use crate::util::unix_timestamp_now;

pub const NEVER_EXPIRES: i64 = 0;
pub const IMMEDIATELY_EXPIRES: i64 = -1;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record {
    key: String,
    flags: u32,
    expire_time: i64,
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

    pub fn is_expired(&self) -> bool {
        if self.expire_time == NEVER_EXPIRES {
            return false;
        }

        if self.expire_time == IMMEDIATELY_EXPIRES {
            return true;
        }

        unix_timestamp_now() >= self.expire_time
    }

    fn expire_time(exp_time: i64) -> i64 {
        if exp_time > 0 {
            unix_timestamp_now() + exp_time
        } else {
            exp_time
        }
    }

    #[cfg(test)]
    pub fn new_with_expire_time(expire_time: i64) -> Self {
        Self {
            expire_time,
            ..Self::default()
        }
    }
}

impl From<SetCommand> for Record {
    fn from(cmd: SetCommand) -> Self {
        Record {
            expire_time: Self::expire_time(cmd.expire_time),
            key: cmd.key,
            flags: cmd.flags,
            byte_count: cmd.byte_count,
            noreply: cmd.noreply,
            data: cmd.data,
        }
    }
}

pub trait Storage {
    fn store(&mut self, key: String, record: Record);
    fn remove(&mut self, key: &str) -> Option<Record>;
    fn find(&mut self, key: &str) -> Option<&Record>;
    fn get(&self, key: &str) -> Option<&Record>;
    fn exists(&self, key: &str) -> bool;
}
