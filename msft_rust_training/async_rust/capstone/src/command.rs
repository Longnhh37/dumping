use crate::error::ChatError;

pub const MAX_NICK_LEN: usize = 20;
pub const MAX_ROOM_LEN: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Join(&'a str),
    Nick(&'a str),
    Rooms,
    Quit,
    Say(&'a str),
    Empty,
    Unknown(&'a str),
}

pub fn parse(line: &str) -> Command<'_> {
    let line = line.trim();
    if line.is_empty() {
        return Command::Empty;
    }

    let Some(rest) = line.strip_prefix('/') else {
        return Command::Say(line);
    };

    let (name, arg) = match rest.split_once(char::is_whitespace) {
        Some((name, arg)) => (name, arg.trim()),
        None => (rest, ""),
    };

    match name.to_ascii_lowercase().as_str() {
        "join" => Command::Join(arg),
        "nick" => Command::Nick(arg),
        "rooms" => Command::Rooms,
        "quit" => Command::Quit,
        _ => Command::Unknown(name),
    }
}

fn is_valid_token(s: &str, max: usize) -> bool {
    !s.is_empty()
        && s.chars().count() <= max
        && !s.chars().any(|c| c.is_whitespace() || c.is_control())
}

pub fn validate_nick(raw: &str) -> Result<String, ChatError> {
    let nick = raw.trim();
    if is_valid_token(nick, MAX_NICK_LEN) {
        Ok(nick.to_owned())
    } else {
        Err(ChatError::InvalidNick { max: MAX_NICK_LEN })
    }
}

pub fn validate_room(raw: &str) -> Result<String, ChatError> {
    let raw = raw.trim();
    let room = raw.strip_prefix('#').unwrap_or(raw);
    if is_valid_token(room, MAX_ROOM_LEN) {
        Ok(room.to_lowercase())
    } else {
        Err(ChatError::InvalidRoom { max: MAX_ROOM_LEN })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands() {
        assert_eq!(parse("/join rust"), Command::Join("rust"));
        assert_eq!(parse("  /JOIN   rust  "), Command::Join("rust"));
        assert_eq!(parse("/nick alice"), Command::Nick("alice"));
        assert_eq!(parse("/rooms"), Command::Rooms);
        assert_eq!(parse("/quit"), Command::Quit);
        assert_eq!(parse("/join"), Command::Join(""));
        assert_eq!(parse("/dance"), Command::Unknown("dance"));
    }

    #[test]
    fn plain_text_is_a_chat_message() {
        assert_eq!(parse("hello world"), Command::Say("hello world"));
        assert_eq!(parse("   "), Command::Empty);
    }

    #[test]
    fn nick_validation() {
        assert!(validate_nick("alice").is_ok());
        assert!(validate_nick("").is_err());
        assert!(validate_nick("bob smith").is_err());
        assert!(validate_nick(&"a".repeat(MAX_NICK_LEN + 1)).is_err());
    }

    #[test]
    fn room_validation_normalizes() {
        assert_eq!(validate_room("#Rust").unwrap(), "rust");
        assert!(validate_room("").is_err());
        assert!(validate_room("#").is_err());
    }
}
