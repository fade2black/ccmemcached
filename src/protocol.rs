use crate::{
    AppError, Result,
    command::{Command, GetCommand, SetCommand},
    storage::{Record, Storage},
};
use std::str;
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error};

type ParserFn = fn(Vec<&str>) -> Result<Command>;

/// Memcached protocol documentation: https://github.com/memcached/memcached/blob/master/doc/protocol.txt
pub fn parse_command(line: &str) -> Result<Command> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 0 {
        error!("invalid command");
        return Err(AppError::InvalidCommand);
    }

    let cmd_name = parts[0];

    let mut parsers: HashMap<&str, ParserFn> = HashMap::new();
    parsers.insert("set", parse_set_cmd);
    parsers.insert("get", parse_get_cmd);

    if let Some(func) = parsers.get(cmd_name) {
        func(parts)
    } else {
        error!("unexpected command `{}`", cmd_name);
        Err(AppError::UnexpectedCommand(cmd_name.into()))
    }
}

pub fn execute<T>(command: Command, storage: Arc<RwLock<T>>) -> Result<String>
where
    T: Storage,
{
    match command {
        Command::Get(command) => exec_get_cmd(command, storage),
        Command::Set(command) => exec_set_cmd(command, storage),
    }
}

fn parse_get_cmd(parts: Vec<&str>) -> Result<Command> {
    let mut command = GetCommand::default();
    command.key = parts[1].to_string();

    Ok(Command::Get(command))
}

fn parse_set_cmd(parts: Vec<&str>) -> Result<Command> {
    let mut command = SetCommand::default();

    command.key = parts[1].to_string();
    command.flags = parts[2].parse::<u32>()?;
    command.expire_time = parts[3].parse::<i32>()?;
    command.byte_count = parts[4].parse::<usize>()?;
    command.noreply = parts.len() > 5 && parts[5] == "noreply";

    Ok(Command::Set(command))
}

fn exec_set_cmd<T>(command: SetCommand, storage: Arc<RwLock<T>>) -> Result<String>
where
    T: Storage,
{
    debug!(
        "Executing: SET {}, {:?}",
        command.key,
        std::str::from_utf8(&command.data)
    );
    let noreply = command.noreply;
    let mut ptr = storage.write().map_err(|_| AppError::StateAccessError)?;

    ptr.store(command.key.clone(), Record::from(command));

    if noreply {
        Ok("".to_string())
    } else {
        Ok("STORED\r\n".to_string())
    }
}

fn exec_get_cmd<T>(command: GetCommand, storage: Arc<RwLock<T>>) -> Result<String>
where
    T: Storage,
{
    debug!("Executing: GET {}", command.key);

    let ptr = storage.read().map_err(|_| AppError::StateAccessError)?;

    let resp = match ptr.find(&command.key) {
        Some(record) => record.to_string().map_err(|_| AppError::StateAccessError)?,
        None => "".to_string(),
    };

    Ok(resp + "END\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_with_noreply() {
        let line = "set test 6 0 10 noreply";
        let command = parse_command(line).unwrap();

        if let Command::Set(cmd) = command {
            assert_eq!(cmd.key, "test");
            assert_eq!(cmd.flags, 6);
            assert_eq!(cmd.expire_time, 0);
            assert_eq!(cmd.byte_count, 10);
            assert!(cmd.noreply);
        } else {
            panic!("Expected Command::Set, but got a different variant.");
        }
    }

    #[test]
    fn test_parse_command_without_noreply() {
        let line = "set test 6 0 10";
        let command = parse_command(line).unwrap();

        if let Command::Set(cmd) = command {
            assert_eq!(cmd.key, "test");
            assert_eq!(cmd.flags, 6);
            assert_eq!(cmd.expire_time, 0);
            assert!(!cmd.noreply);
        } else {
            panic!("Expected Command::Set, but got a different variant.");
        }
    }

    #[test]
    fn test_parse_invalid_command() {
        let line = "";
        let result = parse_command(line);
        assert!(matches!(result, Err(AppError::InvalidCommand)));
    }

    #[test]
    fn test_parse_unexpected_command() {
        let line = "abc 1 2 3";
        let result = parse_command(line);
        assert!(matches!(result, Err(AppError::UnexpectedCommand(_))));
    }
}
