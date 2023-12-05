use ctru::prelude::*;
use ctru::services::gfx::{self, Flush, Screen, Swap};
use ctru::services::gspgpu::FramebufferFormat;
use embedded_graphics::pixelcolor;
use embedded_graphics::prelude::*;
use std::cell::RefMut;
use std::convert::Infallible;
use std::marker::PhantomData;

pub struct CtrDisplay<'gfx, S: Screen + Swap, const WIDE: bool> {
    display: RefMut<'gfx, S>,
    gfx: PhantomData<&'gfx Gfx>,
}

pub type TopDisplayWide<'gfx> = CtrDisplay<'gfx, gfx::TopScreen, true>;
pub type TopDisplay<'gfx> = CtrDisplay<'gfx, gfx::TopScreen, false>;

impl<'gfx, const WIDE: bool> CtrDisplay<'gfx, gfx::TopScreen, WIDE> {
    fn new_generic(gfx: &'gfx Gfx) -> Self {
        let mut display = gfx.top_screen.borrow_mut();
        display.set_wide_mode(WIDE);
        display.set_double_buffering(false);
        display.set_framebuffer_format(FramebufferFormat::Bgr8);
        display.swap_buffers();
        Self {
            display,
            gfx: PhantomData,
        }
    }
}

impl<'gfx> TopDisplayWide<'gfx> {
    pub fn new(gfx: &'gfx Gfx) -> TopDisplayWide<'gfx> {
        CtrDisplay::new_generic(gfx)
    }
    pub fn into_nonwide(mut self) -> TopDisplay<'gfx> {
        self.display.set_wide_mode(false);
        self.display.swap_buffers();
        TopDisplay {
            display: self.display,
            gfx: self.gfx,
        }
    }
}

impl<'gfx> TopDisplay<'gfx> {
    pub fn new(gfx: &'gfx Gfx) -> TopDisplay<'gfx> {
        CtrDisplay::new_generic(gfx)
    }
    pub fn into_wide(mut self) -> TopDisplayWide<'gfx> {
        self.display.set_wide_mode(true);
        self.display.swap_buffers();
        TopDisplayWide {
            display: self.display,
            gfx: self.gfx,
        }
    }
}

impl<'gfx> OriginDimensions for TopDisplayWide<'gfx> {
    fn size(&self) -> Size {
        Size::new(800, 240)
    }
}

impl<'gfx> OriginDimensions for TopDisplay<'gfx> {
    fn size(&self) -> Size {
        Size::new(400, 240)
    }
}

#[inline(always)]
fn pixel_idx(x: i32, y: i32, width: usize, _height: usize) -> i32 {
    (x + (y * width as i32)) * 3
}

#[inline(always)]
fn rotate_90(x: i32, y: i32, width: usize, _height: usize) -> (i32, i32) {
    (width as i32 - y - 1, x)
}

impl<'gfx, S: gfx::Screen + gfx::Swap, const WIDE: bool> DrawTarget for CtrDisplay<'gfx, S, WIDE>
where
    CtrDisplay<'gfx, S, WIDE>: embedded_graphics::geometry::OriginDimensions,
{
    type Color = pixelcolor::Bgr888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let raw = self.display.raw_framebuffer();

        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x < 0 || coord.y < 0 {
                continue;
            }

            if coord.x > raw.height as i32 || coord.y > raw.width as i32 {
                continue;
            }

            unsafe {
                let (x, y) = rotate_90(coord.x, coord.y, raw.width, raw.height);
                let idx: isize = pixel_idx(x, y, raw.width, raw.height) as isize;
                raw.ptr.offset(idx).write(color.b());
                raw.ptr.offset(idx+1).write(color.g());
                raw.ptr.offset(idx+2).write(color.r());
            }
        }
        self.display.flush_buffers();
        Ok(())
    }
}
