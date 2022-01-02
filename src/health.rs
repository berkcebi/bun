pub struct Health {
    pub points: u8,
    pub max_points: u8,
}

impl Health {
    pub fn new(points: u8) -> Self {
        Self {
            points,
            max_points: points,
        }
    }
}
