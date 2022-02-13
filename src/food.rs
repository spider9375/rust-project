use ggez::graphics::Color;
use oorandom::Rand32;
use ggez::{graphics, Context, GameResult};
use crate::shared::GridPosition;

pub struct Food {
    pub pos: GridPosition,
    pub color: Color,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Food { pos, color: Color::BLUE }
    }

    pub fn change_color(&mut self, rng: &mut Rand32) {
        self.color = generate_random_color(rng);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.pos.into(), self.color)?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
    }
}

fn generate_random_color(rng: &mut Rand32) -> Color {
    match rng.rand_range(1..6) {
        1 => Color::GREEN,
        2 => Color::BLUE,
        3 => Color::GREEN,
        4 => Color::RED,
        5 => Color::MAGENTA,
        6 => Color::CYAN,
        _ => Color::WHITE,
    }
}
