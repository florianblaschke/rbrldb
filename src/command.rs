use anyhow::{Result, anyhow};
use bytes::BytesMut;
use std::str::from_utf8;

#[derive(Debug)]
pub enum CommandType {
    Health,
    Insert,
    Get,
    Delete,
}
#[derive(Debug)]
pub struct Payload {
    pub key: String,
    pub value: String,
}

pub struct Command {
    pub kind: CommandType,
    pub payload: Option<Payload>,
}

impl Command {
    pub fn new(line: BytesMut) -> Result<Command> {
        let (kind, payload) = parse(&line)?;
        Ok(Command { kind, payload })
    }
}

fn parse(b: &BytesMut) -> Result<(CommandType, Option<Payload>)> {
    let s = from_utf8(&b)?;
    let (command_type, rest) = s
        .split_once(";")
        .ok_or_else(|| anyhow!("invalid command, missing ;"))?;

    let command = match command_type {
        "!" => (CommandType::Health, None),
        "+" => {
            let payload = parse_insert_payload(rest)?;
            (CommandType::Insert, payload)
        }
        "?" => {
            let payload = Some(Payload {
                key: rest.into(),
                value: "".into(),
            });
            (CommandType::Get, payload)
        }
        "-" => (CommandType::Delete, None),
        _ => {
            return Err(anyhow!("unknown command"));
        }
    };

    Ok(command)
}

fn parse_insert_payload(s: &str) -> Result<Option<Payload>> {
    let (key, value) = s
        .split_once(";")
        .ok_or_else(|| anyhow!("invalid insert command, missing key or/and value"))?;
    let (raw_key, _) = key
        .split_once("$")
        .ok_or_else(|| anyhow!("invalid insert command, missing size parameter"))?;

    Ok(Some(Payload {
        key: raw_key.into(),
        value: value.into(),
    }))
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_parse_get_payload() {
//         let s = "?;foo";

//         let result = parse_get_payload(s).unwrap().unwrap();
//         assert_eq!(result.key, "foo".to_string());
//     }
// }
