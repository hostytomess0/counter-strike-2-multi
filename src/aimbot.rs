pub struct Aimbot {
    target: Option<Target>,
}

impl Aimbot {
    pub fn new() -> Self {
        Aimbot { target: None }
    }

    pub fn find_target(&mut self) {
        // Logic to find target
    }

    pub fn aim(&self) {
        if let Some(target) = &self.target {
            // Logic to aim at target
        }
    }
}