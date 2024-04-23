use crate::ball::Ball;
use crate::planet::Planet;
use crate::drag_drop_fling::Drag_drop_fling;
use tetra::math::Vec2;
use tetra::graphics::{Color, Texture, Camera, DrawParams};
use tetra::time::get_fps;
use tetra::{input, window, Context};
use tetra::{ContextBuilder, State};
use tetra::graphics;
use tetra::input::{Key, MouseButton};
use tetra::Event;
use tetra::TetraError;
use rand::Rng;
use std::f32::consts::PI;

struct Npc {
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
    fn new(ctx: &mut Context, planet_center: Vec2<f32>, orbit_radius: f32) -> tetra::Result<Npc> {
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

pub struct GameState {
    pub balls: Vec<Ball>,
    pub planet: Planet,
    pub drag_drop_fling: Drag_drop_fling,
    pub npcs: Vec<Npc>,
    pub is_right_click: bool,
    pub camera: Camera,
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());
        for ball in &mut self.balls {
            ball.update(ctx, &self.planet);
        }
        for npc in &mut self.npcs {
            npc.update(); // Update each NPC's position
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        for ball in &self.balls {
            ball.draw(ctx);
        }
        self.planet.draw(ctx);
        self.drag_drop_fling.draw(ctx)?;
    
        // Create DrawParams object inside the loop
        for npc in &self.npcs {
            let draw_params = DrawParams::new();  // Recreate for each NPC
            npc.draw(ctx, draw_params)?;
        }
    
        window::set_title(ctx, &format!("Planet Game - {:.0} FPS", get_fps(ctx)));
    
        Ok(())
    }
    
    
    #[allow(unused_variables)]
    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        match event {
            Event::MouseMoved { position, delta } => {
                let mouse_pos = input::get_mouse_position(ctx);
                self.drag_drop_fling.current_position = self.camera.mouse_position(ctx);
                if self.is_right_click {
                    self.camera.position -= delta * (Vec2::new(1.0, 1.0) / self.camera.scale);
                    self.camera.update();
                }
            },

            Event::MouseButtonReleased { button } => {
                if button == MouseButton::Left {
                    let vector_dif = self.drag_drop_fling.end_drag();
                    let start_pos = self.drag_drop_fling.start_position;
                    self.spawn_ball(
                        ctx,
                        start_pos,
                        vector_dif,
                        5.0, 2.5,
                        Color::rgb(0.05, 0.8, 0.4)
                    )?;
                } else if button == input::MouseButton::Right {
                    self.is_right_click = false;
                }
            },

            Event::MouseWheelMoved { amount } => {
                if amount.y > 0 {
                    self.camera.scale = self.camera.scale * 1.1;
                } else if amount.y < 0 {
                    self.camera.scale = self.camera.scale * 0.9
                }
                self.camera.update();
            }

            #[allow(unused_variables)]
            Event::MouseButtonPressed { button } => {
                if button == input::MouseButton::Left {
                    let mouse_pos = input::get_mouse_position(ctx);
                    self.drag_drop_fling.start_drag(self.camera.mouse_position(ctx));
                } else if button == input::MouseButton::Right {
                    self.is_right_click = true;
                }
            },

            Event::KeyReleased { key } => {
                match key {
                    Key::W => {
                        self.camera.position.y -= 20.0;
                        self.camera.update();
                    }
                    Key::A => {
                        self.camera.position.x -= 20.0;
                        self.camera.update();
                    }
                    Key::S => {
                        self.camera.position.y += 20.0;
                        self.camera.update();
                    },
                    Key::D => {
                        self.camera.position.x += 20.0;
                        self.camera.update();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    
        Ok(())
    }
}

impl GameState {
    pub fn start() -> tetra::Result {
        ContextBuilder::new("Planet Game", 1280, 720)
            .show_mouse(true)
            .resizable(true)
            .multisampling(8)
            .build()?
            .run(Self::new)
    }

    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let ball = Ball::new(ctx,  Vec2::new(-200.0, -200.0),
            Vec2::new(50.0, 0.0), 5.0, 2.5, Color::rgb(0.05, 0.8, 0.4))?;
        let planet: Planet = Planet::new(ctx, Vec2::new(0.0, 0.0),
            1000.0, 10000000000.0, Color::rgb(0.05, 0.8, 0.4))?;

        let mut npcs = Vec::new();
        for _ in 0..10 {  // Generate 10 NPCs
            let npc = Npc::new(ctx, planet.position, 1000.0)?;
            npcs.push(npc);
        }

        Ok(GameState {
            balls: vec![ball],
            planet: planet,
            drag_drop_fling: Drag_drop_fling::new(),
            npcs: npcs,
            camera: Camera::new(1280.0, 720.0),
            is_right_click: false,
        })
    }

    pub fn spawn_ball(&mut self, ctx: &mut Context, position: Vec2<f32>, velocity: Vec2<f32>, radius: f32, mass: f32, color: Color) -> tetra::Result<()> {
        let new_ball = Ball::new(ctx, position, velocity, radius, mass, color)?;
        self.balls.push(new_ball);
        Ok(())
    }
}
