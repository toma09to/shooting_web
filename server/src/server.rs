use std::{collections::{HashMap, HashSet, VecDeque}, sync::{Arc, Mutex}, time::{Duration, Instant}};
use std::thread;
use std::f32::consts::PI;

use actix::prelude::*;
use rand::{Rng, rngs::ThreadRng};
use serde::Serialize;
use vector::Vector;

use crate::{bullet::{self, Bullet}, keystate::KeyState, ship::{self, Ship}};

const COLOR_LIST: [&str; 4] = [
    "#00ff00", // Green
    "#ff0000", // Red
    "#0080ff", // Blue
    "#ffff00", // Yellow
];
const ORDINAL_NUMBER: [&str; 4] = [
    "1st",
    "2nd",
    "3rd",
    "4th",
];

#[derive(Debug, Clone, Serialize)]
pub struct Text {
    color: String,
    pos: Vector,
    text: String,
}

impl Text {
    pub fn new(color: String, pos: Vector, text: String) -> Self {
        Self { color, pos, text }
    }

    fn with_color_num(num: u8, x: f32, y: f32, text: String) -> Self {
        Self {
            color: COLOR_LIST[num as usize].to_string(),
            pos: Vector::new(x, y),
            text,
        }
    }

    pub fn player_num(num: u8) -> Self {
        Self::with_color_num(num, 300.0, 100.0,format!("You are Player {}", num + 1).to_string())
    }

    pub fn you(num: u8) -> Self {
        Self::with_color_num(num, 150.0 + 100.0 * num as f32, 250.0, "YOU".to_string())
    }

    pub fn space_to_ready(num: u8) -> Self {
        Self::with_color_num(num, 300.0, 500.0, "Space to Ready".to_string())
    }

    pub fn ranking(ord: usize, num: u8) -> Self {
        Self::with_color_num(
            num,
            300.0,
            200.0 + 50.0 * ord as f32,
            format!("{} Player{}", ORDINAL_NUMBER[ord], num + 1).to_string(),
        )
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum GameObject {
    #[serde(rename = "ship")]
    Ship {
        data: ship::Ship
    },
    #[serde(rename = "bullet")]
    Bullet {
        data: bullet::Bullet
    },
    #[serde(rename = "text")]
    Text {
        data: Text
    },
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "objects")]
    Objects {
        data: Vec<GameObject>,
    },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "finish")]
    Finish {
        data: Vec<GameObject>,
    },
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room: usize,
    pub watch: bool,
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct CreateRoom;

