use tetra::math::Vec2;
use tetra::Context;
use tetra::graphics::mesh::Mesh;
use tetra::graphics::{self, Color};

pub struct Drag_drop_fling {
    pub start_position: Vec2<f32>,
    pub current_position: Vec2<f32>,
    pub is_dragging: bool,
}

impl Drag_drop_fling {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            start_position: Vec2::zero(),
            current_position: Vec2::zero(),
        }
    }

    pub fn start_drag(&mut self, start_position: Vec2<f32>) {
        self.is_dragging = true;
        self.start_position = start_position;
        self.current_position = start_position;
    }

    pub fn update_drag(&mut self, current_position: Vec2<f32>) {
        if self.is_dragging {
            self.current_position = current_position;
        }
    }

    pub fn end_drag(&mut self) -> Vec2<f32> {
        self.is_dragging = false;

        let dif_mouse = self.current_position - self.start_position;
        return dif_mouse;
    }

    pub fn draw(&self, ctx: &mut Context) -> tetra::Result<()> {
        if self.is_dragging {
            let mesh = Mesh::polyline(ctx, 5.0, &[self.start_position, self.current_position])?;
            mesh.draw(ctx, graphics::DrawParams::new().color(Color::rgb(0.2, 0.3, 0.4)));
        }
        Ok(())
    }
}


// let dif_mouse = self.mouse_current_position - self.mouse_down_position;
