extern crate ggez;
use ggez::*;

struct State {
    dt: std::time::Duration
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::get_delta(ctx);
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }
}

fn main() {
    let state = &mut State {
        dt: std::time::Duration::new(0, 0)
    };
    let conf = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("adventure", "trit", conf).unwrap();

    event::run(ctx, state).unwrap();
}