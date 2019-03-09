pub enum Color {
    BLACK,
    GRAY,
    GREENGRASS,
    LIGHTGRAY,
    RED
}

impl Color {
    pub fn get(&self) -> [f32; 4] {
        match self {
            Color::BLACK => [0.0, 0.0, 0.0, 1.0],
            Color::GRAY => [0.5, 0.5, 0.5, 1.0],
            Color::GREENGRASS => [0.8, 0.93, 0.42, 1.0],
            Color::LIGHTGRAY => [0.85, 0.85, 0.85, 1.0],
            Color::RED => [1.0, 0.0, 0.0, 1.0]
        }
    }
}