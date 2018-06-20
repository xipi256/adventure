extern crate ggez;

use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics};
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;

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

    fn change_pos(&mut self, d_x: f32, d_y: f32) {
        self.rect.translate(na::base::Vector2::new(d_x, d_y))
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        self.hero.draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let change_amount = 10.0;
        match keycode {
            KeyCode::K | KeyCode::Up | KeyCode::Numpad8 => { 
                self.hero.change_pos(0.0, -change_amount)
            }
            KeyCode::J | KeyCode::Down | KeyCode::Numpad2 => { 
                self.hero.change_pos(0.0, change_amount)
            }
            KeyCode::H | KeyCode::Left | KeyCode::Numpad4 => { 
                self.hero.change_pos(-change_amount, 0.0)
            }
            KeyCode::L | KeyCode::Right | KeyCode::Numpad6 => { 
                self.hero.change_pos(change_amount, 0.0)
            }
            KeyCode::Y | KeyCode::Numpad7 => {
                self.hero.change_pos(-change_amount, -change_amount)
            }
            KeyCode::U | KeyCode::Numpad9 => {
                self.hero.change_pos(change_amount, -change_amount)
            }
            KeyCode::B | KeyCode::Numpad1 => {
                self.hero.change_pos(-change_amount, change_amount)
            }
            KeyCode::N | KeyCode::Numpad3 => {
                self.hero.change_pos(change_amount, change_amount)
            }
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