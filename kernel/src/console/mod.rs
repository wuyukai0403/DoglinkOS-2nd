mod framebuffer;

use limine::framebuffer::Framebuffer as LimineFrameBuffer;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight};

struct Console {
    width: u64,
    height: u64,
    cursor_x: u64,
    cursor_y: u64,
}

impl Console {
    pub const fn null() -> Console {
        Console {
            width: 0,
            height: 0,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}

pub static mut console: Console = Console::null();

pub fn init(fb: &LimineFrameBuffer) {
    framebuffer::init(fb);
    unsafe {
        console.width = fb.width() / 7;
        console.height = fb.height() / 16;
    }
}

pub fn clear() {
    framebuffer::display_fill(0, 0, 0);
    unsafe {
        console.cursor_x = 0;
        console.cursor_y = 0;
    }
}


pub fn setchar(x: u64, y: u64, c: char) {
    let rc = get_raster(c, FontWeight::Regular, RasterHeight::Size16).unwrap();
    for (i, di) in rc.raster().iter().enumerate() {
        for (j, dj) in di.iter().enumerate() {
            framebuffer::display_setpixel(x * 16 + i as u64, y * 7 + j as u64, *dj, *dj, *dj);
        }
    }
}

pub fn newline() {
    // TODO: roll
    unsafe {
        console.cursor_x += 1;
        console.cursor_y = 0;
    }
}

pub fn inc() {
    unsafe {
        console.cursor_y += 1;
        if console.cursor_y == console.width {
            newline();
        }
    }
}

pub fn putchar(c: char) {
    match c {
        '\n' => newline(),
        oc => {
            unsafe {
                setchar(console.cursor_x, console.cursor_y, oc);
            }
            inc();
        }
    }
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putchar(c);
    }
}
