use clap::Parser;

#[derive(Parser, Debug)] // 为Parameter结构体实现Parser和Debug trait
pub struct Parameter {
    #[arg(short, default_value_t = 30)] // fps帧率，默认30
    pub target_fps: u32,
    #[arg(short, default_value_t = 100.0)] // 镜头距离，默认100
    pub distance_from_cam: f32,
    #[arg(short, default_value_t = 40.0)]
    // 投影缩放因子，默认40，值越大物体显示越大，近大远小效果越强
    pub k1: f32,
    #[arg(short, default_value_t = 40.0)] // 方块大小，默认40
    pub cube_width: f32,
}
