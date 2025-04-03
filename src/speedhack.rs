pub struct Speedhack {
    speed_multiplier: f32,
}

impl Speedhack {
    pub fn new() -> Self {
        Speedhack { speed_multiplier: 1.0 }
    }

    pub fn set_speed(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier;
    }

    pub fn apply(&self) {
        // Logic to apply speed hack
    }
}