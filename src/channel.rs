use anyhow::{Result, anyhow};
use bytes::BytesMut;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

use crate::{
    command::{Command, CommandType},
    memory::{Db, Store, Value},
};

pub struct ChannelPayload {
    command: Command,
    responder: oneshot::Sender<Vec<u8>>,
}

enum Response {
    Ok,
    NotFound,
    Error,
}

impl AsRef<[u8]> for Response {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Ok => b"ok",
            Self::Error => b"error",
            Self::NotFound => b"nf",
        }
    }
}

pub fn spawn_channel() -> Sender<ChannelPayload> {
    let mut db = Db::new();
    let (tx, mut rx) = mpsc::channel::<ChannelPayload>(32);

    tokio::spawn(async move {
        while let Some(ChannelPayload { command, responder }) = rx.recv().await {
            match command.kind {
                CommandType::Health => {
                    responder.send(Response::Ok.as_ref().into()).unwrap();
                }
                CommandType::Insert => {
                    if let Some(payload) = command.payload {
                        db.set(
                            payload.key,
                            Value {
                                data: payload.value.into(),
                                ttl: None,
                            },
                        );
                        responder.send(Response::Ok.as_ref().into()).unwrap();
                    } else {
                        responder.send(Response::Error.as_ref().into()).unwrap();
                    }
                }
                CommandType::Get => {
                    if let Some(payload) = command.payload {
                        match db.get(&payload.key) {
                            Ok(v) => {
                                responder.send(v).unwrap();
                            }
                            Err(_) => {
                                responder.send(Response::NotFound.as_ref().into()).unwrap();
                            }
                        }
                    } else {
                        responder.send(Response::Error.as_ref().into()).unwrap();
                    }
                }
                CommandType::Delete => panic!("not implemented delete"),
            };
        }
    });

    tx
}

pub async fn send_command_to_channel(
    channel: &Sender<ChannelPayload>,
    bytes: &BytesMut,
) -> Result<Vec<u8>> {
    let (send, receive) = oneshot::channel::<Vec<u8>>();
    let command = match Command::new(bytes.to_owned()) {
        Ok(c) => c,
        Err(_) => return Err(anyhow!("unable to parse command")),
    };

    let _ = channel
        .send(ChannelPayload {
            command,
            responder: send,
        })
        .await;

    match receive.await {
        Ok(v) => Ok(v),
        Err(_) => Err(anyhow!("unable to send command")),
    }
}
