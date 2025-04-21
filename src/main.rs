use nalgebra::{Matrix3, Vector3};
use std::thread;
use std::time::Duration;

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

/// 使用矩阵优化的点计算
fn calculate_for_point_matrix(
    x: f32, y: f32, z: f32, ch: char,
    frame_buffer: &mut FrameBuffer,
    screen_size: &ScreenSize,
    camera_settings: &CameraSettings,
    rotation_matrix: &Matrix3<f32>
) {
    // 原始点向量
    let point = Vector3::new(x, y, z);

    // 应用旋转
    let rotated = rotation_matrix * point;

    // 计算透视投影
    let z_distance = rotated.z + camera_settings.distance_from_cam;
    let ooz = 1.0 / z_distance;

    // 计算屏幕坐标
    let xp = (screen_size.width as f32 / 2.0 + camera_settings.k1 * ooz * rotated.x * 2.0) as usize;
    let yp = (screen_size.height as f32 / 2.0 + camera_settings.k1 * ooz * rotated.y) as usize;
    let idx = xp + yp * screen_size.width;

    if idx < screen_size.width * screen_size.height {
        if ooz > frame_buffer.z_buffer[idx] {
            frame_buffer.z_buffer[idx] = ooz;
            frame_buffer.buffer[idx] = ch;
        }
    }
}

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;
    let cube_width = 40.0;
    let screen_size = ScreenSize { width: 100, height: 44 };
    let mut frame_buffer = FrameBuffer {
        buffer: vec![' '; screen_size.width * screen_size.height],
        z_buffer: vec![0.0; screen_size.width * screen_size.height],
    };
    let background_ascii_code = ' ';
    let camera_settings = CameraSettings {
        distance_from_cam: 100.0,
        k1: 40.0,
    };

    print!("\x1b[2J");
    loop {
        frame_buffer.buffer.fill(background_ascii_code);
        frame_buffer.z_buffer.fill(0.0);

        // 预计算旋转矩阵
        let rotation_matrix = calculate_rotation_matrix(a, b, c);

        let mut i = -cube_width / 2.0;
        while i < cube_width / 2.0 {
            let mut j = -cube_width / 2.0;
            while j < cube_width / 2.0 {
                // 使用矩阵计算优化后的版本
                calculate_for_point_matrix(
                    i, j, -cube_width / 2.0, '@',
                    &mut frame_buffer, &screen_size, &camera_settings,
                    &rotation_matrix
                );
                calculate_for_point_matrix(
                    cube_width / 2.0, j, i, '$',
                    &mut frame_buffer, &screen_size, &camera_settings,
                    &rotation_matrix
                );
                calculate_for_point_matrix(
                    -cube_width / 2.0, j, -i, '~',
                    &mut frame_buffer, &screen_size, &camera_settings,
                    &rotation_matrix
                );
                calculate_for_point_matrix(
                    -i, j, cube_width / 2.0, '#',
                    &mut frame_buffer, &screen_size, &camera_settings,
                    &rotation_matrix
                );
                calculate_for_point_matrix(
                    i, -cube_width / 2.0, -j, ';',
                    &mut frame_buffer, &screen_size, &camera_settings,
                    &rotation_matrix
                );
                calculate_for_point_matrix(
                    i, cube_width / 2.0, j, '+',
                    &mut frame_buffer, &screen_size, &camera_settings,
                    &rotation_matrix
                );
                j += 0.15;
            }
            i += 0.15;
        }

        print!("\x1b[H");
        for k in 0..screen_size.width * screen_size.height {
            if k % screen_size.width == 0 {
                println!();
            } else {
                print!("{}", frame_buffer.buffer[k]);
            }
        }

        a += 0.05;
        b += 0.05;
        c += 0.01;
        thread::sleep(Duration::from_micros(80000));
    }
}

/// 计算3D旋转矩阵
fn calculate_rotation_matrix(a: f32, b: f32, c: f32) -> Matrix3<f32> {
    // 绕X轴旋转
    let rx = Matrix3::new(
        1.0, 0.0, 0.0,
        0.0, a.cos(), -a.sin(),
        0.0, a.sin(), a.cos()
    );

    // 绕Y轴旋转
    let ry = Matrix3::new(
        b.cos(), 0.0, b.sin(),
        0.0, 1.0, 0.0,
        -b.sin(), 0.0, b.cos()
    );

    // 绕Z轴旋转
    let rz = Matrix3::new(
        c.cos(), -c.sin(), 0.0,
        c.sin(), c.cos(), 0.0,
        0.0, 0.0, 1.0
    );

    // 组合旋转 (顺序: Z -> Y -> X)
    rz * ry * rx
}
