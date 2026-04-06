use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq)]
pub enum Command {
    Health,
    Insert,
    Get,
    Delete,
}

impl Command {
    pub fn parse(s: &str) -> Result<Command> {
        let c = s.split_once(";").unwrap_or(("", ""));

        let command = match c.0 {
            "!" => Command::Health,
            "+" => Command::Insert,
            "?" => Command::Get,
            "-" => Command::Delete,
            _ => {
                return Err(anyhow!("unknown command"));
            }
        };

        Ok(command)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_health() {
        let c = "!;";

        let result = Command::parse(c).unwrap();
        assert_eq!(result, Command::Health);
    }

    #[test]
    fn test_parse_insert() {
        let c = "+;foo$3;bar";
        let result = Command::parse(c).unwrap();
        assert_eq!(result, Command::Insert);
    }

    #[test]
    fn test_parse_get() {
        let c = "?;foo$3;bar";
        let result = Command::parse(c).unwrap();
        assert_eq!(result, Command::Get);
    }

    #[test]
    fn test_parse_delete() {
        let c = "-;foo$3;bar";
        let result = Command::parse(c).unwrap();
        assert_eq!(result, Command::Delete);
    }
}
