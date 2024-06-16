use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

use crate::{collision::Collidable, coord::Coord};

pub struct Point {
    pub coord: Coord,
    pub max_x: f64,
    pub max_y: f64,
}

impl Point {
    pub fn new(max_x: f64, max_y: f64) -> Self {
        let coord = Point::get_random_coord(max_x, max_y);

        Point {
            coord,
            max_x,
            max_y,
        }
    }

    pub fn get_random_coord(max_x: f64, max_y: f64) -> Coord {
        Coord::randomize(max_x - 1.0, max_y - 1.0)
    }

    pub fn create_new_point(&mut self) {
        self.coord = Point::get_random_coord(self.max_x, self.max_y);
    }
}

impl Collidable for Point {
    fn get_border(&self) -> Vec<Coord> {
        return vec![self.coord.clone()];
    }
}

impl Shape for Point {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let line = Line {
            x1: self.coord.x,
            y1: self.coord.y,
            x2: self.coord.x,
            y2: self.coord.y,
            color: Color::Red,
        };

        line.draw(painter);
    }
}
