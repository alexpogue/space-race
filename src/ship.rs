pub struct Ship {
    x: i32,
    y: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Ship {

    pub fn new(x: i32, y: i32) -> Ship {
        Ship {
            x: x,
            y: y,
            x_vel: 0,
            y_vel: 0,
        }
    }

    pub fn accelerate_x(&mut self, x: i32) {
        self.x_vel += x;
    }

    pub fn accelerate_y(&mut self, y: i32) {
        self.y_vel += y;
    }

    pub fn x(&self) -> i32{
        self.x
    }

    pub fn y(&self) -> i32{
        self.y
    }

    pub fn update(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }
}
