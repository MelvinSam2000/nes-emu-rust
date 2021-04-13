mod nes;
mod cpu {
    pub mod cpu;
    pub mod instructions;
    pub mod addressing;
    pub mod decode;
}
mod mappers {
    pub mod mapper;
    pub mod nrom;
}
mod tests {
    pub mod cputest;
}
mod cartridge;
mod buscpu;
mod ppu;
mod busppu;

extern crate image as im;
extern crate piston_window;

use nes::Nes;
use piston_window::*;

const WIDTH: u32 = 255;
const HEIGHT: u32 = 240;

pub fn main() {

    let mut nes = Nes::new();
    nes.load(String::from("games/nestest.nes"));
    nes.reset();

    let mut window: PistonWindow = WindowSettings::new("NES EMULATOR", (WIDTH, HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_event_settings(EventSettings::new().max_fps(60));

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

                // Draw screen
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let (r, g, b) = nes.screen_pixel(y as u8, x as u8);
                        canvas.put_pixel(x, y, im::Rgba([r, g, b, 255]));
                    }
                }
                // Clock nes
                nes.clock();

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }

}