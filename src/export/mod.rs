use bevy::prelude::*;

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
fn handle_export_requests(mut export_events: EventReader<ExportRequest>) {
    for event in export_events.read() {
        match event.format {
            ExportFormat::PNG => {
                // 导出PNG图像
                println!("导出PNG: {}", event.filename);
            }
            ExportFormat::SVG => {
                // 导出SVG图像
                println!("导出SVG: {}", event.filename);
            }
            ExportFormat::GIF => {
                // 导出GIF动画
                println!("导出GIF: {}", event.filename);
            }
            ExportFormat::MP4 => {
                // 导出MP4视频
                println!("导出MP4: {}", event.filename);
            }
        }
    }
}
