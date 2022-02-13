use ggez::{graphics, Context, GameResult};
use std::collections::LinkedList;
use crate::shared::{GridPosition, Segment, Direction};
use crate::wall::Wall;
use crate::food::Food;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ate {
    Itself,
    Food,
    Wall,
}

pub struct Snake {
    pub head: Segment,
    pub dir: Direction,
    pub body: LinkedList<Segment>,
    pub ate: Option<Ate>,
    pub last_update_dir: Direction,
    pub next_dir: Option<Direction>,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Self {
        let mut body = LinkedList::new();
        body.push_back(Segment::new((pos.x - 1, pos.y).into()));
        Snake {
            head: Segment::new(pos),
            dir: Direction::Right,
            last_update_dir: Direction::Right,
            body,
            ate: None,
            next_dir: None,
        }
    }

    pub fn eats(&self, food: &Food) -> bool {
        self.head.pos == food.pos
    }

    pub fn eats_self(&self) -> bool {
        for seg in self.body.iter() {
            if self.head.pos == seg.pos {
                return true;
            }
        }
        false
    }

    pub fn bumps_wall(&self, wall: &Wall) -> bool {
        for wall_seg in wall.body.iter() {
            if self.head.pos == wall_seg.pos {
                return true;
            }
        }

        return false;
    }

    pub fn update(&mut self, food: &[Food], walls: &Vec<crate::wall::Wall>) {
        if self.last_update_dir == self.dir && self.next_dir.is_some() {
            self.dir = self.next_dir.unwrap();
            self.next_dir = None;
        }
        let new_head_pos = GridPosition::new_from_move(self.head.pos, self.dir);
        let new_head = Segment::new(new_head_pos);
        self.body.push_front(self.head);
        self.head = new_head;

        if self.eats_self() {
            self.ate = Some(Ate::Itself);
        } else if self.eats(&food[0]) || self.eats(&food[1]) {
            self.ate = Some(Ate::Food);
        } else {
            self.ate = None
        }

        for wall in walls {
            if self.bumps_wall(wall) {
                self.ate = Some(Ate::Wall);
                continue;
            }
        }
        if self.ate.is_none() {
            self.body.pop_back();
        }

        self.last_update_dir = self.dir;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for seg in self.body.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.pos.into(),
                [0.3, 0.3, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.head.pos.into(),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
