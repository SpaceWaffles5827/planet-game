use crate::ball::Ball;
use crate::planet::Planet;
use crate::drag_drop_fling::Drag_drop_fling;
use crate::npc::Npc;
use tetra::math::Vec2;
use tetra::graphics::{Color, Camera, Shader, DrawParams};
use tetra::time::get_fps;
use tetra::{input, window, Context};
use tetra::{ContextBuilder, State};
use tetra::graphics;
use tetra::input::{Key, MouseButton};
use tetra::Event;
use tetra::TetraError;

pub struct GameState {
    pub balls: Vec<Ball>,
    pub planet: Planet,
    pub drag_drop_fling: Drag_drop_fling,
    pub npcs: Vec<Npc>,
    pub is_right_click: bool,
    pub camera: Camera,
    shader: Shader,
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

        graphics::set_shader(ctx, &self.shader);
        self.planet.draw(ctx);
        graphics::reset_shader(ctx);

        self.drag_drop_fling.draw(ctx)?;
        graphics::set_shader(ctx, &self.shader);
    
        for npc in &self.npcs {
            let draw_params = DrawParams::new();  // Recreate for each NPC
            npc.draw(ctx, draw_params)?;
        }
        graphics::reset_shader(ctx);

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
            250.0, 10000000000.0, Color::rgb(1.0, 1.0, 1.0))?;

        let mut npcs = Vec::new();
        for _ in 0..10 {  // Generate 10 NPCs
            let npc = Npc::new(ctx, planet.position, 250.0)?;
            npcs.push(npc);
        }

        
        let shader = Shader::new(
            ctx,
            "./src/vertex_shader.glsl",
            "./src/fragment_shader.glsl",
        )?;

        // let shader = Shader::from_fragment_file(ctx, "./src/fragment_shader.glsl")?;

        Ok(GameState {
            balls: vec![ball],
            planet: planet,
            drag_drop_fling: Drag_drop_fling::new(),
            npcs: npcs,
            camera: Camera::new(1280.0, 720.0),
            is_right_click: false,
            shader: shader,
        })
    }

    pub fn spawn_ball(&mut self, ctx: &mut Context, position: Vec2<f32>, velocity: Vec2<f32>, radius: f32, mass: f32, color: Color) -> tetra::Result<()> {
        let new_ball = Ball::new(ctx, position, velocity, radius, mass, color)?;
        self.balls.push(new_ball);
        Ok(())
    }
}
