use super::{MathObject, Position2D, Style};
use bevy::prelude::*;

pub struct AxesPlugin;

impl Plugin for AxesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Axes>()
            .register_type::<Grid>()
            .add_systems(Update, update_axes);
    }
}

/// 坐标轴组件
#[derive(Component, Reflect, Clone)]
pub struct Axes {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub show_numbers: bool,
    pub tick_spacing: f32,
    pub x_label: String,
    pub y_label: String,
    pub show_arrows: bool,
    pub base_range: (f32, f32), // 基础范围，用于缩放计算
}

impl Default for Axes {
    fn default() -> Self {
        Self {
            x_range: (-10.0, 10.0),
            y_range: (-10.0, 10.0),
            show_numbers: true,
            tick_spacing: 1.0,
            x_label: "x".to_string(),
            y_label: "y".to_string(),
            show_arrows: true,
            base_range: (20.0, 20.0), // 基础范围宽度
        }
    }
}

impl Axes {
    /// 根据缩放级别动态计算合适的刻度间距
    pub fn calculate_tick_spacing(&self, zoom: f32) -> f32 {
        let base_spacing = 1.0;
        let effective_range = self.base_range.0 / zoom;

        // 根据有效范围调整刻度间距
        if effective_range > 100.0 {
            base_spacing * 10.0
        } else if effective_range > 50.0 {
            base_spacing * 5.0
        } else if effective_range > 20.0 {
            base_spacing * 2.0
        } else if effective_range > 10.0 {
            base_spacing
        } else if effective_range > 5.0 {
            base_spacing * 0.5
        } else if effective_range > 2.0 {
            base_spacing * 0.2
        } else {
            base_spacing * 0.1
        }
    }

    /// 根据缩放级别更新坐标轴范围
    pub fn update_for_zoom(&mut self, zoom: f32) {
        let half_width = self.base_range.0 / (2.0 * zoom);
        let half_height = self.base_range.1 / (2.0 * zoom);

        self.x_range = (-half_width, half_width);
        self.y_range = (-half_height, half_height);
        self.tick_spacing = self.calculate_tick_spacing(zoom);
    }
}

/// 网格组件
#[derive(Component, Reflect, Clone)]
pub struct Grid {
    pub spacing: f32,
    pub opacity: f32,
    pub show_minor_grid: bool,
    pub minor_spacing: f32,
    pub base_spacing: f32, // 基础间距
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            spacing: 1.0,
            opacity: 0.3,
            show_minor_grid: true,
            minor_spacing: 0.2,
            base_spacing: 1.0,
        }
    }
}

impl Grid {
    /// 根据缩放级别更新网格间距
    pub fn update_for_zoom(&mut self, zoom: f32) {
        // 基础网格间距随缩放调整
        if zoom > 5.0 {
            self.spacing = self.base_spacing * 0.2;
            self.minor_spacing = self.spacing * 0.2;
        } else if zoom > 2.0 {
            self.spacing = self.base_spacing * 0.5;
            self.minor_spacing = self.spacing * 0.2;
        } else if zoom > 0.5 {
            self.spacing = self.base_spacing;
            self.minor_spacing = self.spacing * 0.2;
        } else if zoom > 0.2 {
            self.spacing = self.base_spacing * 2.0;
            self.minor_spacing = self.spacing * 0.2;
        } else {
            self.spacing = self.base_spacing * 5.0;
            self.minor_spacing = self.spacing * 0.2;
        }
    }
}

/// 创建坐标轴的便利函数
pub fn create_axes(
    commands: &mut Commands,
    x_range: (f32, f32),
    y_range: (f32, f32),
    style: Style,
) -> Entity {
    commands
        .spawn((
            MathObject {
                id: format!("axes_{}", rand::random::<u32>()),
                visible: true,
                layer: -1, // 坐标轴在底层
            },
            Axes {
                x_range,
                y_range,
                show_numbers: true,
                tick_spacing: 1.0,
                x_label: "x".to_string(),
                y_label: "y".to_string(),
                show_arrows: true,
                base_range: ((x_range.1 - x_range.0).abs(), (y_range.1 - y_range.0).abs()),
            },
            Position2D { x: 0.0, y: 0.0 },
            style,
            Transform::default(),
        ))
        .id()
}

/// 创建带自定义标签的坐标轴
pub fn create_axes_with_labels(
    commands: &mut Commands,
    x_range: (f32, f32),
    y_range: (f32, f32),
    x_label: String,
    y_label: String,
    style: Style,
) -> Entity {
    commands
        .spawn((
            MathObject {
                id: format!("axes_{}", rand::random::<u32>()),
                visible: true,
                layer: -1,
            },
            Axes {
                x_range,
                y_range,
                show_numbers: true,
                tick_spacing: 1.0,
                x_label,
                y_label,
                show_arrows: true,
                base_range: ((x_range.1 - x_range.0).abs(), (y_range.1 - y_range.0).abs()),
            },
            Position2D { x: 0.0, y: 0.0 },
            style,
            Transform::default(),
        ))
        .id()
}

/// 创建网格的便利函数
pub fn create_grid(commands: &mut Commands, spacing: f32, style: Style) -> Entity {
    commands
        .spawn((
            MathObject {
                id: format!("grid_{}", rand::random::<u32>()),
                visible: true,
                layer: -2, // 网格在最底层
            },
            Grid {
                spacing,
                opacity: 0.3,
                show_minor_grid: true,
                minor_spacing: spacing / 5.0,
                base_spacing: spacing,
            },
            Position2D { x: 0.0, y: 0.0 },
            style,
            Transform::default(),
        ))
        .id()
}

/// 更新坐标轴的系统
fn update_axes(mut query: Query<&mut Axes, Changed<Axes>>) {
    for mut axes in query.iter_mut() {
        // 这里可以添加坐标轴更新逻辑
        // 比如根据视图范围自动调整刻度间隔
        let x_span = axes.x_range.1 - axes.x_range.0;
        let y_span = axes.y_range.1 - axes.y_range.0;

        // 自动调整刻度间隔
        let max_span = x_span.max(y_span);
        axes.tick_spacing = if max_span > 50.0 {
            10.0
        } else if max_span > 20.0 {
            5.0
        } else if max_span > 10.0 {
            2.0
        } else {
            1.0
        };
    }
}
