extern crate ggez;
use ggez::*;

struct State {
    //text: graphics::Text
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);        
        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let state = &mut State {
        
    };
    
    let ctx = &mut ContextBuilder::new("adventure", "quantumtrip")
        .window_setup(conf::WindowSetup::default().title("Trit's Adventure"))
        .window_mode(conf::WindowMode::default()
            .dimensions(1920,1080)
            .borderless(true))
        .build().unwrap();

    mouse::set_grabbed(ctx, true);

    event::run(ctx, state).unwrap();
}