pub struct Rect {
    pub length: u32,
    pub breath: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.length * self.breath
    }

    pub fn perimeter(&self) -> u32 {
        2 * (self.length + self.breath)
    }
}
