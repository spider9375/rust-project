use oorandom::Rand32;

use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{Color, Text};
use ggez::{event, graphics, timer, Context, GameResult};

mod wall;
mod snake;
mod shared;
mod food;
use wall::Wall;
use snake::{Snake, Ate};
use food::Food;
use crate::shared::{Direction, GridPosition, SCREEN_SIZE, GRID_SIZE, DESIRED_FPS};

struct GameState {
    walls: Vec<Wall>,
    color: Color,
    score: i128,
    snake: Snake,
    food: [Food; 2],
    gameover: bool,
    rng: Rand32,
}

impl GameState {
    pub fn new() -> Self {
        let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        let food_pos = GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);
        let food_pos2 = GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

        GameState {
            walls: Vec::new(),
            color: Color::BLUE,
            score: 0,
            snake: Snake::new(snake_pos),
            food: [Food::new(food_pos), Food::new(food_pos2)],
            gameover: false,
            rng,
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            if !self.gameover {
                self.snake.update(&self.food, &self.walls);
                if let Some(ate) = self.snake.ate {
                    match ate {
                        Ate::Food => {
                            let eaten_food_index: usize = self.food.iter().position(|food| food.pos == self.snake.head.pos).unwrap();
                            let eaten_food: &mut food::Food = &mut self.food[eaten_food_index];

                            if eaten_food.color == self.color {
                                self.score += 1;
                                self.walls.pop();

                            } else {
                                self.walls.push(wall::Wall::new(&mut self.rng, &self.snake))
                            }

                            let pos_array = self.walls.iter()
                                .flat_map(|x| x.body.iter().map(|seg| seg.pos)).collect::<Vec<shared::GridPosition>>();

                            loop {
                                let new_food_pos = GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);

                                if !pos_array.contains(&new_food_pos) {
                                    eaten_food.pos = new_food_pos;
                                    eaten_food.change_color(&mut self.rng);
                                    break;
                                }
                            }

                            self.color = match self.rng.rand_range(0..2) {
                                0 => self.food[0].color,
                                1 => self.food[1].color,
                                _ => Color::BLUE
                            };

                            // self.color = generate_random_color(&mut self.rng);
                        }
                        // If it ate itself, we set our gameover state to true.
                        Ate::Itself => {
                            self.gameover = true;
                        }

                        Ate::Wall => {
                            self.gameover = true;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK.into());
        self.snake.draw(ctx)?;

        for wall in self.walls.iter().enumerate() {
            wall.1.draw(ctx)?;
        }

        for food in self.food.iter().enumerate() {
            food.1.draw(ctx)?;
        }

        draw_score(self.score, ctx)?;
        draw_correct_color(self.color, ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        }
    }
}

fn draw_score(score : i128, ctx: &mut Context) -> GameResult<()> {
    let score = Text::new(format!(
        "Score: {}", score
    ));

    graphics::draw(
        ctx,
        &score,
        (ggez::mint::Point2 { x: 0.0, y: 0.0 }, Color::WHITE),
    )
}

fn draw_correct_color(color: Color, ctx: &mut Context) -> GameResult<()> {
    let txt = Text::new("Allowed Color");

    graphics::draw(
        ctx,
        &txt,
        (ggez::mint::Point2 { x: 100.0, y: 0.0 }, color),
    )
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("snake", "Dimitar Mihaylov")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new();
    event::run(ctx, events_loop, state)
}
