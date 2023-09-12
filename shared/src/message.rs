#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub enum ClientMessage {
    Text(String),
    Ping,
    Pong,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub enum ServerMessage {
    Text(String),
    Ping,
    Pong,
}

impl networking::Message for ClientMessage {
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }
    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_ping() -> Self {
        Self::Ping
    }
    fn default_pong() -> Self {
        Self::Pong
    }
}

impl networking::Message for ServerMessage {
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }
    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_ping() -> Self {
        Self::Ping
    }
    fn default_pong() -> Self {
        Self::Pong
    }
}