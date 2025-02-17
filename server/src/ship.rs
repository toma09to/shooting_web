use std::{f32::consts::PI, time::{Instant, Duration}};

use vector::{Vector, WIDTH, HEIGHT};
use serde::Serialize;

use crate::{bullet::Bullet, keystate::KeyState, server::Text};

pub const ROTATE_SPEED: f32 = 0.07;
pub const ACCEL_FACTOR: f32 = 0.03;
pub const DECEL_FACTOR: f32 = 0.005;
pub const MAX_LIVES: u32 = 3;
pub const HIT_AREA_RADIUS: f32 = 12.0;
pub const CHARGE_TIME: Duration = Duration::from_millis(500);
pub const RESPAWN_TIME: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    #[serde(skip)]
    pub player_num: u8,
    color: String,
    pos: Vector,
    rad: f32,
    #[serde(skip)]
    speed: Vector,
    #[serde(skip)]
    lives: u32,
    is_alive: bool,
    is_accelerating: bool,
    pub is_ready: bool,
    #[serde(skip)]
    last_fire_time: Instant,
    #[serde(skip)]
    last_hit_time: Instant,
}

impl Ship {
    pub fn new(player_num: u8, color: String, pos: Vector, rad: f32) -> Self {
        Self {
            player_num,
            color,
            pos,
            rad,
            speed: Vector::new(0.0, 0.0),
            lives: MAX_LIVES,
            is_alive: true,
            is_accelerating: false,
            is_ready: false,
            last_fire_time: Instant::now(),
            last_hit_time: Instant::now(),
        }
    }

    pub fn process_one_frame(&mut self, key_state: &KeyState) {
        self.is_accelerating = key_state.up;
        if self.is_alive {
            self.move_by_one_frame(key_state);
        } else if Instant::now().duration_since(self.last_hit_time) > RESPAWN_TIME {
            self.lives -= 1;
            self.is_alive = true;
            self.put_on_random_place();
        }
    }

    fn move_by_one_frame(&mut self, key_state: &KeyState) {
        if key_state.up {
            self.speed += Vector::new(ACCEL_FACTOR, 0.0).rotate(self.rad);
        }

        self.speed *= 1.0 - DECEL_FACTOR;

        self.pos += self.speed;

        if key_state.right {
            self.rad += ROTATE_SPEED;
        }
        if key_state.left {
            self.rad -= ROTATE_SPEED;
        }
    }

    pub fn put_on_random_place(&mut self) {
        self.pos.x = rand::random_range(0.0 .. WIDTH as f32);
        self.pos.y = rand::random_range(0.0 .. HEIGHT as f32);
        self.rad   = rand::random_range(0.0 .. PI * 2.0);
        self.speed = Vector::new(0.0, 0.0);
    }

    pub fn fire(&mut self) -> Option<Bullet> {
        let now = Instant::now();

        if self.is_alive && now.duration_since(self.last_fire_time) > CHARGE_TIME {
            self.last_fire_time = now;

            let head = self.pos + Vector::new(15.0, 0.0).rotate(self.rad);
            Some(Bullet::new(self.color.clone(), head, self.rad))
        } else {
            None
        }
    }

    pub fn collision_process(&mut self, bullet: &Bullet) {
        if Vector::dist2(self.pos, bullet.pos) < HIT_AREA_RADIUS * HIT_AREA_RADIUS {
            if self.is_alive {
                self.is_alive = false;
            }

            // Continues an invicible time even if a player is dead.
            self.last_hit_time = Instant::now();
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.lives == 0
    }

    pub fn ready_text(&self) -> Text {
        Text::new(
            self.color.clone(), 
            Vector::new(self.pos.x, self.pos.y + 40.0),
            "READY".to_string()
        )
    }
}
