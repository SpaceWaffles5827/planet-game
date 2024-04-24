use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::math::{vec2, Vec2};
use tetra::Context;
use tetra::graphics::Rectangle;
use tetra::graphics::{self, Color};

pub struct Grid {
    origin: Vec2<f32>,
    pixel_size: f32,
    width: f32,
    height: f32,
    mesh: Mesh,  // Store the mesh
}

impl Grid {
    pub fn new(ctx: &mut Context, origin: Vec2<f32>, pixel_size: f32, width: f32, height: f32) -> tetra::Result<Grid> {
        let mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, pixel_size, pixel_size)
        )?;
        
        Ok(Grid {
            origin,
            pixel_size,
            width,
            height,
            mesh,
        })
    }

    pub fn draw(&self, ctx: &mut Context) {

        let top_left = Vec2::new(self.origin.x - self.width / 2.0, self.origin.y - self.height / 2.0);

        for x in 0..(self.width / self.pixel_size) as i32 {
            for y in 0..(self.height / self.pixel_size) as i32 {
                self.mesh.draw(ctx, graphics::DrawParams::new()
                    .position(top_left + Vec2::new(x as f32 * self.pixel_size, y as f32 * self.pixel_size))
                    .color(Color::rgb(0.5, 0.5, 0.5)));  // Gray color
            }
        }
    }
}
