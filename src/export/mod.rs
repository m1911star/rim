use bevy::prelude::*;
use bevy::render::view::window::screenshot::{save_to_disk, Screenshot};
use bevy::window::PrimaryWindow;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ExportPlugin;

impl Plugin for ExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExportRequest>()
            .add_systems(Update, handle_export_requests);
    }
}

/// 导出格式枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    PNG,
    SVG,
    GIF,
    MP4,
}

/// 导出请求事件
#[derive(Event)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub filename: String,
    pub resolution: (u32, u32),
}

/// 处理导出请求的系统
fn handle_export_requests(mut export_events: EventReader<ExportRequest>, mut commands: Commands) {
    for event in export_events.read() {
        match event.format {
            ExportFormat::PNG => {
                // 使用新的截图API
                let path = format!("screenshots/{}", event.filename);

                // 确保screenshots目录存在
                if let Some(parent) = Path::new(&path).parent() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        error!("Failed to create screenshots directory: {}", e);
                        continue;
                    }
                }

                // 使用新的截图API
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(path.clone()));

                info!("Screenshot requested: {}", path);
            }
            ExportFormat::SVG => {
                // 导出SVG图像 (暂未实现)
                warn!("SVG export not yet implemented: {}", event.filename);
            }
            ExportFormat::GIF => {
                // 导出GIF动画 (暂未实现)
                warn!("GIF export not yet implemented: {}", event.filename);
            }
            ExportFormat::MP4 => {
                // 导出MP4视频 (暂未实现)
                warn!("MP4 export not yet implemented: {}", event.filename);
            }
        }
    }
}

/// 便利函数：请求PNG截图
pub fn request_png_screenshot(
    export_writer: &mut EventWriter<ExportRequest>,
    filename: Option<String>,
) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = filename.unwrap_or_else(|| format!("screenshot_{}.png", timestamp));

    export_writer.write(ExportRequest {
        format: ExportFormat::PNG,
        filename,
        resolution: (1920, 1080), // 默认分辨率
    });
}
