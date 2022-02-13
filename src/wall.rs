use std::collections::LinkedList;
use oorandom::Rand32;
use crate::shared::{Segment, GridPosition, GRID_SIZE, Direction};
use crate::Snake;
use ggez::{graphics, Context, GameResult};
use ggez::graphics::Color;

pub struct Wall {
    pub body: LinkedList<Segment>
}

impl Wall {
    pub fn new(rng: &mut Rand32, snake: &Snake) -> Self {
        let mut body: LinkedList<Segment> = LinkedList::new();
        let mut pos: GridPosition;

        let mut snake_positions = snake.body.clone().into_iter().map(|x| x.pos).collect::<Vec<GridPosition>>();
        snake_positions.push(snake.head.pos);

        loop {
            pos = GridPosition::random(rng, GRID_SIZE.0, GRID_SIZE.1);

            if !snake_positions.contains(&pos) {
                break;
            }
        }

        let available_x = if pos.x >= 25 { Direction::Left } else { Direction::Right };
        let available_y = if pos.y >= 15 { Direction::Up } else { Direction::Down };

        let final_direction = match rng.rand_range(0..2) {
            0 => available_x,
            1 => available_y,
            _ => Direction::Right,
        };

        body.push_back(Segment::new(pos));
        for i in 1..6 {
            let x = match final_direction {
                Direction::Left => pos.x - i,
                Direction::Right => pos.x + i,
                _ => pos.x,
            };

            let y = match final_direction {
                Direction::Up => pos.y - i,
                Direction::Down => pos.y + i,
                _ => pos.y,
            };

            body.push_back(Segment::new(GridPosition::new(x, y)));
        }

        return Wall { body }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for seg in self.body.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.pos.into(),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        Ok(())
    }
}