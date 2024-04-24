use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{self, Color, Canvas, DrawParams};
use tetra::math::Vec2;
use tetra::Context;

pub struct Planet {
    pub canvas: Canvas,
    pub position: Vec2<f32>,
    pub mass: f32,
    pub radious: f32,
    pub color: Color,
}

impl Planet {
    pub fn new(ctx: &mut Context, position: Vec2<f32>, radious: f32, mass: f32, color: Color) -> tetra::Result<Planet> {
        // let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        // let mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, graphics::Rectangle::new(0.0, 0.0, 20.0, 20.0))?;
        let canvas = Canvas::new(ctx, (radious*2.0) as i32, (radious*2.0) as i32)?;

        let width = canvas.width() as usize;
        let height = canvas.height() as usize;
        let mut data = vec![0u8; width * height * 4];
        let center_x = (width / 2) as f32;
        let center_y = (height / 2) as f32;
        let radius_squared = radious * radious;

        for y in 0..height {
            for x in 0..width {
                let idx = (x + y * width) * 4;
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance_squared = dx * dx + dy * dy;
    
                // Check if the pixel is inside the circle
                if distance_squared <= radius_squared {
                    data[idx] = 255u8;   
                    data[idx + 1] = 255u8;
                    data[idx + 2] = 255u8;
                    data[idx + 3] = 255; // Alpha (fully opaque)
                } else {
                    data[idx + 3] = 0;   // Make outside of the circle transparent
                }
            }
        }

        canvas.set_data(ctx, 0, 0, width as i32, height as i32, &data);

        Ok(Planet { canvas, position, mass, radious, color })
    }

    pub fn draw(&self, ctx: &mut Context) {
            self.canvas.draw(ctx, DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(self.radious, self.radious)));
        }
}