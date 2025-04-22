mod structs;
mod utils;

use crate::structs::parameter::*;
use clap::Parser;
use crossterm::{
    cursor, event::{poll, read, Event, KeyCode},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::Write;
use std::{io, time::Duration};
use structs::ring_buffer::*;
use structs::frame_stat::*;
use structs::screen::*;
use utils::handler::*;

fn main() -> io::Result<()> {
    let params = Parameter::parse(); // 解析命令行参数
    
    let mut stdout = io::stdout();
    // 初始化终端
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(ClearType::All))?;

    // 获取终端实际尺寸
    let (mut term_width, mut term_height) = terminal::size()?;
    let mut screen_size = ScreenSize {
        width: term_width as usize,
        height: (term_height as usize).saturating_sub(1),
    };

    let cube_width = params.cube_width;
    let camera_settings = CameraSettings {
        distance_from_cam: params.distance_from_cam,
        k1: params.k1,
    };

    // 使用环形缓冲区
    let mut renderer = RingBufferedRenderer::new(screen_size.width, screen_size.height, 3);
    let mut angles = (0.0, 0.0, 0.0);
    let mut frame_stats = FrameStats::new(params.target_fps);

    loop {
        // 检查终端尺寸是否变化
        let (new_width, new_height) = terminal::size()?;
        if new_width != term_width || new_height != term_height {
            term_width = new_width;
            term_height = new_height;
            screen_size = ScreenSize {
                width: term_width as usize,
                height: (term_height as usize).saturating_sub(1),
            };
            renderer = RingBufferedRenderer::new(screen_size.width, screen_size.height, 10);
        }

        frame_stats.begin_frame();

        // 清空当前缓冲区
        renderer.current_buffer().clear(' ');

        // 计算旋转矩阵
        let rotation_matrix = calculate_rotation_matrix(angles.0, angles.1, angles.2);

        // 在当前缓冲区绘制立方体
        draw_cube(
            renderer.current_buffer(),
            &screen_size,
            &camera_settings,
            &rotation_matrix,
            cube_width
        );

        // 切换到下一个缓冲区
        renderer.next_buffer();

        // 渲染当前缓冲区到终端
        stdout.execute(cursor::MoveTo(0, 0))?;
        renderer.render(&mut stdout)?;

        // 显示帮助信息和帧率
        let frame_time = frame_stats.end_frame();
        write!(
            stdout,
            "FPS: {:3} | Frame: {:3.1}ms | Size: {}x{} | Angles: {:.1},{:.1},{:.1} | Press 'q' to quit",
            frame_stats.fps,
            frame_time.as_secs_f32() * 1000.0,
            screen_size.width,
            screen_size.height,
            angles.0, angles.1, angles.2
        )?;
        stdout.flush()?;

        // 更新旋转角度
        angles.0 += 0.05;
        angles.1 += 0.05;
        angles.2 += 0.01;

        // 处理退出
        if poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // 恢复终端
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}