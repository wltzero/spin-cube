

use std::io;
use std::io::Write;
use crate::structs::screen::ScreenSize;
use crate::structs::frame_buffer::FrameBuffer;

pub struct DoubleBufferedRenderer {
    pub front: FrameBuffer,
    pub back: FrameBuffer,
    pub screen_size: ScreenSize,
}

impl DoubleBufferedRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            front: FrameBuffer::new(width, height),
            back: FrameBuffer::new(width, height),
            screen_size: ScreenSize { width, height },
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }

    pub fn back_buffer(&mut self) -> &mut FrameBuffer {
        &mut self.back
    }

    pub fn render(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.front.render(stdout, &self.screen_size)
    }
}