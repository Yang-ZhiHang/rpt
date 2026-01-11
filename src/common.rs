use rand::Rng;

pub fn random() -> f32 {
    rand::rng().random()
}

pub fn random_in_range(min: u32, max: u32) -> u32 {
    min + (rand::rng().random::<f32>() * (max - min) as f32) as u32
}
