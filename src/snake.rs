use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

use crate::{collision::Collidable, coord::Coord, game::Direction};

pub struct SnakeHead {
    pub coord: Coord,
    pub direction: Direction,
}

impl SnakeHead {
    pub fn new() -> Self {
        Self {
            coord: Coord::new(12.0, 0.0),
            direction: Direction::Right,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_head(&mut self) {
        match self.direction {
            Direction::Up => self.coord.move_up(1.0),
            Direction::Down => self.coord.move_down(1.0),
            Direction::Left => self.coord.move_left(1.0),
            Direction::Right => self.coord.move_right(1.0),
        }
    }
}

impl Collidable for SnakeHead {
    fn get_border(&self) -> Vec<Coord> {
        vec![self.coord.clone()]
    }
}

pub struct SnakeBody {
    corners: Vec<Coord>,
}

impl SnakeBody {
    fn new() -> Self {
        SnakeBody {
            corners: vec![Coord { x: 0.0, y: 0.0 }],
        }
    }

    fn move_tail(&mut self, head_coord: &Coord, total_length: f64) {
        let tail_coord = &self.corners[0];

        if self.corners.len() == 1 {
            let tail_distance_x = Coord::get_x_distance(tail_coord, head_coord);
            let tail_distance_y = Coord::get_y_distance(tail_coord, head_coord);

            if tail_distance_x + tail_distance_y > total_length {
                self.corners[0].move_toward(head_coord);
            }
        } else {
            let next_corner = self.corners[1].clone();
            self.corners[0].move_toward(&next_corner);

            if self.corners[0].compare(&next_corner) {
                self.corners.remove(0);
            }
        }
    }
}

impl Collidable for SnakeBody {
    fn get_border(&self) -> Vec<Coord> {
        self.corners.clone()
    }
}

pub struct Snake {
    pub total_length: i32,
    pub body: SnakeBody,
    pub head: SnakeHead,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            total_length: 12,
            body: SnakeBody::new(),
            head: SnakeHead::new(),
        }
    }

    pub fn grow(&mut self) {
        self.total_length += 1;
    }

    pub fn move_snake(&mut self) {
        self.head.move_head();
        self.body
            .move_tail(&self.head.coord, self.total_length as f64);
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.head.change_direction(direction);
        self.body
            .corners
            .push(Coord::new(self.head.coord.x, self.head.coord.y));
        self.move_snake();
    }
}

impl Shape for Snake {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        self.body.corners.windows(2).for_each(|arr| {
            let start_coord = &arr[0];
            let end_coord = &arr[1];

            let body_line = Line {
                x1: start_coord.x,
                y1: start_coord.y,
                x2: end_coord.x,
                y2: end_coord.y,
                color: Color::Blue,
            };

            body_line.draw(painter);
        });

        let head_line = Line {
            x1: self.head.coord.x,
            y1: self.head.coord.y,
            x2: self.body.corners.last().unwrap().x,
            y2: self.body.corners.last().unwrap().y,
            color: Color::Blue,
        };

        head_line.draw(painter);
    }
}
