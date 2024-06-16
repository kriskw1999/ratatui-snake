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
    pub state: GameState,
    pub frame_num: i32,
    speed: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            state: GameState::Startup,
            frame_num: 0,
            speed: SPEED,
        }
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
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

    pub fn game_over(&mut self) {
        self.state = GameState::GameOver;
    }

    pub fn restart(&mut self) {
        self.score = 0;
        self.state = GameState::Running;
        self.frame_num = 0;
    }
}
