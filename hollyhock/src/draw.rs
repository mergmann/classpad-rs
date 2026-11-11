use embedded_graphics_core::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::{IntoStorage, Rgb565},
    primitives::Rectangle,
};

use crate::sdk::SYMS;

pub const VRAM_WIDTH: u32 = 320;
pub const VRAM_HEIGHT: u32 = 528;

pub struct Vram {
    ptr: *mut u16,
}

impl Vram {
    pub fn get_vram() -> Self {
        Self {
            ptr: unsafe { (SYMS.LCD_GetVRAMAddress)() },
        }
    }

    #[inline]
    fn idx(&self, p: Point) -> usize {
        (p.y as u32 * VRAM_WIDTH + p.x as u32) as usize
    }

    pub fn show(&self) {
        unsafe { (SYMS.LCD_Refresh)() }
    }
}

impl Dimensions for Vram {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(VRAM_WIDTH, VRAM_HEIGHT))
    }
}

impl DrawTarget for Vram {
    type Color = Rgb565;
    type Error = !;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bbox = self.bounding_box();
        for Pixel(p, c) in pixels {
            if bbox.contains(p) {
                let i = self.idx(p);
                // Safety:
                //  p has been checked to be in bounds
                unsafe { self.ptr.add(i).write_volatile(c.into_storage()) };
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.intersection(&self.bounding_box());
        let Some(br) = area.bottom_right() else {
            return Ok(());
        };

        let c = color.into_storage();
        let w = area.size.width as usize;
        for y in area.top_left.y..=br.y {
            let start = self.idx(Point::new(area.top_left.x, y));
            for i in start..start + w {
                unsafe { self.ptr.add(i).write_volatile(c) };
            }
        }

        Ok(())
    }
}
