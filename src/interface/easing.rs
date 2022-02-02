pub enum Easing {
    OutQuart,
}

pub fn ease(value: f32, easing: Easing) -> f32 {
    match easing {
        Easing::OutQuart => 1.0 - (1.0 - value).powf(4.0),
    }
}
