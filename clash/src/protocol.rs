use anyhow::Result;
use bytes::{Buf, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use thiserror::Error;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, io::BufWriter, net::TcpStream};

use crate::lobby::{LobbyOptions, SharedLobby};
use crate::player::PlayerOptions;
use crate::room::Room;
use crate::spatula::Spatula;
use crate::{LobbyId, PlayerId};

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum ProtocolError {
    #[error("Invalid Player ID {0:#X}")]
    InvalidPlayerId(PlayerId),
    #[error("Invalid Lobby ID {0:#X}")]
    InvalidLobbyId(LobbyId),
    #[error("Invalid Message")]
    InvalidMessage,
    // TODO: This probably shouldn't be an error
    #[error("Player disconnected")]
    Disconnected,
    #[error("Client version '{0}' does not match server version '{1}'")]
    VersionMismatch(String, String),
}

// TODO: Take more advantage of the type system (e.g. Client/Server messages)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Message {
    Version { version: String },
    Error { error: ProtocolError },
    ConnectionAccept { player_id: u32 },
    PlayerOptions { options: PlayerOptions },
    GameHost,
    GameJoin { lobby_id: u32 },
    GameOptions { options: LobbyOptions },
    GameLobbyInfo { lobby: SharedLobby },
    GameBegin,
    GameCurrentRoom { room: Option<Room> },
    GameForceWarp { room: Room },
    GameItemCollected { item: Item },
    GameEnd,
    GameLeave,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Item {
    Spatula(Spatula),
}

#[derive(Debug, Error)]
pub enum FrameError {
    #[error("Frame exceeded max length")]
    FrameLength,
    #[error("Full frame is not available yet.")]
    FrameIncomplete,
    #[error("Connection reset by peer")]
    ConnectionReset,
    #[error("I/O Error: {0}")]
    Io(std::io::Error),
    #[error("Serialization Error: {0}")]
    Bincode(bincode::Error),
}

impl From<std::io::Error> for FrameError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<bincode::Error> for FrameError {
    fn from(e: bincode::Error) -> Self {
        Self::Bincode(e)
    }
}

#[derive(Debug)]
pub struct Connection {
    read_stream: OwnedReadHalf,
    write_stream: BufWriter<OwnedWriteHalf>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        let (read_stream, write_stream) = socket.into_split();
        Self {
            read_stream,
            write_stream: BufWriter::new(write_stream),
            buffer: BytesMut::with_capacity(64),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Message>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.read_stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    // Remote closed Connection
                    return Ok(None);
                } else {
                    // Connection closed while still sending data
                    return Err(FrameError::ConnectionReset.into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Message>, tokio::io::Error> {
        // Use a Cursor to avoid advancing the internal cursor of self.buffer
        let mut buf = Cursor::new(&self.buffer[..]);

        if self.buffer.len() < 2 {
            return Ok(None);
        }

        // Check if the buffer contains the full message yet
        let message_len = buf.get_u16().into();
        if self.buffer.remaining() < message_len + std::mem::size_of::<u16>() {
            return Ok(None);
        }

        // Consume the frame from the buffer and deserialize a message
        self.buffer.advance(std::mem::size_of::<u16>());
        let message = bincode::deserialize::<Message>(&self.buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        self.buffer.advance(message_len);

        Ok(Some(message))
    }

    pub async fn write_frame(&mut self, frame: Message) -> Result<(), FrameError> {
        let mut bytes: Bytes = bincode::serialize(&frame)?.into();
        if bytes.len() > u16::MAX.into() {
            return Err(FrameError::FrameLength);
        }
        let len = bytes.len() as u16;
        let len = len.to_be_bytes();
        self.write_stream.write(&len).await?;
        self.write_stream.write_buf(&mut bytes).await?;
        self.write_stream.flush().await?;
        Ok(())
    }
}
