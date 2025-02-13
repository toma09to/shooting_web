use vector::Vector;

const SPEED_SIZE: f32 = 7.0;

pub struct Bullet {
    color: String,
    pos: Vector,
    speed: Vector,
}

impl Bullet {
    pub fn new(color: String, pos: Vector, rad: f32) -> Self {
        Self {
            color,
            pos,
            speed: Vector::new(SPEED_SIZE, 0.0).rotate(rad),
        }
    }

    pub fn move_by_one_frame(&mut self) {
        self.pos += self.speed;
    }

    pub fn is_alive(&self) -> bool {
        // Considers a bullet dead if pos will overflow after moving

        let epsilon = 1.0;
        let after_moved = self.pos + self.speed;

        Vector::dist2(self.pos, after_moved) <= SPEED_SIZE * SPEED_SIZE + epsilon
    }
}
