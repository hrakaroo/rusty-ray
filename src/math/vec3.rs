
pub struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e0, e1, e2 }
    }

    pub fn x(&self) -> f32 {
        self.e0
    }
    pub fn y(&self) -> f32 {
        self.e1
    }
    pub fn z(&self) -> f32 {
        self.e2
    }
    pub fn r(&self) -> f32 {
        self.e0
    }
    pub fn g(&self) -> f32 {
        self.e1
    }
    pub fn b(&self) -> f32 {
        self.e2
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        [(self.e0 * 255.99) as u8, (self.e1 * 255.99) as u8, (self.e2 * 255.99) as u8]
    }

    pub fn neg(&self) -> Vec3 {
        Vec3 {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
        }
    }

    pub fn length(&self) -> f32 {
        (self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2).sqrt() as f32
    }
}
