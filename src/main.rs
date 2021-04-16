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
    pub mod regaddrdata;
    pub mod regloopy;
}
mod mappers {
    pub mod mapper;
    pub mod nrom;
}
mod tests {
    pub mod cputest;
}
mod events {
    pub mod drawevent;
}
mod cartridge;
mod buscpu;
mod busppu;

extern crate image as im;
extern crate piston_window;
extern crate time;

use nes::Nes;
use piston_window::*;
use events::drawevent::DrawEvent;
use time::PreciseTime;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

pub fn main() {

    let mut nes = Nes::new();
    nes.load(String::from("games/nestest.nes"));
    nes.reset();

    let mut window: PistonWindow = WindowSettings::new("NES EMULATOR", (WIDTH, HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_event_settings(EventSettings::new().bench_mode(true));

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
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {

                texture_context.encoder.flush(device);

                
                for evt in nes.get_draw_events() {
                    let (r, g, b) = evt.rgb;
                    let (y, x) = evt.position;
                    canvas.put_pixel(x as u32, y as u32, im::Rgba([r, g, b, 255]));
                }
                //let start = PreciseTime::now();
                for _ in 0..10000 {
                    nes.clock();
                }
                
                
                //let end = PreciseTime::now();
                //println!("\r{}", start.to(end).num_microseconds().unwrap());
                
                //clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }

}