use collision_detection::{coord::Coord, Collidable};
use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

pub struct Walls {
    pub corners: [Coord; 4],
}

impl Walls {
    pub fn new(width: f64, height: f64) -> Self {
        Walls {
            corners: [
                Coord::new(width / 2.0, height),
                Coord::new(width / 2.0, -height),
                Coord::new(-width / 2.0, -height),
                Coord::new(-width / 2.0, height),
            ],
        }
    }
}

impl Shape for Walls {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for i in 0..self.corners.len() {
            let next: usize = (i + 1) % self.corners.len();

            let line = Line {
                x1: self.corners[i].x,
                y1: self.corners[i].y,
                x2: self.corners[next].x,
                y2: self.corners[next].y,
                color: Color::White,
            };

            line.draw(painter);
        }
    }
}

impl Collidable for Walls {
    fn get_border(&self) -> Vec<Coord> {
        let mut borders = self.corners.to_vec();
        borders.push(self.corners[0].clone());

        borders
            .iter()
            .map(|coord| Coord {
                x: coord.x.round(),
                y: coord.y.round(),
            })
            .collect()
    }
}
