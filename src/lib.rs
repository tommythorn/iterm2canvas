//! # The Terminal inline picture library
//!
//! iTerm2 (and others) support displaying pictures inline in the
//! terminal.  This crate implements a very simple canvas `Pict` that
//! can be dumped directly in the iTerm2 console with
//! `dump_iterm2_image`.

use base64::prelude::*;

/// `Pict` is a simple rectangular canvas of 32-bit RGB pixel values
pub struct Pict {
    width: usize,
    height: usize,
    pixel: Vec<u32>,
}

impl Pict {
    /// Constructs a new canvas of `width` x `height` black (0)
    /// pixels.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixel: vec![0; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Plots a pixel of color `c` at location (`x`, `y`).
    /// # Panics
    /// Will panic if x or y is outsides the width/height bounds.
    pub fn plot(&mut self, x: usize, y: usize, c: u32) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixel[y * self.width + x] = c;
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
}
