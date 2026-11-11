#![no_std]
#![no_main]

use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::{Rgb565, RgbColor, WebColors},
    primitives::{Circle, Primitive, PrimitiveStyle, Triangle},
    text::{Alignment, Text},
};
use hollyhock::{
    debug::wait_key,
    define_app_cargo,
    draw::Vram,
    env::vars,
    fs::{File, FugueError},
    io::bufread::{BufReadExt, BufReader},
    prelude::*,
};

define_app_cargo!();

fn main_draw() {
    let mut vram = Vram::get_vram();

    let bbox = vram.bounding_box();

    vram.bounding_box()
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::CSS_GRAY, 16))
        .draw(&mut vram);

    vram.clear(Rgb565::WHITE);

    let (cx, cy) = (160, 100);
    Triangle::new(
        Point::new(cx - 64, cy - 64),
        Point::new(cx + 64, cy - 64),
        Point::new(cx, cy + 64),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 4))
    .draw(&mut vram);

    Circle::new(bbox.center() - Point::new(32, 32), 64)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
        .draw(&mut vram);

    let (cx, cy) = (160, 428);
    Triangle::new(
        Point::new(cx - 64, cy + 64),
        Point::new(cx + 64, cy + 64),
        Point::new(cx, cy - 64),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 4))
    .draw(&mut vram);

    Text::with_alignment(
        "Hello World!",
        Point::new(160, 346),
        MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_PURPLE),
        Alignment::Center,
    )
    .draw(&mut vram);

    vram.show();
}

fn main_read() -> Result<(), FugueError> {
    let mut file = BufReader::new(File::open(c"\\fls0\\example.txt")?);
    for line in file.lines() {
        let line = line?;
        println!("line={line}");
    }
    Ok(())
}

#[hollyhock::main]
fn main() {
    println!("Hello from Rust!");
    for (name, var) in vars() {
        println!("{name} = {var}");
    }

    if let Err(err) = main_read() {
        eprintln!("Error: {err}");
    }
    wait_key();

    main_draw();
    wait_key();
}
