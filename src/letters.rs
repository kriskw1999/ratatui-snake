use ratatui::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

pub struct Letter {
    pub lines: Vec<Vec<f64>>,
    pub starting_x: f64,
    pub scale: u8,
    pub color: Color,
}

impl Shape for Letter {
    fn draw(&self, painter: &mut Painter) {
        for line_points in &self.lines {
            let line = Line::new(
                line_points[0] * (self.scale as f64) + self.starting_x,
                line_points[1] * (self.scale as f64),
                line_points[2] * (self.scale as f64) + self.starting_x,
                line_points[3] * (self.scale as f64),
                self.color,
            );
            line.draw(painter);
        }
    }
}

impl Letter {
    pub fn new_letter(letter: char, starting_x: f64) -> Self {
        let lines = match letter {
            // A
            'a' => vec![
                vec![1.0, 0.0, 2.5, 5.0],
                vec![2.5, 5.0, 4.0, 0.0],
                vec![1.25, 2.5, 3.75, 2.5],
            ],
            // B
            'b' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 5.0, 3.0, 5.0],
                vec![3.0, 5.0, 4.0, 4.0],
                vec![4.0, 4.0, 4.0, 3.0],
                vec![4.0, 3.0, 3.0, 2.5],
                vec![3.0, 2.5, 4.0, 2.0],
                vec![4.0, 2.0, 4.0, 1.0],
                vec![4.0, 1.0, 3.0, 0.0],
                vec![3.0, 0.0, 0.0, 0.0],
            ],
            // C
            'c' => vec![
                vec![4.0, 5.0, 1.0, 5.0],
                vec![1.0, 5.0, 0.0, 4.0],
                vec![0.0, 4.0, 0.0, 1.0],
                vec![0.0, 1.0, 1.0, 0.0],
                vec![1.0, 0.0, 4.0, 0.0],
            ],
            // D
            'd' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 5.0, 3.0, 5.0],
                vec![3.0, 5.0, 4.0, 4.0],
                vec![4.0, 4.0, 4.0, 1.0],
                vec![4.0, 1.0, 3.0, 0.0],
                vec![3.0, 0.0, 0.0, 0.0],
            ],
            // E
            'e' => vec![
                vec![4.0, 5.0, 0.0, 5.0],
                vec![0.0, 5.0, 0.0, 0.0],
                vec![0.0, 0.0, 4.0, 0.0],
                vec![0.0, 2.5, 3.0, 2.5],
            ],
            // F
            'f' => vec![
                vec![0.0, 5.0, 0.0, 0.0],
                vec![0.0, 5.0, 4.0, 5.0],
                vec![0.0, 2.5, 3.0, 2.5],
            ],
            // G
            'g' => vec![
                vec![4.0, 5.0, 1.0, 5.0],
                vec![1.0, 5.0, 0.0, 4.0],
                vec![0.0, 4.0, 0.0, 1.0],
                vec![0.0, 1.0, 1.0, 0.0],
                vec![1.0, 0.0, 4.0, 0.0],
                vec![4.0, 0.0, 4.0, 2.5],
                vec![4.0, 2.5, 2.5, 2.5],
            ],
            // H
            'h' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 2.5, 4.0, 2.5],
                vec![4.0, 0.0, 4.0, 5.0],
            ],
            // I
            'i' => vec![
                vec![1.0, 5.0, 4.0, 5.0],
                vec![2.5, 5.0, 2.5, 0.0],
                vec![1.0, 0.0, 4.0, 0.0],
            ],
            // J
            'j' => vec![
                vec![1.0, 5.0, 4.0, 5.0],
                vec![2.5, 5.0, 2.5, 1.0],
                vec![2.5, 1.0, 1.5, 0.0],
                vec![1.5, 0.0, 0.0, 0.0],
            ],
            // K
            'k' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 2.5, 4.0, 5.0],
                vec![0.0, 2.5, 4.0, 0.0],
            ],
            // L
            'l' => vec![vec![0.0, 5.0, 0.0, 0.0], vec![0.0, 0.0, 4.0, 0.0]],
            // M
            'm' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 5.0, 2.5, 2.5],
                vec![2.5, 2.5, 5.0, 5.0],
                vec![5.0, 5.0, 5.0, 0.0],
            ],
            // N
            'n' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 5.0, 5.0, 0.0],
                vec![5.0, 0.0, 5.0, 5.0],
            ],
            // O
            'o' => vec![
                vec![1.0, 5.0, 4.0, 5.0],
                vec![4.0, 5.0, 5.0, 4.0],
                vec![5.0, 4.0, 5.0, 1.0],
                vec![5.0, 1.0, 4.0, 0.0],
                vec![4.0, 0.0, 1.0, 0.0],
                vec![1.0, 0.0, 0.0, 1.0],
                vec![0.0, 1.0, 0.0, 4.0],
                vec![0.0, 4.0, 1.0, 5.0],
            ],
            // P
            'p' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 5.0, 4.0, 5.0],
                vec![4.0, 5.0, 4.0, 2.5],
                vec![4.0, 2.5, 0.0, 2.5],
            ],
            // Q
            'q' => vec![
                vec![1.0, 5.0, 4.0, 5.0],
                vec![4.0, 5.0, 5.0, 4.0],
                vec![5.0, 4.0, 5.0, 1.0],
                vec![5.0, 1.0, 4.0, 0.0],
                vec![4.0, 0.0, 1.0, 0.0],
                vec![1.0, 0.0, 0.0, 1.0],
                vec![0.0, 1.0, 0.0, 4.0],
                vec![0.0, 4.0, 1.0, 5.0],
                vec![3.0, 2.0, 5.0, 0.0],
            ],
            // R
            'r' => vec![
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0, 5.0, 4.0, 5.0],
                vec![4.0, 5.0, 4.0, 2.5],
                vec![4.0, 2.5, 0.0, 2.5],
                vec![0.0, 2.5, 4.0, 0.0],
            ],
            // S
            's' => vec![
                vec![4.0, 5.0, 1.0, 5.0],
                vec![1.0, 5.0, 0.0, 4.0],
                vec![0.0, 4.0, 4.0, 1.0],
                vec![4.0, 1.0, 4.0, 0.0],
                vec![4.0, 0.0, 1.0, 0.0],
                vec![1.0, 0.0, 0.0, 1.0],
            ],
            // T
            't' => vec![vec![0.0, 5.0, 5.0, 5.0], vec![2.5, 5.0, 2.5, 0.0]],
            // U
            'u' => vec![
                vec![0.0, 5.0, 0.0, 1.0],
                vec![0.0, 1.0, 1.0, 0.0],
                vec![1.0, 0.0, 4.0, 0.0],
                vec![4.0, 0.0, 5.0, 1.0],
                vec![5.0, 1.0, 5.0, 5.0],
            ],
            // V
            'v' => vec![vec![0.0, 5.0, 2.5, 0.0], vec![2.5, 0.0, 5.0, 5.0]],
            // W
            'w' => vec![
                vec![0.0, 5.0, 1.5, 0.0],
                vec![1.5, 0.0, 2.5, 2.5],
                vec![2.5, 2.5, 3.5, 0.0],
                vec![3.5, 0.0, 5.0, 5.0],
            ],
            // X
            'x' => vec![vec![0.0, 0.0, 5.0, 5.0], vec![5.0, 0.0, 0.0, 5.0]],
            // Y
            'y' => vec![
                vec![0.0, 5.0, 2.5, 2.5],
                vec![5.0, 5.0, 2.5, 2.5],
                vec![2.5, 2.5, 2.5, 0.0],
            ],
            // Z
            'z' => vec![
                vec![0.0, 5.0, 5.0, 5.0],
                vec![5.0, 5.0, 0.0, 0.0],
                vec![0.0, 0.0, 5.0, 0.0],
            ],
            // Space
            ' ' => vec![],

            _ => vec![],
        };

        Letter {
            lines,
            starting_x,
            scale: 1,
            color: Color::White,
        }
    }
}

pub struct Word {
    pub word: String,
    pub starting_x: f64,
}

impl Word {
    pub fn new(word: String, starting_x: f64) -> Self {
        Word { word, starting_x }
    }
}

impl Shape for Word {
    fn draw(&self, painter: &mut Painter) {
        for (i, letter) in self.word.chars().enumerate() {
            let letter = Letter::new_letter(letter, self.starting_x + (i as f64) * 7.0);

            letter.draw(painter);
        }
    }
}
