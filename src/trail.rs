use tetra::graphics::mesh::Mesh;
use tetra::math::Vec2;
use tetra::Context;
use tetra::graphics::{self, Color};
use std::collections::VecDeque;

pub struct Trail {
    positions: VecDeque<Vec2<f32>>,
    max_length: usize,
    line_width: f32,
    color: Color,
}

impl Trail {
    pub fn new(max_length: usize, line_width: f32, color: Color) -> Trail {
        Trail {
            positions: VecDeque::with_capacity(max_length),
            max_length,
            line_width,
            color,
        }
    }

    pub fn push(&mut self, position: Vec2<f32>) {
        if self.positions.len() >= self.max_length {
            self.positions.pop_front();
        }
        self.positions.push_back(position);
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }

    pub fn draw(&self, ctx: &mut Context) -> tetra::Result<()> {
        if self.positions.len() > 1 {
            let positions_vec: Vec<_> = self.positions.iter().copied().collect();
            let mesh = Mesh::polyline(ctx, self.line_width, &positions_vec)?;
            mesh.draw(ctx, graphics::DrawParams::new().color(self.color));
        }
        Ok(())
    }
}
