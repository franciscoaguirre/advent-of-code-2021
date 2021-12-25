#[derive(Clone)]
pub struct LanternFish(pub u32);

impl LanternFish {
    pub fn new(internal_timer: u32) -> LanternFish {
        LanternFish(internal_timer)
    }

    pub fn from_string(string: &str) -> LanternFish {
        LanternFish(string.parse().unwrap())
    }

    pub fn tick(&mut self) -> Option<LanternFish> {
        if self.0 == 0 {
            self.0 = 6;
            return Some(LanternFish::new(8));
        }

        self.0 -= 1;

        None
    }
}
