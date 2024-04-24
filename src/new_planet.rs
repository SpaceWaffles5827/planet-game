use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Color;
use tetra::math::Vec2;
use tetra::Context;

pub struct Planet {
    pub mesh: Mesh,
    pub position: Vec2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
}

impl Planet {
    pub fn new(ctx: &mut Context, position: Vec2<f32>, radius: f32, mass: f32, color: Color) -> tetra::Result<Planet> {
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        Ok(Planet { mesh, position, mass, radius, color })
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.mesh.draw(
            ctx,
        tetra::graphics::DrawParams::new()
                .position(self.position)
                .color(self.color)
        );
    }
}