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

#[cfg(test)]
mod tests {
    use ggez::{graphics::Color};

    use crate::{GameState, shared::Direction, wall::Wall, snake::Ate};

    #[test]
    fn eats_correct_food_score_increments() {
        let gs = &mut GameState::new();
        gs.color = Color::BLUE;
        let food_index = 0;
        set_food_color(gs, food_index, Color::BLUE);
        eat_food(gs, food_index);

        assert_eq!(gs.score, 0);
        gs.update();
        assert_eq!(gs.score, 1);
    }

    #[test]
    fn eats_incorrect_food_score_no_change() {
        let gs = &mut GameState::new();
        gs.color = Color::BLUE;
        let food_index = 0;
        set_food_color(gs, food_index, Color::RED);
        eat_food(gs, food_index);

        assert_eq!(gs.score, 0);
        gs.update();
        assert_eq!(gs.score, 0);
    }

    #[test]
    fn has_one_wall_eats_correct_food_wall_disapears() {
        let gs = &mut GameState::new();
        gs.color = Color::BLUE;
        gs.walls.push(Wall::new(&mut gs.rng, &gs.snake));
        let food_index = 0;
        set_food_color(gs, food_index, gs.color);
        eat_food(gs, food_index);

        assert_eq!(gs.walls.len(), 1);
        gs.update();
        assert_eq!(gs.walls.len(), 0);
    }

    #[test]
    fn eats_incorrect_food_walls_increase() {
        let gs = &mut GameState::new();
        gs.color = Color::BLUE;
        let food_index = 0;
        set_food_color(gs, food_index, Color::RED);
        eat_food(gs, food_index);

        assert_eq!(gs.walls.len(), 0);
        gs.update();
        assert_eq!(gs.walls.len(), 1);
    }

    #[test]
    fn food_always_2() {
        let gs = &mut GameState::new();
        assert_eq!(gs.food.len(), 2);

        eat_food(gs, 0);
        eat_food(gs, 1);
        gs.update();
        assert_eq!(gs.food.len(), 2);
    }

    #[test]
    fn eats_self_gameover() {
        let gs = &mut GameState::new();
        gs.snake.dir = Direction::Right;
        gs.snake.head.pos = gs.snake.body.front().unwrap().pos;
        gs.snake.head.pos.x -= 1;

        assert_eq!(gs.gameover, false);
        gs.update();
        assert_eq!(gs.snake.ate.unwrap(), Ate::Itself);
        assert_eq!(gs.gameover, true);
    }

    #[test]
    fn bumps_wall_gameover() {
        let gs = &mut GameState::new();
        gs.color = Color::BLUE;
        let food_index = 0;
        set_food_color(gs, food_index, Color::RED);
        eat_food(gs, food_index);
        gs.update();

        gs.snake.dir = Direction::Right;
        gs.snake.head.pos = gs.walls.first().unwrap().body.front().unwrap().pos;
        gs.snake.head.pos.x -= 1;
        gs.update();
        assert_eq!(gs.snake.ate.unwrap(), Ate::Wall);
        assert_eq!(gs.gameover, true);
    }

    fn eat_food(gs: &mut GameState, index: usize) {
        gs.snake.dir = Direction::Right;
        gs.snake.head.pos = gs.food[index].pos;
        gs.snake.head.pos.x -= 1;
    }

    fn set_food_color(gs: &mut GameState, index: usize, color: Color) {
        gs.food[index].color = color;
    }
}

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

    fn update_allowed_color(&mut self) {
        self.color = match self.rng.rand_range(0..2) {
            0 => self.food[0].color,
            1 => self.food[1].color,
            _ => Color::BLUE
        };
    }

    fn update_walls(&mut self, color: Color) {
        if color == self.color {
            self.score += 1;
            self.walls.pop();

        } else {
            self.walls.push(wall::Wall::new(&mut self.rng, &self.snake))
        }
    }

    fn generate_food_pos(&mut self, forbidden_positions: Vec<GridPosition>) -> GridPosition {
        loop {
            let new_food_pos = GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);

            if !forbidden_positions.contains(&new_food_pos) {
                return new_food_pos;
            }
        }
    }

    fn get_eaten_food_index_and_color(&self) -> (usize, Color) {
        let eaten_food_index: usize = self.food.iter().position(|food| food.pos == self.snake.head.pos).unwrap();
        return (eaten_food_index, self.food[eaten_food_index].color);
    }

    fn update_eaten_food(&mut self, index: usize) {
        let walls_positions = self.walls.iter()
                                .flat_map(|x| x.body.iter().map(|seg| seg.pos)).collect::<Vec<shared::GridPosition>>();
        let pos = self.generate_food_pos(walls_positions);

        self.food[index].pos = pos;
        self.food[index].change_color(&mut self.rng);
    }

    fn update(&mut self) {
        if !self.gameover {
            self.snake.update(&self.food, &self.walls);
            if let Some(ate) = self.snake.ate {
                match ate {
                    Ate::Food => {
                        let (eaten_food_index, eaten_food_color) = self.get_eaten_food_index_and_color();
                        
                        self.update_walls(eaten_food_color);
                        self.update_eaten_food(eaten_food_index);
                        self.update_allowed_color();

                    }
                    _ => { self.gameover = true; }
                }
            }
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.update();
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
