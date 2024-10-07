use serde::{Serialize, Deserialize};

/// Client sends a Message enum to server. 
/// If Global send to everyone on server, including self.
/// If Direct; send to server and server sends to peer.
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Direct(DirectContent),
    Global(Content),
}

/// Content of a Global message.
/// File field includes a Base64 string
/// of the file and the message text.
/// Sender is infered from server/client-ip.
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub text: String,
    pub file: Option<String>,
}

/// Same as Content, but peer is sender to server,
/// and after leaving the server; peer is the
/// reciever of the message/client.
#[derive(Serialize, Deserialize, Debug)]
pub struct DirectContent {
    pub text: String,
    pub file: Option<String>,
    pub peer: String,
}

/// Sent on connection to server.
/// Failes, returning Response(InvaildUsername), on non-unique username.
#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    name: String,
    color: Color,
}

/// Color of username in chat.
#[derive(Serialize, Deserialize, Debug)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Cyan,
    Cerise,
    Purple,
}

/// On Login; returned InvaildUsername if non-unique username.
/// On Direct message; InvaildUsername if peer doesn't exist.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok,
    InvaildUsername,
}

/// On connection, server sends the Version number
/// the server is running on.
#[derive(Serialize, Deserialize, Debug)]
pub struct Version(pub u16);

/*
    Impls
*/

impl TryFrom<&[u8]> for Message {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Message> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Content {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Content> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Content) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for DirectContent {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<DirectContent> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: DirectContent) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Login {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Login> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Login) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Color> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Color) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Response> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Version> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Version) -> Result<Self, Self::Error> {
        rmp_serde::to_vec(&value)
    }
}
