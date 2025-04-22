use crate::structs::screen::ScreenSize;
use crossterm::{cursor, ExecutableCommand};
use std::io;
use std::io::Write;

pub struct FrameBuffer {
    pub buffer: Vec<char>,
    pub z_buffer: Vec<f32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![' '; width * height],
            z_buffer: vec![0.0; width * height],
        }
    }

    pub fn clear(&mut self, clear_char: char) {
        self.buffer.fill(clear_char);
        self.z_buffer.fill(0.0);
    }

    pub fn render(&self, stdout: &mut impl Write, screen_size: &ScreenSize) -> io::Result<()> {
        stdout.execute(cursor::MoveTo(0, 0))?;
        for y in 0..screen_size.height {
            let line_start = y * screen_size.width;
            let line_end = line_start + screen_size.width;
            write!(
                stdout,
                "{}",
                &self.buffer[line_start..line_end].iter().collect::<String>()
            )?;
        }
        stdout.flush()?;
        Ok(())
    }
}
