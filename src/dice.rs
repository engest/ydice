use rand::distributions::{Distribution, Uniform};

pub struct Die {
    val: u8,
}

impl Die {
    pub fn new() -> Self {
        Self {
            val: 1,
        }
    }

    pub fn get_val(&self) -> u8 {
        self.val
    }
    
    pub fn throw(&mut self) {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..7);
        self.val = die.sample(&mut rng);
    }
}