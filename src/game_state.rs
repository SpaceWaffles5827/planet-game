use crate::ball::Ball;
use crate::planet::Planet;
use tetra::math::Vec2;
use tetra::graphics::Color;
use tetra::time::get_fps;
use tetra::Context;
use tetra::{ContextBuilder, State};
use tetra::graphics;
use tetra::input::Key;

pub struct GameState {
    pub balls: Vec<Ball>,
    pub planet: Planet,
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        for ball in &mut self.balls {
            ball.update(ctx, &self.planet);
        }

        let fps = get_fps(ctx);
        println!("FPS: {}", fps);
        
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        for ball in &self.balls {
            ball.draw(ctx);
        }
        self.planet.draw(ctx);
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: tetra::Event) -> Result<(), tetra::TetraError> {
        if let tetra::Event::KeyReleased { key } = event {
            if key == Key::S {
                self.spawn_ball(ctx, Vec2::new(300.0, 300.0), Vec2::new(0.0, 10.0), 5.0, 2.5, Color::rgb(0.2, 0.3, 0.4))?;
            } else if key == Key::D {
                self.despawn_ball();
            }
        }
        Ok(())
    }
}

impl GameState {
    pub fn start() -> tetra::Result {
        ContextBuilder::new("Planet Game", 1280, 720)
            .show_mouse(true)
            .resizable(true)
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

        Ok(GameState { balls, planet })
    }

    pub fn spawn_ball(&mut self, ctx: &mut Context, position: Vec2<f32>, velocity: Vec2<f32>, radius: f32, mass: f32, color: Color) -> tetra::Result<()> {
        let new_ball = Ball::new(ctx, position, velocity, radius, mass, color)?;
        self.balls.push(new_ball);
        Ok(())
    }

    pub fn despawn_ball(&mut self) -> tetra::Result<()> {
        self.balls.pop();
        Ok(())
    }
}
