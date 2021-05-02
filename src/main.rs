mod nes;
mod cpu {
    pub mod cpu;
    pub mod instructions;
    pub mod addressing;
    pub mod decode;
}
mod ppu {
    pub mod ppu;
    pub mod regcontrol;
    pub mod regmask;
    pub mod regstatus;
}
mod mappers {
    pub mod mapper;
    pub mod nrom;
}
mod tests {
    pub mod cputest;
    pub mod pputest;
}
mod events {
    pub mod drawevent;
}
mod cartridge;
mod buscpu;
mod busppu;
mod joypad;

extern crate image as im;
extern crate piston_window;
extern crate time;

use nes::Nes;
use piston_window::*;
use std::time::Instant;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

pub fn main() {

    let mut nes = Nes::new();
    nes.load(String::from("games/nestest.nes"));
    nes.reset();

    //bench(&mut nes);
    gui(&mut nes);
}

pub fn bench(nes: &mut Nes) {
    let now = Instant::now();
    for _ in 0..100000 {
        nes.clock();
    }
    println!("Time elapsed: {} us", now.elapsed().as_micros());
}

pub fn gui(nes: &mut Nes) {
    let mut window: PistonWindow = WindowSettings::new("NES EMULATOR", (WIDTH, HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_event_settings(EventSettings::new());

    let mut canvas = im::ImageBuffer::new(WIDTH, HEIGHT);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();
    

    while let Some(e) = window.next() {

        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A =>       nes.press_btn(joypad::Button::BTN_A),
                Key::S =>       nes.press_btn(joypad::Button::BTN_B),
                Key::Space =>   nes.press_btn(joypad::Button::SELECT),
                Key::Return =>  nes.press_btn(joypad::Button::START),
                Key::Up =>      nes.press_btn(joypad::Button::UP),
                Key::Down =>    nes.press_btn(joypad::Button::DOWN),
                Key::Left =>    nes.press_btn(joypad::Button::LEFT),
                Key::Right =>   nes.press_btn(joypad::Button::RIGHT),
                _ => {}
            }
        };

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::A =>       nes.release_btn(joypad::Button::BTN_A),
                Key::S =>       nes.release_btn(joypad::Button::BTN_B),
                Key::Space =>   nes.release_btn(joypad::Button::SELECT),
                Key::Return =>  nes.release_btn(joypad::Button::START),
                Key::Up =>      nes.release_btn(joypad::Button::UP),
                Key::Down =>    nes.release_btn(joypad::Button::DOWN),
                Key::Left =>    nes.release_btn(joypad::Button::LEFT),
                Key::Right =>   nes.release_btn(joypad::Button::RIGHT),
                _ => {}
            }
        };
        
        // Render
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {

                texture_context.encoder.flush(device);

                
                for evt in nes.get_draw_events() {
                    
                    let (r, g, b) = evt.rgb;
                    let (x, y) = evt.position;
                    canvas.put_pixel(x as u32, y as u32, im::Rgba([r, g, b, 255]));
                }
                //let start = PreciseTime::now();
                for _ in 0..10000 {
                    nes.clock();
                }
                //nes.clock();
                
                //let end = PreciseTime::now();
                //println!("\r{}", start.to(end).num_microseconds().unwrap());
                
                //clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }

}