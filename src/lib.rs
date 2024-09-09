//! # The Terminal inline picture library
//!
//! iTerm2 (and others) support displaying pictures inline in the
//! terminal.  This crate implements a very simple canvas `Pict` that
//! can be dumped directly in the iTerm2 console with
//! `dump_iterm2_image`.

use base64::prelude::*;

/// `Pict` is a simple rectangular canvas of 32-bit RGB pixel values
pub struct Pict {
    width: isize,
    height: isize,
    pixel: Vec<u32>,
}

impl Pict {
    /// Constructs a new canvas of `width` x `height` black (0)
    /// pixels.
    pub fn new(width: isize, height: isize) -> Self {
        assert!(0 <= width);
        assert!(0 <= height);
        Self {
            width,
            height,
            pixel: vec![0; (width * height) as usize],
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    /// Plots a pixel of color `c` at location (`x`, `y`).
    /// # Panics
    /// Will panic if x or y is outsides the width/height bounds.
    pub fn plot(&mut self, x: isize, y: isize, c: u32) {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.pixel[(y * self.width + x) as usize] = c;
        }
    }

    /// Display the canvas directly inline in the iTerm2 terminal,
    /// with an optional scaling of the terminal width, eg. calling
    /// with `Some(100)` will scale the canvas when printed to fill
    /// 100% of the width of the terminal.
    pub fn dump_iterm2_image(&self, scale: Option<usize>) {
        let mut buf = vec![];
        let header: String = format!("P6\n{} {}\n255\n", self.width, self.height);
        buf.extend_from_slice(header.as_bytes());

        for c in &self.pixel {
            buf.extend([(c >> 16) as u8, (c >> 8) as u8, *c as u8]);
        }

        let scale_str: String = match scale {
            None => "".into(),
            Some(f) => format!(";width={f}%"),
        };

        println!(
            "]1337;File=inline=1{scale_str}:{}{}",
            BASE64_STANDARD.encode(buf.as_slice()),
            7 as char
        );
    }

    pub fn draw_line(&mut self, p0: (isize, isize), p1: (isize, isize), c: u32) {
        let xd = p1.0 - p0.0;
        let xs = xd.signum();
        let xd = xd.abs();
        let yd = p1.1 - p0.1;
        let ys = yd.signum();
        let yd = yd.abs();

        if yd < xd {
            // Slope is yd/xd so whenever x * yd/xd crosses integer
            // boundy we should advance y

            let mut err = yd / 2;
            let (mut x, mut y) = (p0.0, p0.1);

            for _ in 0..xd {
                self.plot(x, y, c);
                x += xs;
                err += yd;
                if xd < err {
                    y += ys;
                    err -= xd;
                }
            }
        } else {
            // Slope is xd/yd so whenever y * xd/yd crosses integer
            // boundx we should advance x

            let mut err = xd / 2;
            let (mut x, mut y) = (p0.0, p0.1);

            for _ in 0..yd {
                self.plot(x, y, c);
                y += ys;
                err += xd;
                if yd < err {
                    x += xs;
                    err -= yd;
                }
            }
        }
    }
}

#[test]
fn test() {
    use Pict;

    let mut pict = Pict::new(100, 100);
    for x in 0..100 {
        for y in 0..100 {
            pict.plot(x, y, 0xFFFF00);
        }
    }

    for x in 0..10 {
        pict.draw_line((x * 10, 0), (0, 100), 0);
        pict.draw_line((0, x * 10), (100, 0), 0);
        pict.draw_line((x * 10, 0), (100, 100), 0);
        pict.draw_line((0, x * 10), (0, 0), 0);
    }
    pict.dump_iterm2_image(Some(25));
    //panic!("This is what it should look like");
}
