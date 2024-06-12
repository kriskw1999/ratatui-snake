use crate::coord::Coord;

const SPEED: u8 = 8;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum GameState {
    Startup,
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
    speed: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            state: GameState::Startup,
            corners: Vec::from([Coord::new(-12.0, 10.0)]),
            total_length: 12,
            head_coord: Coord::new(0.0, 10.0),
            direction: Direction::Right,
            point_coord: None,
            frame_num: 0,
            speed: SPEED,
        }
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
        self.total_length += 2;
    }

    pub fn increase_frame_num(&mut self) {
        self.frame_num += {
            if self.frame_num > 10 / (self.speed as i32) {
                -self.frame_num
            } else {
                1
            }
        };
    }

    pub fn move_head(&mut self) {
        match self.direction {
            Direction::Up => self.head_coord.move_up(1.0),
            Direction::Down => self.head_coord.move_down(1.0),
            Direction::Left => self.head_coord.move_left(1.0),
            Direction::Right => self.head_coord.move_right(1.0),
        }
    }

    pub fn game_over(&mut self) {
        self.state = GameState::GameOver;
    }

    pub fn move_tail(&mut self) {
        let tail_coord = &self.corners[0];

        if self.corners.len() == 1 {
            let tail_distance_x = Coord::get_x_distance(tail_coord, &self.head_coord);
            let tail_distance_y = Coord::get_y_distance(tail_coord, &self.head_coord);

            if tail_distance_x + tail_distance_y > (self.total_length as f64) {
                self.corners[0].move_toward(&self.head_coord);
            }
        } else {
            let next_corner = self.corners[1].clone();
            self.corners[0].move_toward(&next_corner);

            if self.corners[0].compare(&next_corner) {
                self.corners.remove(0);
            }
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.corners
            .push(Coord::new(self.head_coord.x, self.head_coord.y))
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
        self.point_coord = Some(Coord::randomize(width - 1.0, height - 1.0));
    }

    pub fn check_beat(&mut self) {
        self.corners.windows(2).for_each(|pair| {
            let start = &pair[0];
            let end = &pair[1];

            // check if the head coord is between start and end
            if (self.head_coord.x - start.x) * (self.head_coord.x - end.x) <= 0.0
                && (self.head_coord.y - start.y) * (self.head_coord.y - end.y) <= 0.0
            {
                self.state = GameState::GameOver;
            }
        });
    }
}
