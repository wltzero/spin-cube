use std::thread;
use std::time::{Duration, Instant};

pub struct FrameStats {
    pub last_frame_time: Instant,
    pub frame_count: u32,
    pub fps: u32,
    pub last_fps_update: Instant,
    pub target_frame_time: Duration,
}
impl FrameStats {
    pub fn new(target_fps: u32) -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps: 0,
            last_fps_update: Instant::now(),
            target_frame_time: Duration::from_secs_f32(1.0 / target_fps as f32),
        }
    }

    pub fn begin_frame(&mut self) {
        self.last_frame_time = Instant::now();
    }

    pub fn end_frame(&mut self) -> Duration {
        let elapsed = self.last_frame_time.elapsed();

        // FPS计算
        self.frame_count += 1;
        if self.last_fps_update.elapsed().as_secs_f32() >= 1.0 {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
        }

        // 帧率控制
        if elapsed < self.target_frame_time {
            let sleep_time = self.target_frame_time - elapsed;
            thread::sleep(sleep_time);
            self.target_frame_time
        } else {
            elapsed
        }
    }
}
