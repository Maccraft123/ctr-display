use ctr_display::TopDisplay;

use ctru::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::mono_font::{ascii::{FONT_6X9, FONT_10X20}, MonoTextStyleBuilder, MonoTextStyle};
use embedded_graphics::pixelcolor::Bgr888;
use embedded_graphics::primitives::{Circle, Rectangle, PrimitiveStyle};

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let mut display = TopDisplay::new(&gfx);

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Bgr888::new(255, 255, 255))
        .background_color(Bgr888::new(0, 0, 0))
        .build();

    Text::new(
        "\nThis is a\nmultiline\nHello World text\n\n\nPress Start to exit",
        Point::new(1, 1),
        style,
    )
    .draw(&mut display).unwrap();

    Rectangle::new(Point::new(0, 0), Size::new(400, 240))
        .into_styled(PrimitiveStyle::with_stroke(Bgr888::RED, 1))
        .draw(&mut display).unwrap();

    Circle::new(Point::new(50, 120), 40)
        .into_styled(PrimitiveStyle::with_stroke(Bgr888::YELLOW, 5))
        .draw(&mut display).unwrap();

    let colors = [
        Bgr888::new(0, 0, 0),
        Bgr888::new(163, 163, 163),
        Bgr888::new(255, 255, 255),
        Bgr888::new(128, 0, 128),
    ];
    
    Rectangle::new(Point::new(200, 10), Size::new(180, 40))
        .into_styled(PrimitiveStyle::with_fill(colors[0]))
        .draw(&mut display).unwrap();

    Rectangle::new(Point::new(200, 50), Size::new(180, 40))
        .into_styled(PrimitiveStyle::with_fill(colors[1]))
        .draw(&mut display).unwrap();
    
    Rectangle::new(Point::new(200, 90), Size::new(180, 40))
        .into_styled(PrimitiveStyle::with_fill(colors[2]))
        .draw(&mut display).unwrap();

    Rectangle::new(Point::new(200, 130), Size::new(180, 40))
        .into_styled(PrimitiveStyle::with_fill(colors[3]))
        .draw(&mut display).unwrap();

    Rectangle::new(Point::new(200, 10), Size::new(180, 40*4))
        .into_styled(PrimitiveStyle::with_stroke(Bgr888::WHITE, 2))
        .draw(&mut display).unwrap();

    Text::new("UwU :3", Point::new(200, 200),
        MonoTextStyle::new(&FONT_10X20, Bgr888::WHITE))
        .draw(&mut display).unwrap();


    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }

}
