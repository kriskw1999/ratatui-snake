use collision_detection::check_collisions;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use game::{Direction, Game, GameState};

use letters::Word;
use point::Point;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    symbols::Marker,
    widgets::canvas::Canvas,
};
use snake::Snake;
use std::io::{stdout, Result};
use walls::Walls;

mod game;
mod letters;
mod point;
mod snake;
mod walls;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // the size of the board
    let size = terminal.size()?;
    let height = size.height as f64;
    let width = size.width as f64;

    let mut game = Game::new();
    let mut snake = Snake::new();
    let mut point = Point::new(width, height);
    let walls = Walls::new(width, height);

    loop {
        game.increase_frame_num();

        let _ = terminal.draw(|frame| {
            let area = frame.size();

            if check_collisions(&snake.head, &snake.body) || check_collisions(&snake.head, &walls) {
                game.game_over();
            }

            if check_collisions(&snake.head, &point) {
                snake.grow();
                game.increase_score();
                point.create_new_point();
            }

            // movement
            if game.state == GameState::Running && game.frame_num == 0 {
                snake.move_snake();
            }

            frame.render_widget(
                Canvas::default()
                    .x_bounds([-width / 2.0, width / 2.0])
                    .y_bounds([-height, height])
                    .marker(Marker::HalfBlock)
                    .paint(|ctx| {
                        ctx.draw(&walls);

                        ctx.layer();

                        if game.state != GameState::Startup {
                            ctx.print(
                                -width / 2.0 + 3.0,
                                height - 4.0,
                                format!("Score: {}", game.score),
                            );
                        }

                        ctx.layer();

                        match game.state {
                            GameState::Running | GameState::Paused => {
                                ctx.draw(&snake);
                                ctx.draw(&point);
                            }
                            GameState::GameOver => {
                                ctx.draw(&Word::new("gameover".to_string(), -27.0));
                                ctx.print(-9.0, -5.0, "Press R to restart");
                            }
                            GameState::Startup => {
                                ctx.draw(&Word::new("ratatui snake".to_string(), -48.0));
                                ctx.print(-15.0, -5.0, "Press any character to start");
                            }
                        }
                    }),
                area,
            )
        });

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if game.state == GameState::Startup {
                        game.state = GameState::Running;
                    } else {
                        match key.code {
                            KeyCode::Char('q') => {
                                break;
                            }
                            KeyCode::Char('p') => {
                                if game.state == GameState::Paused {
                                    game.state = GameState::Running;
                                } else if GameState::Running == game.state {
                                    game.state = GameState::Paused;
                                }
                            }
                            KeyCode::Char('a') | KeyCode::Char('h') => {
                                if snake.head.direction != Direction::Right {
                                    snake.change_direction(Direction::Left);
                                }
                            }
                            KeyCode::Char('d') | KeyCode::Char('l') => {
                                if snake.head.direction != Direction::Left {
                                    snake.change_direction(Direction::Right);
                                }
                            }
                            KeyCode::Char('w') | KeyCode::Char('k') => {
                                if snake.head.direction != Direction::Down {
                                    snake.change_direction(Direction::Up);
                                }
                            }
                            KeyCode::Char('s') | KeyCode::Char('j') => {
                                if snake.head.direction != Direction::Up {
                                    snake.change_direction(Direction::Down);
                                }
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                if game.state == GameState::GameOver {
                                    game.restart();
                                    snake = snake::Snake::new();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