#[derive(Message)]
#[rtype(result = "()")]
pub struct DeleteRoom {
    pub room_id: usize,
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
#[rtype(result = "bool")]
pub struct Join {
    pub id: usize,
    pub room: usize,
    pub addr: Recipient<Message>,
    pub watch: bool,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct KeyUpdate {
    pub id: usize,
    pub state: KeyState,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct IsPlaying {
    pub room_id: usize,
}

#[derive(Debug, Clone)]
pub struct GameData {
    // (RoomID, [PlayerID])
    pub listeners: Arc<Mutex<HashMap<usize, HashSet<usize>>>>,
    // (RoomID, [(PlayerID, Ship)])
    pub ships: Arc<Mutex<HashMap<usize, HashMap<usize, Ship>>>>,
    // (RoomID, [Bullet])
    pub bullets: Arc<Mutex<HashMap<usize, Vec<Bullet>>>>,
    // (PlayerID, PlayerSession)
    pub sessions: Arc<Mutex<HashMap<usize, Recipient<Message>>>>,
    // (PlayerID, KeyState)
    pub keystates: Arc<Mutex<HashMap<usize, KeyState>>>,
    // (RoomID, isPlaying)
    pub is_playing: Arc<Mutex<HashMap<usize, bool>>>,
    // (RoomID, [PlayerNum])
    pub ranking: Arc<Mutex<HashMap<usize, Vec<u8>>>>,
}

#[derive(Debug)]
pub struct GameThread {
    data: GameData,
}

impl GameThread {
    pub fn new() -> Self {
        let listeners = Arc::new(Mutex::new(HashMap::<usize, HashSet<usize>>::new()));
        let ships = Arc::new(Mutex::new(HashMap::<usize, HashMap<usize, Ship>>::new()));
        let bullets = Arc::new(Mutex::new(HashMap::<usize, Vec<Bullet>>::new()));
        let sessions = Arc::new(Mutex::new(HashMap::<usize, Recipient<Message>>::new()));
        let keystates = Arc::new(Mutex::new(HashMap::new()));
        let is_playing = Arc::new(Mutex::new(HashMap::new()));
        let ranking = Arc::new(Mutex::new(HashMap::<usize, Vec<u8>>::new()));

        let listeners_cloned = listeners.clone();
        let ships_cloned = ships.clone();
        let bullets_cloned = bullets.clone();
        let sessions_cloned = sessions.clone();
        let keystates_cloned = keystates.clone();
        let is_playing_cloned = is_playing.clone();
        let ranking_cloned = ranking.clone();

        let mut all_ready_time: HashMap<usize, Instant> = HashMap::new();

        thread::spawn(move || {
            let mut before_frame = Instant::now();

            loop {
                let now = Instant::now();
                // 60fps
                if now.duration_since(before_frame) < Duration::from_micros(16_666) {
                    continue;
                }

                before_frame = Instant::now();

                // Game Logic
                let mut objects = Vec::new();
                let mut ended_games = Vec::new();
                for (room_id, ships) in ships_cloned.lock().unwrap().iter_mut() {
                    objects.clear();

                    let mut all_ready = ships.len() > 1;
                    let mut not_broadcast_texts = HashMap::new();
                    for (player_id, ship) in ships.iter_mut() {
                        all_ready &= ship.is_ready;

                        if let Some(true) = is_playing_cloned.lock().unwrap().get(room_id) {
                            // Moves ships
                            ship.process_one_frame(
                                keystates_cloned.lock().unwrap().get(player_id).expect("KeyState not found")
                            );

                            // Generates a bullet from the ship
                            let mut bullets = bullets_cloned.lock().unwrap();

                            if keystates_cloned.lock().unwrap().get(player_id).expect("KeyState not found").space {
                                if let Some(bullet) = ship.fire() {
                                    log::debug!("{bullet:?}");
                                    bullets.get_mut(room_id).expect("Bullets not found").push(bullet);
                                }
                            }
                        } else {
                            // Display 'Ready' to all players
                            if ship.is_ready {
                                objects.push(GameObject::Text { data: ship.ready_text() });
                            }

                            // Display texts to one player
                            not_broadcast_texts.insert(*player_id, vec![
                                GameObject::Text { data: Text::player_num(ship.player_num) },
                                GameObject::Text { data: Text::you(ship.player_num) },
                                GameObject::Text { data: Text::space_to_ready(ship.player_num) }
                            ]);

                            // Makes a ship ready
                            if keystates_cloned.lock().unwrap().get(player_id).expect("KeyState not found").space {
                                ship.is_ready = true;
                            }
                        }

                        // Anti-flicker
                        if !ship.is_game_over() {
                            objects.push(GameObject::Ship { data: ship.clone() });
                        }
                    }

                    // If all ships are ready and the game is not being played
                    // all_ready_timing = now
                    if all_ready && !*is_playing_cloned.lock().unwrap().get(room_id).expect("isPlaying not found") {
                        if let None = all_ready_time.get(room_id) {
                            all_ready_time.insert(*room_id, Instant::now());
                        }
                    }

                    // ... after 1 second, starts the game
                    if let Some(all_ready_timing) = all_ready_time.get(room_id) {
                        if Instant::now().duration_since(*all_ready_timing) > Duration::from_secs(1) {
                            is_playing_cloned.lock().unwrap().insert(*room_id, true);
                            all_ready_time.remove(room_id);

                            for ship in ships.values_mut() {
                                ship.put_on_random_place();
                            }
                        }
                    }

                    // Moves bullets and removes dead bullets
                    let mut bullets = bullets_cloned.lock().unwrap();
                    let mut bullets_alive: Vec<Bullet> = bullets.get(room_id)
                        .expect("Bullets not found")
                        .iter()
                        .cloned()
                        .filter(|b| b.is_alive())
                        .collect();
                    
                    for bullet in bullets_alive.iter_mut() {
                        bullet.move_by_one_frame();
                        objects.push(GameObject::Bullet { data: bullet.clone() });
                    }

                    let mut dead_players_id = Vec::new();
                    for (player_id, ship) in ships.iter_mut() {
                        for bullet in &bullets_alive {
                            ship.collision_process(bullet);
                        }

                        if ship.is_game_over() {
                            dead_players_id.push((*player_id, ship.player_num));
                        }
                    }

                    // Updates data of bullets
                    let new_bullets = bullets.get_mut(room_id).expect("Bullets not found");
                    *new_bullets = bullets_alive;

                    // Send data of objects to clients
                    let listeners = listeners_cloned.lock().unwrap();
                    let listeners = listeners.get(room_id).expect("Listeners not found");
                    for player_id in listeners {
                        if let Some(session) = sessions_cloned.lock().unwrap().get(player_id) {
                            let mut data = objects.clone();
                            if let Some(not_broadcast_texts) = not_broadcast_texts.remove(player_id) {
                                data.extend(not_broadcast_texts);
                            }

                            session.do_send(Message::Objects { data });
                        }
                    }

                    // Delete dead ships
                    for (id, num) in dead_players_id {
                        ships.remove(&id);
                        ranking_cloned.lock().unwrap().get_mut(room_id).expect("Ranking not found")
                            .push(num);
                    }

                    // If the number of players is 1, game is over
                    if ships.len() == 1 && *is_playing_cloned.lock().unwrap().get(room_id).expect("isPlaying not found") {
                        ranking_cloned.lock().unwrap().get_mut(room_id).expect("Ranking not found")
                            .push(ships.values().next().unwrap().player_num);

                        let mut ranking = ranking_cloned.lock().unwrap().get_mut(room_id)
                            .expect("Ranking not found")
                            .clone();


                        ranking.reverse();
                        for player_id in listeners {
                            if let Some(session) = sessions_cloned.lock().unwrap().get(player_id) {
                                let ranking_texts: Vec<_> = ranking.iter().enumerate()
                                    .map(|(i, player_num)| {
                                        GameObject::Text { data: Text::ranking(i, *player_num) }
                                    })
                                    .collect();

                                let game_over = vec![
                                    GameObject::Text {
                                        data: Text::new(
                                            COLOR_LIST[0].to_string(),
                                            Vector::new(300.0, 100.0),
                                            "GAME OVER".to_string(),
                                        )
                                    }
                                ];

                                let data = [ranking_texts, game_over].concat();
                                session.do_send(Message::Finish { data });
                            }
                        }

                        ended_games.push(*room_id);
                    }
                }

                while let Some(room_id) = ended_games.pop() {
                    ships_cloned.lock().unwrap().remove(&room_id);
                }
            }
        });

        Self {
            data: GameData {
                listeners,
                ships,
                bullets,
                sessions,
                keystates,
                is_playing,
                ranking,
            }
        }
    }

    pub fn get_game_data(&self) -> GameData {
        self.data.clone()
    }
}

#[derive(Debug)]
pub struct GameServer {
    room_num: HashMap<usize, usize>,
    rng: ThreadRng,
    thread: GameThread,
    player_num_pool: HashMap<usize, VecDeque<u8>>,
}

impl GameServer {
    pub fn new() -> Self {
        Self {
            room_num: HashMap::new(),
            rng: rand::rng(),
            thread: GameThread::new(),
            player_num_pool: HashMap::new(),
        }
    }

    pub fn get_game_data(&self) -> GameData {
        self.thread.get_game_data()
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
        let Disconnect { id, room, watch } = msg;

        match self.room_num.get_mut(&room) {
            Some(0) => (),
            Some(room_num) => {
                if *room_num > 0 && !watch {
                    *room_num -= 1;
                }
            },
            None => return,
        }

        let game_data = self.get_game_data();
        let mut ships = game_data.ships.lock().unwrap();
        let mut sessions = game_data.sessions.lock().unwrap();
        let mut keystates = game_data.keystates.lock().unwrap();
        let mut listeners = game_data.listeners.lock().unwrap();

        if let Some(ships_in_room) = ships.get_mut(&room) {
            if let Some(player_ship) = ships_in_room.get(&id) {
                self.player_num_pool.get_mut(&room)
                    .expect("Room not found")
                    .push_back(player_ship.player_num);
            }
            ships_in_room.remove(&id);
        }
        keystates.remove(&id);
        sessions.remove(&id);
        listeners.get_mut(&room).unwrap().remove(&id);
    }
}

impl Handler<CreateRoom> for GameServer {
    type Result = usize;

    fn handle(&mut self, _msg: CreateRoom, _ctx: &mut Self::Context) -> Self::Result {
        let mut id = self.rng.random_range(0..10000);
        while let Some(_) = self.room_num.get(&id) {
            id = self.rng.random_range(0..10000);
        }

        self.room_num.insert(id, 0);

        let game_data = self.get_game_data();
        let mut listeners = game_data.listeners.lock().unwrap();
        let mut ships = game_data.ships.lock().unwrap();
        let mut bullets = game_data.bullets.lock().unwrap();
        let mut is_playing = game_data.is_playing.lock().unwrap();
        let mut ranking = game_data.ranking.lock().unwrap();

        listeners.insert(id, HashSet::new());
        ships.insert(id, HashMap::new());
        bullets.insert(id, Vec::new());
        is_playing.insert(id, false);
        ranking.insert(id, Vec::new());
        self.player_num_pool.insert(id, VecDeque::from([0, 1, 2, 3]));

        log::info!("Created room {id}");

        id
    }
}

impl Handler<DeleteRoom> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: DeleteRoom, _ctx: &mut Self::Context) {
        self.room_num.remove(&msg.room_id);
        self.player_num_pool.remove(&msg.room_id);

