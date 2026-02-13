use macroquad::prelude::*;
use rhai::{Engine, Scope, AST};

pub struct VM {
    engine: Engine,
    ast: Option<AST>,
    scope: Scope<'static>,
}

impl VM {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Rhai uses f64 by default.
        // circle(x, y, r, r, g, b)
        engine.register_fn("circle", |x: f64, y: f64, r: f64, red: f64, green: f64, blue: f64| {
            draw_circle(x as f32, y as f32, r as f32, Color::new(red as f32, green as f32, blue as f32, 1.0));
        });

        // rect(x, y, w, h, r, g, b)
        engine.register_fn("rect", |x: f64, y: f64, w: f64, h: f64, red: f64, green: f64, blue: f64| {
            draw_rectangle(x as f32, y as f32, w as f32, h as f32, Color::new(red as f32, green as f32, blue as f32, 1.0));
        });

        engine.register_fn("time", || {
            get_time() as f64
        });

        engine.register_fn("mouse_x", || mouse_position().0 as f64);
        engine.register_fn("mouse_y", || mouse_position().1 as f64);

        engine.register_fn("screen_w", || screen_width() as f64);
        engine.register_fn("screen_h", || screen_height() as f64);

        // Math helpers
        engine.register_fn("sin", |x: f64| x.sin());
        engine.register_fn("cos", |x: f64| x.cos());

        Self {
            engine,
            ast: None,
            scope: Scope::new(),
        }
    }

    pub fn load_script(&mut self, script: &str) -> Result<(), Box<rhai::EvalAltResult>> {
        let ast = self.engine.compile(script)?;
        self.ast = Some(ast);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<rhai::EvalAltResult>> {
        if let Some(ast) = &self.ast {
             self.engine.run_ast_with_scope(&mut self.scope, ast)?;
        }
        Ok(())
    }
}
