use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex, RwLock}, time::{Duration, Instant}};
use std::thread::{self, JoinHandle};

use actix::prelude::*;
use rand::{Rng, rngs::ThreadRng};

use crate::{bullet::{self, Bullet}, keystate::KeyState, ship::{self, Ship}};

#[derive(Debug)]
enum GameObject {
    Ship(ship::Ship),
    Bullet(bullet::Bullet),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

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
#[rtype(result = "usize")]
pub struct GetPlayerCount {
    pub room_id: usize
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<usize>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub room: usize,
}

#[derive(Debug)]
pub struct GameThread {
    thread: JoinHandle<()>,
    // (RoomID, [PlayerID])
    players: Arc<Mutex<HashMap<usize, HashSet<usize>>>>,
    // (PlayerID, PlayerSession)
    sessions: Arc<Mutex<HashMap<usize, Recipient<Message>>>>,
    // (PlayerID, KeyState)
    keystates: Arc<Mutex<HashMap<usize, KeyState>>>,
}

impl GameThread {
    pub fn new() -> Self {
        let players = Arc::new(Mutex::new(HashMap::new()));
        let sessions = Arc::new(Mutex::new(HashMap::new()));
        let keystates = Arc::new(Mutex::new(HashMap::new()));

        // (RoomID, [Ship])
        let ships: HashMap<usize, HashSet<Ship>> = HashMap::new();
        // (RoomID, [Bullet])
        let bullets: HashMap<usize, HashSet<Bullet>> = HashMap::new();
        // (RoomID, isPlaying)
        let is_playing: HashMap<usize, bool> = HashMap::new();

        let players_cloned = players.clone();
        let sessions_cloned = sessions.clone();
        let keystates_cloned = keystates.clone();

        let thread = thread::spawn(move || {
            let mut before_frame = Instant::now();
            loop {
                let now = Instant::now();
                if now.duration_since(before_frame) < Duration::from_micros(16_666) {
                    continue;
                }

                before_frame = Instant::now();

                // Game Logic
                let players_in_room = players_cloned.lock().unwrap();
                for room_id in players_in_room.keys() {
                    if let Some(ships) = ships.get(room_id) {
                        for ship in ships {
                            // ship.process_one_frame(_);
                        }
                    }
                }
            }
        });

        Self {
            thread,
            players,
            sessions,
            keystates,
        }
    }
}

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<usize, HashSet<usize>>,
    rng: ThreadRng,
    thread: GameThread,
}

impl GameServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::rng(),
            thread: GameThread::new(),
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.rng.random_range(0..=usize::MAX)
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ListRooms> for GameServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _msg: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(*key);
        }

        MessageResult(rooms)
    }
}

impl Handler<Join> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) {
        let Join { id, room } = msg;

        self.rooms.entry(room).or_default().insert(id);
    }
}

impl Handler<GetPlayerCount> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: GetPlayerCount, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get(&msg.room_id) {
            room.len()
        } else {
            0
        }
    }
}
