#[derive(Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Self {
        Coord {
            x,
            y,
        }
    }

    pub fn move_left(&mut self, speed: f64) {
        self.x -= speed;
    }

    pub fn move_right(&mut self, speed: f64) {
        self.x += speed;
    }

    pub fn move_up(&mut self, speed: f64) {
        self.y += speed;
    }

    pub fn move_down(&mut self, speed: f64) {
        self.y -= speed;
    }

    pub fn get_x_distance(c1: &Coord, c2: &Coord) -> f64 {
        (c1.x - c2.x).abs()
    }

    pub fn get_y_distance(c1: &Coord, c2: &Coord) -> f64 {
        (c1.y - c2.y).abs()
    }

    pub fn randomize(x_max: f64, y_max: f64) -> Self {
        let x = (rand::random::<f64>() * x_max - x_max / 2.0).round();
        let y = (rand::random::<f64>() * y_max - y_max / 2.0).round();

        Coord::new(x, y)
    }

    pub fn move_toward(&mut self, c: &Coord) {
        let x_diff = if self.x < c.x { 1.0 } else if self.x > c.x { -1.0 } else { 0.0 };
        let y_diff = if self.y < c.y { 1.0 } else if self.y > c.y { -1.0 } else { 0.0 };

        self.x += x_diff;
        self.y += y_diff;
    }

    pub fn compare(&self, c: &Coord) -> bool {
        self.x == c.x && self.y == c.y
    }
}
