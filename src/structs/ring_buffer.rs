use std::io;
use std::io::Write;
use crate::structs::screen::ScreenSize;
use crate::structs::frame_buffer::FrameBuffer;
use ringbuffer::{RingBuffer,AllocRingBuffer};

pub struct RingBufferedRenderer {
    buffers: AllocRingBuffer<FrameBuffer>,
    screen_size: ScreenSize,
    current_index: usize,
}

impl RingBufferedRenderer {
    pub fn new(width: usize, height: usize, buffer_count: usize) -> Self {
        let mut buffers = AllocRingBuffer::new(buffer_count);
        for _ in 0..buffer_count {
            buffers.push(FrameBuffer::new(width, height));
        }

        Self {
            buffers,
            screen_size: ScreenSize { width, height },
            current_index: 0,
        }
    }

    pub fn current_buffer(&mut self) -> &mut FrameBuffer {
        self.buffers.get_mut(self.current_index).unwrap()
    }

    pub fn next_buffer(&mut self) {
        self.current_index += 1;
    }

    pub fn render(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.buffers.get(self.current_index).unwrap().render(stdout, &self.screen_size)
    }
}