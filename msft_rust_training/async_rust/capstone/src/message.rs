use std::sync::Arc;

pub type ClientId = u64;
pub const SERVER_ID: ClientId = 0;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: ClientId,
    body: Arc<str>,
}

impl Message {
    // reguler chat line: `nick: text`
    pub fn chat(sender: ClientId, nick: &str, text: &str) -> Self {
        Self::new(sender, format!("{nick}: {text}\n"))
    }

    // system notice such as "alice joined #general"
    pub fn system(sender: ClientId, text: impl AsRef<str>) -> Self {
        Self::new(sender, format!("* {}\n", text.as_ref()))
    }

    // a notice broadcast from the server
    pub fn server(text: impl AsRef<str>) -> Self {
        Self::system(SERVER_ID, text)
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    fn new(sender: ClientId, body: String) -> Self {
        Self {
            sender,
            body: body.into(),
        }
    }
}
