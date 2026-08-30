use ggez::{
    Context, ContextBuilder,
    event::{self, EventHandler},
    graphics::{self, Color},
};
use mlua::{Lua, Result as ResultLua, Table};

pub struct Engine {
    buffer: Vec<Draw>,
}

pub enum Draw {
    Rectangle(f32, f32, f32, f32),
}

impl Engine {
    pub fn new(_ctx: &mut Context) -> Engine {
        // Load/create resources such as images here.
        Self { buffer: Vec::new() }
    }
}

impl EventHandler for Engine {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(_ctx)
    }
}

pub fn create_window(name: &str, author: &str) {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new(name, author)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Engine::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

pub fn init_graphics_env(lua: &Lua) -> ResultLua<()> {
    let globals = lua.globals();
    let create_window_fn = lua.create_function(|_, config_table: Table| {
        let name: String = config_table.get("title")?;
        let author: String = config_table.get("author")?;
        create_window(&name, &author);
        Ok(())
    })?;
    globals.set("create_window", create_window_fn)?;
    Ok(())
}
