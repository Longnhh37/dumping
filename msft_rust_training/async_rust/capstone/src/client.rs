use std::fmt::Write as _;

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
                        if self.handle_line(&line).await? == Flow::Quit {
                            break;
                        }
                    }
                    Some(Err(LinesCodecError::MaxLineLengthExceeded)) => {
                        self.reply("* line too long, disconnecting").await?;
                        break;
                    }
                    Some(Err(LinesCodecError::Io(e))) => return Err(e.into()),
                },

                received = self.rx.recv() => match received {
                    Ok(msg) => self.deliver(&msg).await?,
                    Err(RecvError::Lagged(skipped)) => self.notify_lagged(skipped).await?,
                    Err(RecvError::Closed) => break,
                },

                () = &mut idle => {
                        let text = format!(
                        "* disconnected: idle for more than {:?}",
                            self.config.idle_timeout
                        );
                    self.reply(&text).await?;
                    break;
                },

                _ = self.shutdown.changed() => {
                    self.drain().await?;
                    break;
                },
            };
        }

        Ok(())
    }

    async fn drain(&mut self) -> Result<(), ChatError> {
        loop {
            match self.rx.try_recv() {
                Ok(msg) => self.deliver(&msg).await?,
                Err(TryRecvError::Lagged(skipped)) => self.notify_lagged(skipped).await?,
                Err(TryRecvError::Empty | TryRecvError::Closed) => return Ok(()),
            }
        }
    }
    async fn handle_line(&mut self, line: &str) -> Result<Flow, ChatError> {
        match command::parse(line) {
            Command::Empty => {}

            Command::Say(text) => {
                let _ = self.tx.send(Message::chat(self.id, &self.nick, text));
            }

            Command::Nick(raw) => match validate_nick(raw) {
                Ok(new_nick) => {
                    let old = std::mem::replace(&mut self.nick, new_nick);
                    let _ = self.tx.send(Message::system(
                        self.id,
                        format!("{old} is now known as {}", self.nick),
                    ));
                    self.reply(&format!("* you are now known as {}", self.nick))
                        .await?;
                }
                Err(e) => self.reply(&format!("* error: {e}")).await?,
            },

            Command::Join(raw) => match validate_room(raw) {
                Ok(room) => self.switch_room(room).await?,
                Err(e) => self.reply(&format!("* error: {e}")).await?,
            },

            Command::Rooms => {
                let rooms = self.rooms.list().await;
                let mut out = format!("* active rooms ({}):", rooms.len());
                for (name, members) in &rooms {
                    let _ = write!(out, "\n\t#{name} ({members})");
                }
                self.reply(&out).await?;
            }

            Command::Quit => {
                self.reply("* bye!").await?;
                return Ok(Flow::Quit);
            }

            Command::Unknown(cmd) => {
                let text = format!("* unknown command /{cmd} (try /join, /nick, /rooms, /quit)");
                self.reply(&text).await?;
            }
        }

        Ok(Flow::Continue)
    }

    async fn switch_room(&mut self, target: String) -> Result<(), ChatError> {
        if target == self.room {
            return self.reply(&format!("* you are already in #{target}")).await;
        }
        let _ = self.tx.send(Message::system(
            self.id,
            format!("{} left #{}", self.nick, self.room),
        ));
        let (tx, rx) = self.rooms.join(&target).await;
        let old_room = std::mem::replace(&mut self.room, target);
        (self.tx, self.rx) = (tx, rx);
        self.rooms.leave(&old_room).await;

        let _ = self.tx.send(Message::system(
            self.id,
            format!("{} joined #{}", self.nick, self.room),
        ));
        self.reply(&format!("* you joined #{}", self.room)).await
    }

    async fn deliver(&mut self, msg: &Message) -> Result<(), ChatError> {
        if msg.sender != self.id {
            self.writer.write_all(msg.body().as_bytes()).await?;
        }
        Ok(())
    }

    async fn notify_lagged(&mut self, skipped: u64) -> Result<(), ChatError> {
        warn!(
            client = self.id,
            skipped, "receiver lagged, messages dropped"
        );
        self.reply(&format!(
            "* you are too slow: {skipped} messages were skipped"
        ))
        .await
    }

    async fn reply(&mut self, text: &str) -> Result<(), ChatError> {
        self.writer
            .write_all(format!("{text}\n").as_bytes())
            .await?;
        Ok(())
    }

    async fn leave(mut self) {
        let _ = self.writer.shutdown().await;
        let _ = self.tx.send(Message::system(
            self.id,
            format!("{} left #{}", self.nick, self.room),
        ));
        let Client {
            rx, room, rooms, ..
        } = self;
        drop(rx);
        rooms.leave(&room).await;
    }
}
