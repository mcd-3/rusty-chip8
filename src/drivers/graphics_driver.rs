use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct GraphicsDriver {
    vram: [u8; 2048],
    canvas: Canvas<Window>
}

impl GraphicsDriver {
    pub fn new(vram: [u8; 2048], canvas: Canvas<Window>) -> Result<Self, &'static str>{
        Ok(GraphicsDriver {
            vram,
            canvas
        })
    }

    /// Update the data in the vram
    pub fn update_vram(&mut self, vram: [u8; 2048]) {
        self.vram = vram
    }

    /// Draw the vram data to a canvas
    pub fn draw_to_screen(&mut self) {
        // vram: [u8; 2048], canvas: &mut Canvas<Window>
        let pixel_width: u32 = 10;
        let pixel_height: u32 = 10;
        let left_margin: u32 = 11;
        let row_length: usize = 64;
    
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    
        for i in 0..self.vram.len() {
            self.canvas.set_draw_color(self.get_color(self.vram[i]));
            let x: u32 = (i % row_length) as u32;
            let y: u32 = (i / row_length) as u32;
            self.canvas.fill_rect(
                Rect::new(
                    ((left_margin + x) + (pixel_width * x)) as i32,
                    ((left_margin + y) + (pixel_height * y)) as i32,
                    pixel_width,
                    pixel_height
                )
            ).unwrap();
        }
        self.canvas.present();
    }
    
    /// Get the color of the bit
    /// White if on, dark grey if off
    fn get_color(&self, pixel_bit: u8) -> Color {
        if pixel_bit == 1 {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(33, 33, 33)
        }
    }
}
