use crate::coord::Coord;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    pub score: i32,
    pub total_length: i32,
    pub state: GameState,
    pub corners: Vec<Coord>,
    pub head_coord: Coord,
    pub direction: Direction,
    pub point_coord: Option<Coord>,
    pub frame_num: i32,
    speed: i32,
}

fn get_direction(position: f64, target: f64) -> f64 {
    if position < target { 1.0 } else if position > target { -1.0 } else { 0.0 }
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            state: GameState::Running,
            corners: Vec::from([Coord::new(-12.0, 10.0)]),
            total_length: 12,
            head_coord: Coord::new(0.0, 10.0),
            direction: Direction::Right,
            point_coord: None,
            frame_num: 0,
            speed: 8,
        }
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
        self.total_length += 2;
    }

    pub fn increase_frame_num(&mut self) {
        self.frame_num += {
            if self.frame_num > 10 / self.speed { -self.frame_num } else { 1 }
        };
    }

    pub fn move_head(&mut self) {
        match self.direction {
            Direction::Up => {
                self.head_coord.y += 1.0;
            }
            Direction::Down => {
                self.head_coord.y -= 1.0;
            }
            Direction::Left => {
                self.head_coord.x -= 1.0;
            }
            Direction::Right => {
                self.head_coord.x += 1.0;
            }
        }
    }

    pub fn game_over(&mut self) {
        self.state = GameState::GameOver;
    }

    pub fn move_tail(&mut self) {
        let tail_coord = &self.corners[0];

        if self.corners.len() == 1 {
            let x_diff = get_direction(tail_coord.x, self.head_coord.x);
            let y_diff = get_direction(tail_coord.y, self.head_coord.y);

            if
                (tail_coord.x - self.head_coord.x).abs() > (self.total_length as f64) ||
                (tail_coord.y - self.head_coord.y).abs() > (self.total_length as f64)
            {
                self.corners[0] = Coord::new(tail_coord.x + x_diff, tail_coord.y + y_diff);
            }
        } else {
            let last_corner_coord = &self.corners[1];

            let x_diff = get_direction(tail_coord.x, last_corner_coord.x);
            let y_diff = get_direction(tail_coord.y, last_corner_coord.y);

            if
                tail_coord.x + x_diff != last_corner_coord.x ||
                tail_coord.y + y_diff != last_corner_coord.y
            {
                self.corners[0] = Coord::new(tail_coord.x + x_diff, tail_coord.y + y_diff);
            } else {
                self.corners.remove(0);
            }
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.corners.push(Coord::new(self.head_coord.x, self.head_coord.y))
    }

    pub fn restart(&mut self) {
        self.score = 0;
        self.total_length = 12;
        self.state = GameState::Running;
        self.corners = Vec::from([Coord::new(-12.0, 10.0)]);
        self.head_coord = Coord::new(0.0, 10.0);
        self.direction = Direction::Right;
        self.point_coord = None;
        self.frame_num = 0;
    }

    pub fn generate_new_point(&mut self, width: f64, height: f64) {
        self.point_coord = Some(
            Coord::new(
                (rand::random::<f64>() * width - 1.0 - width / 2.0).round(),
                (rand::random::<f64>() * height - 1.0 - height / 2.0).round()
            )
        );
    }

    pub fn check_beat(&mut self) {
        self.corners.windows(2).for_each(|pair| {
            let start = &pair[0];
            let end = &pair[1];

            // check if the head coord is between start and end
            if
                (self.head_coord.x - start.x) * (self.head_coord.x - end.x) <= 0.0 &&
                (self.head_coord.y - start.y) * (self.head_coord.y - end.y) <= 0.0
            {
                self.state = GameState::GameOver;
            }
        });
    }
}
