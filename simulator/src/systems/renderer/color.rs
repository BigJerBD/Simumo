pub enum Color {
    LIGHTGRAY,
    RED
}

impl Color {
    pub fn get(&self) -> [f32; 4] {
        match self {
            Color::LIGHTGRAY => [0.85, 0.85, 0.85, 1.0],
            Color::RED => [1.0, 0.0, 0.0, 1.0]
        }
    }
}