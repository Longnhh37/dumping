use std::fmt::Write as _;

use futures::channel::mpsc::Receiver;
use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{
        broadcast::error::{RecvError, TryRecvError},
        watch,
    },
    time::{Instant, sleep},
};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};
use tracing::warn;

use crate::{
    command::{self, Command, validate_nick, validate_room},
    error::ChatError,
    message::{ClientId, Message},
    room::{RoomRx, RoomTx, Rooms},
    server::Config,
};

const DEFAULT_ROOM: &str = "general";
const MAX_LINE_LEN: usize = 1024;

type Lines = FramedRead<OwnedReadHalf, LinesCodec>;

pub struct Context {
    pub rooms: Rooms,
    pub config: Config,
    pub shutdown: watch::Receiver<bool>,
}

#[derive(PartialEq, Eq)]
enum Flow {
    Continue,
    Quit,
}

struct Client {
    id: ClientId,
    nick: String,
    room: String,
    tx: RoomTx,
    rx: RoomRx,
    writer: OwnedWriteHalf,
    rooms: Rooms,
    config: Config,
    shutdown: watch::Receiver<bool>,
}

pub async fn handle_connection(
    socket: TcpStream,
    id: ClientId,
    ctx: Context,
) -> Result<(), ChatError> {
    let (read_half, write_half) = socket.into_split();
    let mut lines: Lines =
        FramedRead::new(read_half, LinesCodec::new_with_max_length(MAX_LINE_LEN));

    let (tx, rx) = ctx.rooms.join(DEFAULT_ROOM).await;
    let mut client = Client {
        id,
        nick: format!("guest-{id}"),
        room: DEFAULT_ROOM.to_owned(),
        tx,
        rx,
        writer: write_half,
        rooms: ctx.rooms,
        config: ctx.config,
        shutdown: ctx.shutdown,
    };

    let _ = client.tx.send(Message::system(
        id,
        format!("{} joined #{}", client.nick, client.room),
    ));

    let res = match client.welcome().await {
        Ok(()) => client.event_loop(&mut lines).await,
        Err(e) => Err(e),
    };
    client.leave().await;
    res
}

impl Client {
    async fn welcome(&mut self) -> Result<(), ChatError> {
        let text = format!(
            "Welcome, {}! You are in #{}. Commands: /join <room>, /nick <name>, /rooms, /quit",
            self.nick, self.room
        );
        self.reply(&text).await
    }

    async fn event_loop(&mut self, lines: &mut Lines) -> Result<(), ChatError> {
        let idle = sleep(self.config.idle_timeout);
        tokio::pin!(idle);

        loop {
            tokio::select! {
                frame = lines.next() => match frame {
                    None => break,
                    Some(Ok(line)) => {
                        idle.as_mut().reset(Instant::now() + self.config.idle_timeout);
                    }
                }
            }
        }
    }
}
