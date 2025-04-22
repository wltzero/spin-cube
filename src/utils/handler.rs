use nalgebra::{Matrix3, Vector3};
use crate::structs::frame_buffer::FrameBuffer;
use crate::structs::screen::{CameraSettings, ScreenSize};

pub fn calculate_for_point(
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

    // 添加边界检查
    if xp < screen_size.width && yp < screen_size.height {
        if let Some(idx) = frame_buffer.buffer.get_mut(xp + yp * screen_size.width) {
            if ooz > frame_buffer.z_buffer[xp + yp * screen_size.width] {
                frame_buffer.z_buffer[xp + yp * screen_size.width] = ooz;
                *idx = ch;
            }
        }
    }
}
pub fn calculate_rotation_matrix(a: f32, b: f32, c: f32) -> Matrix3<f32> {
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
pub fn draw_cube(
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