use coord::Coord;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use game::{Direction, Game, GameState};

use letters::Word;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, Line, Rectangle},
};
use std::io::{stdout, Result};

mod coord;
mod game;
mod letters;

fn get_collision(p1: &Coord, p2: &Coord) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut game = Game::new();

    loop {
        game.increase_frame_num();

        let _ = terminal.draw(|frame| {
            let area = frame.size();

            // the size of the board
            let height = area.height as f64;
            let width = area.width as f64;

            // movement
            if game.state == GameState::Running && game.frame_num == 0 {
                game.move_tail();
                game.move_head();

                game.check_beat();
            }

            // check if reached the borders
            if game.head_coord.x.abs() + 1.0 >= width / 2.0
                || game.head_coord.y.abs() + 1.0 == height
            {
                game.game_over();
            }

            // check if the snake has eaten the point
            if game.point_coord.is_none() {
                game.generate_new_point(width, height);
            } else if let Some(point) = &game.point_coord {
                if get_collision(&game.head_coord, point) {
                    game.increase_score();
                    game.generate_new_point(width, height);
                }
            }

            let last_corner_coord = game.corners.last().unwrap();

            frame.render_widget(
                Canvas::default()
                    .x_bounds([-width / 2.0, width / 2.0])
                    .y_bounds([-height, height])
                    .marker(Marker::HalfBlock)
                    .paint(|ctx| {
                        ctx.draw(
                            &(Rectangle {
                                x: -width / 2.0,
                                y: -height,
                                width,
                                height: height * 2.0,
                                color: Color::White,
                            }),
                        );

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
                                game.corners.windows(2).for_each(|arr| {
                                    let start_coord = &arr[0];
                                    let end_coord = &arr[1];

                                    ctx.draw(
                                        &(Line {
                                            x1: start_coord.x,
                                            y1: start_coord.y,
                                            x2: end_coord.x,
                                            y2: end_coord.y,
                                            color: Color::Blue,
                                        }),
                                    );
                                });

                                ctx.draw(
                                    &(Line {
                                        x1: game.head_coord.x,
                                        y1: game.head_coord.y,
                                        x2: last_corner_coord.x,
                                        y2: last_corner_coord.y,
                                        color: Color::Blue,
                                    }),
                                );

                                if let Some(point) = &game.point_coord {
                                    ctx.draw(
                                        &(Line {
                                            x1: point.x,
                                            y1: point.y,
                                            x2: point.x,
                                            y2: point.y,
                                            color: Color::Red,
                                        }),
                                    );
                                }
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
                            KeyCode::Char('a') => {
                                if game.direction != Direction::Right {
                                    game.change_direction(Direction::Left);
                                }
                            }
                            KeyCode::Char('d') => {
                                if game.direction != Direction::Left {
                                    game.change_direction(Direction::Right);
                                }
                            }
                            KeyCode::Char('w') => {
                                if game.direction != Direction::Down {
                                    game.change_direction(Direction::Up);
                                }
                            }
                            KeyCode::Char('s') => {
                                if game.direction != Direction::Up {
                                    game.change_direction(Direction::Down);
                                }
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                if game.state == GameState::GameOver {
                                    game.restart();
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
