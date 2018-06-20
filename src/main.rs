extern crate ggez;

use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics};
use ggez::event::{KeyCode, KeyMods};


struct MainState {
    hero: Hero
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            hero: Hero::new(ctx)?
        })
    }
}

struct Hero {
    rect: graphics::Rect
}
impl Hero {
    fn new(_ctx: &mut Context) -> GameResult<Hero> {
        Ok(Hero {
            rect: graphics::Rect::new(100.0, 100.0, 10.0, 10.0)
        })
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::rectangle(ctx, [1.0, 0.55, 0.0, 1.0].into(), graphics::DrawMode::Fill, *&self.rect)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        &self.hero.draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ctx.quit(),
            _ => println!("{:?}", keycode)
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("adventure", "quantumtrip")
        .window_setup(conf::WindowSetup::default().title("Adventure"))
        .build()?;
    
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}