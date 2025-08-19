const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug)]
pub struct HolyParticle {
    pub position: crate::HolyVector2,
    pub dx: i32,
    pub dy: i32,
    speed: i32,
}

impl HolyParticle {
    pub fn new(width: u32, height: u32) -> Self {
        let (dx, dy) = DIRS[rand::random_range(0..DIRS.len())];

        HolyParticle {
            position: crate::HolyVector2::new(
                rand::random_range(0..width) as i32,
                rand::random_range(0..height) as i32,
            ),
            dx,
            dy,
            speed: rand::random_range(1..=5),
        }
    }
}

impl HolyParticle {
    pub fn update(&mut self) {
        self.position.x += self.dx * self.speed;
        self.position.y += self.dy * self.speed;
    }

    pub fn is_dead(&self, width: u32, height: u32) -> bool {
        self.position.x < -(width as i32)
            || self.position.y < -(height as i32)
            || self.position.x > width as i32
            || self.position.y > height as i32
    }
}
