use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub fn draw_to_screen(vram: [u8; 2048], canvas: &mut Canvas<Window>) {
    let pixel_width: u32 = 10;
    let pixel_height: u32 = 10;
    let left_margin: u32 = 10;
    let row_length: usize = 64;

    canvas.clear();

    for i in 0..vram.len() {
        canvas.set_draw_color(get_color(vram[i]));
        let x: u32 = (i % row_length) as u32;
        let y: u32 = (i / row_length) as u32;
        canvas.fill_rect(
            Rect::new(
                ((left_margin + x) + (pixel_width * x)) as i32,
                ((left_margin + y) + (pixel_height * y)) as i32,
                pixel_width,
                pixel_height
            )
        ).unwrap();
    }
    canvas.present();
}

fn get_color(pixel_bit: u8) -> Color {
    if pixel_bit == 1 {
        // TODO: Reverse me
        // Color::RGB(255, 255, 255)
        Color::RGB(0, 0, 0)
    } else {
        Color::RGB(255, 255, 255)
        // Color::RGB(0, 0, 0)
    }
}
