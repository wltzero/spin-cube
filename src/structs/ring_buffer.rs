use std::io;
use std::io::Write;
use crate::structs::screen::ScreenSize;
use crate::structs::frame_buffer::FrameBuffer;

pub struct RingBufferedRenderer {
    buffers: Vec<FrameBuffer>,
    current_index: usize,
    screen_size: ScreenSize,
}

impl RingBufferedRenderer {
    pub fn new(width: usize, height: usize, buffer_count: usize) -> Self {
        let buffers = (0..buffer_count)
            .map(|_| FrameBuffer::new(width, height))
            .collect();

        Self {
            buffers,
            current_index: 0,
            screen_size: ScreenSize { width, height },
        }
    }

    pub fn current_buffer(&mut self) -> &mut FrameBuffer {
        &mut self.buffers[self.current_index]
    }

    pub fn next_buffer(&mut self) {
        self.current_index = (self.current_index + 1) % self.buffers.len();
    }

    pub fn render(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.buffers[self.current_index].render(stdout, &self.screen_size)
    }
}