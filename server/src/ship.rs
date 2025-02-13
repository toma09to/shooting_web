use std::f32::consts::PI;

use vector::{Vector, WIDTH, HEIGHT};

use crate::{bullet::Bullet, keystate::KeyState};

pub const ROTATE_SPEED: f32 = 0.07;
pub const ACCEL_FACTOR: f32 = 0.03;
pub const DECEL_FACTOR: f32 = 0.005;
pub const MAX_LIVES: u32 = 3;

pub struct Ship {
    color: String,
    pos: Vector,
    rad: f32,
    speed: Vector,
    lives: u32,
    is_alive: bool,
    is_accelerating: bool,
    is_ready: bool,
    // last_fire_time: <some duration type>,
    // last_hit_time: <some duration type>,
}

impl Ship {
    pub fn new(color: String, pos: Vector, rad: f32) -> Self {
        Self {
            color,
            pos,
            rad,
            speed: Vector::new(0.0, 0.0),
            lives: MAX_LIVES,
            is_alive: true,
            is_accelerating: false,
            is_ready: false,
        }
    }

    pub fn process_one_frame(&mut self, key_state: &KeyState) {
        self.is_accelerating = key_state.up;
        self.move_by_one_frame(key_state);
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

    pub fn fire(&self) -> Option<Bullet> {
        // if last_fire_time < ... {
        //     return None;
        // }

        let head = self.pos + Vector::new(15.0, 0.0).rotate(self.rad);
        Some(Bullet::new(self.color.clone(), head, self.rad))
    }
}
