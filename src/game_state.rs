use crate::ball::Ball;
use crate::planet::Planet;
use crate::drag_drop_fling::Drag_drop_fling;
use tetra::math::Vec2;
use tetra::graphics::Color;
use tetra::time::get_fps;
use tetra::{window, Context};
use tetra::{ContextBuilder, State};
use tetra::graphics;
use tetra::input::{Key, MouseButton};
use tetra::Event;
use tetra::TetraError;

pub struct GameState {
    pub balls: Vec<Ball>,
    pub planet: Planet,
    pub drag_drop_fling: Drag_drop_fling,
    pub mouse_position: Vec2<f32>,
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        for ball in &mut self.balls {
            ball.update(ctx, &self.planet);
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        for ball in &self.balls {
            ball.draw(ctx);
        }
        self.planet.draw(ctx);
        self.drag_drop_fling.draw(ctx)?;

        window::set_title(ctx, &format!("Planet Game - {:.0} FPS", get_fps(ctx)));

        Ok(())
    }

    #[allow(unused_variables)]
    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        match event {
            #[allow(unused_variables)]
            Event::MouseMoved { position, delta } => {
                self.mouse_position = position;
                self.drag_drop_fling.current_position = position;
            }
            Event::MouseButtonReleased { button } => {
                
                if button == MouseButton::Left {
                    let vector_dif = self.drag_drop_fling.end_drag();
                    let start_pos = self.drag_drop_fling.start_position;
                    self.spawn_ball(
                        ctx,
                        start_pos,
                        vector_dif,
                        5.0,2.5,
                        Color::rgb(0.05, 0.8, 0.4)
                    )?;
                }
            }
            #[allow(unused_variables)]
            Event::MouseButtonPressed { button} => {
                self.drag_drop_fling.start_drag(self.mouse_position)
            },
            Event::KeyReleased { key } => {
                match key {
                    Key::S => {
                        println!("Key press")
                    },
                    Key::D => {
                        println!("Key press")
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
        let ball = Ball::new(ctx,  Vec2::new(1280.0/2.0, 275.0),
            Vec2::new(100.0, 0.0), 5.0, 2.5, Color::rgb(0.05, 0.8, 0.4))?;
        let planet: Planet = Planet::new(ctx, Vec2::new(1280.0/2.0, 720.0/2.0),
            50.0, 1000000.0, Color::rgb(0.05, 0.8, 0.4))?;

        let second_ball = Ball::new(ctx,  Vec2::new(1280.0/2.0, 100.0),
        Vec2::new(60.0, 0.0), 5.0, 2.5, Color::rgb(0.05, 0.8, 0.4))?;

        let mut balls: Vec<Ball> = vec![ball];

        balls.push(second_ball);

        Ok(GameState {
            balls: balls,
            planet: planet,
            drag_drop_fling: Drag_drop_fling::new(),
            mouse_position: Vec2::new(0.0, 0.00),
        })
    }

    pub fn spawn_ball(&mut self, ctx: &mut Context, position: Vec2<f32>, velocity: Vec2<f32>, radius: f32, mass: f32, color: Color) -> tetra::Result<()> {
        let new_ball = Ball::new(ctx, position, velocity, radius, mass, color)?;
        self.balls.push(new_ball);
        Ok(())
    }

    pub fn despawn_last_ball(&mut self) -> tetra::Result<()> {
        self.balls.pop();
        Ok(())
    }
}
