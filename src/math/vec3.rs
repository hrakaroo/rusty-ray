
#[derive(Copy, Clone)]
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

    pub fn add_vec3(&self, other: &Vec3) -> Vec3 {
        Vec3 { e0: self.e0 + other.e0, e1: self.e1 + other.e1, e2: self.e2 + other.e2 }
    }

    pub fn subtract_vec3(&self, other: &Vec3) -> Vec3 {
        Vec3 { e0: self.e0 - other.e0, e1: self.e1 - other.e1, e2: self.e2 - other.e2 }
    }

    pub fn multiply_vec3(&self, other: &Vec3) -> Vec3 {
        Vec3 { e0: self.e0 * other.e0, e1: self.e1 * other.e1, e2: self.e2 * other.e2 }
    }

    pub fn multiply_scalar(&self, scalar: f32) -> Vec3 {
        Vec3 { e0: self.e0 * scalar, e1: self.e1 * scalar, e2: self.e2 * scalar }
    }

    pub fn length(&self) -> f32 {
        (self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2).sqrt() as f32
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        Vec3 { e0: self.e0 / len, e1: self.e1 / len, e2: self.e2 / len }
    }
}
