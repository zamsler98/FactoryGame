[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Entity {
    pub position: Position,
    pub id: i32,
}


