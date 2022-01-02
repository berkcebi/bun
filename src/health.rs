pub struct Health {
    pub points: u16,
    pub max_points: u16,
}

impl Health {
    pub fn new(points: u16) -> Self {
        Self {
            points,
            max_points: points,
        }
    }
}