        let game_data = self.get_game_data();
        let mut listeners = game_data.listeners.lock().unwrap();
        let mut ships = game_data.ships.lock().unwrap();
        let mut bullets = game_data.bullets.lock().unwrap();
        let mut is_playing = game_data.is_playing.lock().unwrap();
        let mut ranking = game_data.ranking.lock().unwrap();

        listeners.remove(&msg.room_id);
        ships.remove(&msg.room_id);
        bullets.remove(&msg.room_id);
        is_playing.remove(&msg.room_id);
        if let Some(_) = ranking.remove(&msg.room_id) {
            log::info!("Deleted room {}", msg.room_id);
        }
    }
}

impl Handler<ListRooms> for GameServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _msg: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        let mut room_list = Vec::new();

        for (room_id, room_num) in &self.room_num {
            if *room_num > 0 {
                room_list.push(*room_id);
            }
        }

        MessageResult(room_list)
    }
}

impl Handler<Join> for GameServer {
    type Result = bool;

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> bool {
        let Join { id, room, addr, watch } = msg;

        let game_data = self.get_game_data();
        let mut listeners = game_data.listeners.lock().unwrap();
        let mut ships = game_data.ships.lock().unwrap();
        let mut sessions = game_data.sessions.lock().unwrap();
        let mut keystates = game_data.keystates.lock().unwrap();
        let is_playing = game_data.is_playing.lock().unwrap();

        if !watch {
            if let Some(room_num) = self.room_num.get_mut(&room) {
                *room_num += 1;
            } else {
                return false;
            }

            if let Some(false) = is_playing.get(&room) {
                let player_num = self.player_num_pool.get_mut(&room)
                    .expect("Room not found")
                    .pop_front()
                    .unwrap();

                keystates.insert(id, KeyState::new());
                let ships_in_room = ships.get_mut(&room).unwrap();
                ships_in_room.insert(id, Ship::new(
                    player_num,
                    COLOR_LIST[player_num as usize].to_string(),
                    vector::Vector { x: 100.0 * player_num as f32 + 150.0, y: 300.0 },
                    -PI / 2.0,
                ));
            }
        }
        sessions.insert(id, addr);
        let listeners_in_room = listeners.get_mut(&room).unwrap();
        listeners_in_room.insert(id);

        true
    }
}

impl Handler<KeyUpdate> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: KeyUpdate, _ctx: &mut Self::Context) -> Self::Result {
        let KeyUpdate { id, state } = msg;

        self.get_game_data().keystates.lock().unwrap()
            .insert(id, state);
    }
}

impl Handler<GetPlayerCount> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: GetPlayerCount, _ctx: &mut Self::Context) -> Self::Result {
        *self.room_num.get(&msg.room_id).unwrap_or(&0)
    }
}

impl Handler<IsPlaying> for GameServer {
    type Result = bool;

    fn handle(&mut self, msg: IsPlaying, _ctx: &mut Self::Context) -> Self::Result {
        *self.get_game_data().is_playing.lock().unwrap()
            .get(&msg.room_id)
            .unwrap_or(&false)
    }
}
