use tetra::math::Vec2;
use tetra::graphics::Texture;
use tetra::Context;
use rand::Rng;
use std::f32::consts::PI;

pub struct Npc {
    texture: Texture,
    position: Vec2<f32>,
    rotation: f32,
    orbit_center: Vec2<f32>,
    orbit_radius: f32,
    current_angle: f32,
    target_angle: f32,
    angular_velocity: f32,
}

impl Npc {
    pub fn new(ctx: &mut Context, planet_center: Vec2<f32>, orbit_radius: f32) -> tetra::Result<Npc> {
        let texture = Texture::new(ctx, "./src/resources/spaceMan.png")?;
        let initial_angle = rand::thread_rng().gen_range(0.0..2.0 * PI);
        let target_angle = rand::thread_rng().gen_range(0.0..2.0 * PI);

        Ok(Npc {
            texture,
            position: Npc::calculate_position(planet_center, orbit_radius, initial_angle),
            rotation: 0.0,
            orbit_center: planet_center,
            orbit_radius,
            current_angle: initial_angle,
            target_angle: target_angle,
            angular_velocity: 0.001,  // This can be adjusted for speed
        })
    }

    fn calculate_position(orbit_center: Vec2<f32>, orbit_radius: f32, angle: f32) -> Vec2<f32> {
        Vec2::new(
            orbit_center.x + orbit_radius * angle.cos(),
            orbit_center.y + orbit_radius * angle.sin()
        )
    }

    fn update_target_angle(&mut self) {
        self.target_angle = rand::thread_rng().gen_range(0.0..2.0 * PI);
    }

    pub fn update(&mut self) {
        let angle_difference = (self.target_angle - self.current_angle + 2.0 * PI) % (2.0 * PI);

        if angle_difference > 0.0 && angle_difference < self.angular_velocity {
            self.current_angle = self.target_angle;  // Snap to the target angle if very close
            self.update_target_angle();  // Select a new target angle once reached
        } else {
            if angle_difference > PI {  // If more than 180 degrees away, decrease angle
                self.current_angle -= self.angular_velocity;
            } else {  // If less than 180 degrees away, increase angle
                self.current_angle += self.angular_velocity;
            }
        }

        self.current_angle = (self.current_angle + 2.0 * PI) % (2.0 * PI);  // Normalize angle
        self.position = Npc::calculate_position(self.orbit_center, self.orbit_radius, self.current_angle);
        self.rotation = self.current_angle + PI / 2.0;
    }

    pub fn draw(&self, ctx: &mut Context, draw_params: tetra::graphics::DrawParams) -> tetra::Result<()> {
        self.texture.draw_region(
            ctx,
            tetra::graphics::Rectangle::new(0.0, 0.0, self.texture.width() as f32, self.texture.height() as f32),
            draw_params.position(self.position)
                .origin(Vec2::new(self.texture.width() as f32 / 2.0, self.texture.height() as f32))
                .rotation(self.rotation)
        );
        Ok(())
    }
}
