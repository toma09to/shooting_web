use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use serde::Deserialize;

use crate::{keystate::KeyState, server};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug)]
pub struct GameSession {
    pub id: usize,
    pub hb: Instant,
    pub room: usize,
    pub addr: Addr<server::GameServer>,
    pub watch: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "keystate")]
    KeyState {
        data: KeyState,
    },
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "finish")]
    Finish,
    Error(String),
}

impl GameSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect {
                    id: act.id,
                    room: act.room,
                });
                ctx.stop();
                return;
            }

            ctx.text(serde_json::to_string(&server::Message::Ping).unwrap());
        });
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        self.addr
            .send(server::Connect)
            .into_actor(self)
            .then(|res, act, ctx| {
                log::debug!("connect response: {:?}", res);
                match res {
                    Ok(res) => {
                        act.id = res;
                    },
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect {
            id: self.id,
            room: self.room,
        });
        Running::Stop
    }
}

impl Handler<server::Message> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            },
            Ok(msg) => msg,
        };

        log::debug!("Websocket ID: {0:?}", self.id);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            },
            ws::Message::Text(text) => {
                let msg: ClientMessage = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(_) => ClientMessage::Error(text.to_string()),
                };

                log::debug!("{msg:?}");
                match msg {
                    ClientMessage::Join => {
                        self.addr
                            .send(server::Join {
                                id: self.id,
                                room: self.room,
                                addr: ctx.address().recipient(),
                                watch: self.watch,
                            })
                            .into_actor(self)
                            .then(|res, _act, ctx| {
                                match res {
                                    Ok(true) => (),
                                    _ => ctx.stop(),
                                }
                                fut::ready(())
                            })
                            .wait(ctx);
                    },
                    ClientMessage::KeyState { data } => {
                        self.addr
                            .send(server::KeyUpdate {
                                id: self.id,
                                state: data,
                            })
                            .into_actor(self)
                            .then(|res, _act, ctx| {
                                if let Err(_) = res {
                                    ctx.stop();
                                }
                                fut::ready(())
                            })
                            .wait(ctx);
                    },
                    ClientMessage::Pong => {
                        self.hb = Instant::now();
                    },
                    ClientMessage::Finish => {
                        self.addr
                            .send(server::DeleteRoom {
                                room_id: self.room
                            })
                            .into_actor(self)
                            .then(|_res, _act, _ctx| fut::ready(()))
                            .wait(ctx);
                    }
                    ClientMessage::Error(text) => log::warn!("Invalid message: {text}"),
                }
            },
            ws::Message::Binary(_) => log::warn!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            },
            ws::Message::Continuation(_) => {
                ctx.stop();
            },
            ws::Message::Nop => (),
        }
    }
}
