pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub direction: Direction
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}