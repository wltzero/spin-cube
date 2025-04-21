use nalgebra::{Matrix3, Vector3};
use std::{io, thread, time::Duration};
use std::io::Write;
use crossterm::{
    cursor, terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    event::{poll, read, Event, KeyCode},
};

struct FrameBuffer {
    buffer: Vec<char>,
    z_buffer: Vec<f32>,
}

struct ScreenSize {
    width: usize,
    height: usize,
}

struct CameraSettings {
    distance_from_cam: f32,
    k1: f32,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![' '; width * height],
            z_buffer: vec![0.0; width * height],
        }
    }

    fn clear(&mut self, clear_char: char) {
        self.buffer.fill(clear_char);
        self.z_buffer.fill(0.0);
    }

    fn render(&self, stdout: &mut impl Write, screen_size: &ScreenSize) -> io::Result<()> {
        stdout.execute(cursor::MoveTo(0, 0))?;
        let mut output = String::with_capacity(screen_size.width * screen_size.height + screen_size.height);

        for y in 0..screen_size.height {
            let line_start = y * screen_size.width;
            let line_end = line_start + screen_size.width;
            output.push_str(&self.buffer[line_start..line_end].iter().collect::<String>());
        }

        write!(stdout, "{}", output)?;
        stdout.flush()?;
        Ok(())
    }
}

fn calculate_rotation_matrix(a: f32, b: f32, c: f32) -> Matrix3<f32> {
    let rx = Matrix3::new(
        1.0, 0.0, 0.0,
        0.0, a.cos(), -a.sin(),
        0.0, a.sin(), a.cos()
    );

    let ry = Matrix3::new(
        b.cos(), 0.0, b.sin(),
        0.0, 1.0, 0.0,
        -b.sin(), 0.0, b.cos()
    );

    let rz = Matrix3::new(
        c.cos(), -c.sin(), 0.0,
        c.sin(), c.cos(), 0.0,
        0.0, 0.0, 1.0
    );

    rz * ry * rx
}

fn calculate_for_point(
    x: f32, y: f32, z: f32, ch: char,
    frame_buffer: &mut FrameBuffer,
    screen_size: &ScreenSize,
    camera_settings: &CameraSettings,
    rotation_matrix: &Matrix3<f32>
) {
    let point = Vector3::new(x, y, z);
    let rotated = rotation_matrix * point;
    let z_distance = rotated.z + camera_settings.distance_from_cam;
    let ooz = 1.0 / z_distance;

    let xp = (screen_size.width as f32 / 2.0 + camera_settings.k1 * ooz * rotated.x * 2.0) as usize;
    let yp = (screen_size.height as f32 / 2.0 + camera_settings.k1 * ooz * rotated.y) as usize;

    if let Some(idx) = frame_buffer.buffer.get_mut(xp + yp * screen_size.width) {
        if ooz > frame_buffer.z_buffer[xp + yp * screen_size.width] {
            frame_buffer.z_buffer[xp + yp * screen_size.width] = ooz;
            *idx = ch;
        }
    }
}

fn draw_cube(
    frame_buffer: &mut FrameBuffer,
    screen_size: &ScreenSize,
    camera_settings: &CameraSettings,
    rotation_matrix: &Matrix3<f32>,
    cube_width: f32
) {
    let step = 0.15;
    let half_width = cube_width / 2.0;

    let mut i = -half_width;
    while i < half_width {
        let mut j = -half_width;
        while j < half_width {
            calculate_for_point(i, j, -half_width, '@', frame_buffer, screen_size, camera_settings, rotation_matrix);
            calculate_for_point(half_width, j, i, '$', frame_buffer, screen_size, camera_settings, rotation_matrix);
            calculate_for_point(-half_width, j, -i, '~', frame_buffer, screen_size, camera_settings, rotation_matrix);
            calculate_for_point(-i, j, half_width, '#', frame_buffer, screen_size, camera_settings, rotation_matrix);
            calculate_for_point(i, -half_width, -j, ';', frame_buffer, screen_size, camera_settings, rotation_matrix);
            calculate_for_point(i, half_width, j, '+', frame_buffer, screen_size, camera_settings, rotation_matrix);
            j += step;
        }
        i += step;
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // 初始化终端
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(ClearType::All))?; // 初始清屏一次

    // 获取终端实际尺寸
    let (term_width, term_height) = terminal::size()?;
    let screen_size = ScreenSize {
        width: term_width as usize,
        height: (term_height as usize).saturating_sub(1), // 留一行给可能的提示信息
    };

    let cube_width = 40.0;
    let mut frame_buffer = FrameBuffer::new(screen_size.width, screen_size.height);
    let camera_settings = CameraSettings {
        distance_from_cam: 100.0,
        k1: 40.0,
    };

    let mut angles = (0.0, 0.0, 0.0); // (a, b, c)

    loop {
        // 重置帧缓冲区
        frame_buffer.clear(' ');

        // 计算旋转矩阵
        let rotation_matrix = calculate_rotation_matrix(angles.0, angles.1, angles.2);

        // 绘制立方体
        draw_cube(&mut frame_buffer, &screen_size, &camera_settings, &rotation_matrix, cube_width);

        // 渲染到终端
        stdout.execute(cursor::MoveTo(0, 0))?; // 只移动光标，不清屏

        // 渲染帧缓冲区内容
        frame_buffer.render(&mut stdout, &screen_size)?;

        // 在最后一行显示帮助信息
        write!(
            stdout,
            "Press 'q' to quit | Size: {}x{} | Angles: {:.1},{:.1},{:.1}",
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

        thread::sleep(Duration::from_micros(80000));
    }

    // 恢复终端
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}