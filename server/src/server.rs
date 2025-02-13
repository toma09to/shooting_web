use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::keystate::KeyState;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub KeyState);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub key_state: KeyState,
    pub room: usize,
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct GetPlayerCount;

pub struct ListRooms;
impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub room: usize,
}

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<usize, HashSet<usize>>,
    rng: ThreadRng,
}

impl Actor for GameServer {
    type Context = Context<Self>;
}
