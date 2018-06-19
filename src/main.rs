extern crate ggez;

use ggez::{
    Context, ContextBuilder, graphics, event, GameResult, conf,
    mouse
};
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use std::{path, env};

struct State {
    
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State{})
    }
}

struct Hero {
    font: graphics::Font,
    text: graphics::Text,
    pos: na::Point2<f32>
}

impl Hero {
    fn new(ctx: &mut Context) -> GameResult<Hero> {
        let font = graphics::Font::new(ctx, "/Envy Code R.ttf", 14)?;
        let text = graphics::Text::new(ctx, "@", &font)?;

        let h = Hero {
            font: font, 
            text: text, 
            pos: na::Point2::new(100.0, 100.0)
        };

        Ok(h)
    }

    fn change_pos(&mut self, d_x: f32, d_y: f32) {
        // self.pos.coords.x += d_x;
        // self.pos.coords.y += d_y;
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, ggez::graphics::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });  
        
        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        let _change_amount = 12.0;       

        // match keycode {
        //     Keycode::K | Keycode::Up | Keycode::Kp8 => { 
        //         self.change_pos(0.0, -change_amount)
        //     }
        //     Keycode::J | Keycode::Down | Keycode::Kp2 => { 
        //         self.change_pos(0.0, change_amount)
        //     }
        //     Keycode::H | Keycode::Left | Keycode::Kp4 => { 
        //         self.change_pos(-change_amount, 0.0)
        //     }
        //     Keycode::L | Keycode::Right | Keycode::Kp6 => { 
        //         self.change_pos(change_amount, 0.0)
        //     }
        //     Keycode::Y | Keycode::Kp7 => {
        //         self.change_pos(-change_amount, -change_amount)
        //     }
        //     Keycode::U | Keycode::Kp9 => {
        //         self.change_pos(change_amount, -change_amount)
        //     }
        //     Keycode::B | Keycode::Kp1 => {
        //         self.change_pos(-change_amount, change_amount)
        //     }
        //     Keycode::N | Keycode::Kp3 => {
        //         self.change_pos(change_amount, change_amount)
        //     }
        //     Keycode::Escape => { 
        //         ctx.quit().expect("Should quit")
        //     }
        //     _ => { println!("{:?}", keycode) }
        // };
    }
}

fn main() { 
    // let fullscreen_type = if cfg!(windows) {
    //     conf::FullscreenType::Off
    // } else {
    //     conf::FullscreenType::Desktop
    // };

    ggez::event::EventsLoop::get_available_monitors;
    
    let mut cb = ContextBuilder::new("adventure", "quantumtrip")
        .window_setup(conf::WindowSetup::default().title("Trit's Adventure"))
        .window_mode(conf::WindowMode::default()
            .dimensions(1920/2,1080/2)
            .borderless(false));
           // .fullscreen_type(fullscreen_type));

    let mut resource_dir = path::PathBuf::from(env::current_dir().unwrap());
    resource_dir.push("resources");
    cb = cb.add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build().unwrap();
    mouse::set_cursor_state(ctx, mouse::CursorState::Grab).unwrap();

    let state = &mut State::new(ctx).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}