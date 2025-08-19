#[derive(Debug, Clone, Copy)]
pub struct HolyVector2 {
    pub x: i32,
    pub y: i32,
}

impl HolyVector2 {
    pub const fn new(x: i32, y: i32) -> Self {
        HolyVector2 { x, y }
    }
}
