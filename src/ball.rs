use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Color;
use tetra::math::Vec2;
use tetra::{Context, time};
use crate::trail::Trail;
use crate::planet::Planet;

fn get_force_of_gravity(object1: &Ball, object2: &Planet) -> Vec2<f32> {
    // Constants
    // let g: f32 = 6.67430e-11; // Gravitational constant
    let g: f32  = 1.0;

    // Calculate distance
    let dx = object2.position.x - object1.position.x;
    let dy = object2.position.y - object1.position.y;
    let distance = (dx*dx + dy*dy).sqrt();

    // Avoid division by zero by adding a small epsilon if needed
    let distance = if distance == 0.0 { 1e-10 } else { distance };

    // Calculate normal vector
    let normal_vector = Vec2::new(dx / distance, dy / distance);

    // Calculate force magnitude
    let force_magnitude = g * (object1.mass * object2.mass) / (distance * distance);

    // Calculate force vector
    let force_vector = normal_vector * force_magnitude;

    force_vector
}

pub struct Ball {
    pub is_visable: bool,
    pub mass: f32,
    pub mesh: Mesh,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub radius: f32,
    pub color: Color,
    pub trail: Trail,
}

impl Ball {
    pub fn new(ctx: &mut Context, position: Vec2<f32>, velocity: Vec2<f32>, radius: f32, mass: f32, color: Color) -> tetra::Result<Ball> {
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        Ok(Ball {
            is_visable: true,
            mesh,
            position,
            velocity,
            radius,
            mass,
            color,
            trail: Trail::new(50, 5.0, Color::rgb(0.2, 0.3, 0.4)),
        })
    }

    pub fn update(&mut self, ctx: &Context, planet: &Planet) {
        let delta_time = time::get_delta_time(ctx).as_secs_f32();
    
        // Calculate gravitational force
        let force_gravity = get_force_of_gravity(self, planet);

        // Update position based on velocity and half the acceleration from gravity
        self.velocity += force_gravity / self.mass * delta_time;

        // Update position based on new velocity
        self.position += self.velocity * delta_time;

        // Calculate the distance and direction vector from the ball to the planet
        let dx = planet.position.x - self.position.x;
        let dy = planet.position.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let normal_vector = Vec2::new(dx / distance, dy / distance); // Normalized direction vector from ball to planet

        // Check for collision
        if distance <= self.radius + planet.radius {

            self.is_visable = false;

            // Reflect velocity
            let normal_velocity = self.velocity.dot(normal_vector);
            self.velocity -= 2.0 * normal_velocity * normal_vector;

            // Correct position if ball is inside the planet
            let overlap = (self.radius + planet.radius) - distance;
            self.position -= normal_vector * overlap;
        }
        self.trail.push(self.position);
    }


    pub fn draw(&self, ctx: &mut Context) {
        if self.is_visable {
            self.trail.draw(ctx);
            self.mesh.draw(ctx, self.position);
        }
    }
}
