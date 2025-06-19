/*
 * RIM - Mathematical Visualization Tool
 * Copyright (C) 2024 m1911star
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};
use std::sync::Arc;
use std::time::{Duration, Instant};

mod animation;
mod export;
mod interaction;
mod math_objects;
mod render;
mod scene;

use animation::AnimationPlugin;
use export::{ExportFormat, ExportPlugin, ExportRequest};
use interaction::InteractionPlugin;
use math_objects::{
    create_axes_with_labels, create_circle_with_resolution, create_grid, Axes, Grid,
    MathObjectPlugin, Style as MathStyle,
};
use render::RenderPlugin;
use scene::ScenePlugin;

/// UI显示状态资源
#[derive(Resource)]
struct UiVisibility {
    show_ui: bool,
}

impl Default for UiVisibility {
    fn default() -> Self {
        Self { show_ui: true }
    }
}

/// 性能监控状态资源
#[derive(Resource)]
struct PerformanceState {
    pub show_performance: bool,
    pub fps_history: Vec<f32>,
    pub memory_history: Vec<f32>,
    pub last_update: Instant,
    pub frame_count: u32,
    pub fps: f32,
    pub memory_usage_mb: f32,
    pub max_history_len: usize,
}

impl Default for PerformanceState {
    fn default() -> Self {
        Self {
            show_performance: false,
            fps_history: Vec::new(),
            memory_history: Vec::new(),
            last_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            memory_usage_mb: 0.0,
            max_history_len: 60, // 保持60个历史记录点
        }
    }
}

/// 相机状态资源，管理缩放和平移
#[derive(Resource)]
struct CameraState {
    pub zoom: f32,                // 当前缩放级别
    pub target_zoom: f32,         // 目标缩放级别
    pub zoom_speed: f32,          // 缩放速度
    pub min_zoom: f32,            // 最小缩放
    pub max_zoom: f32,            // 最大缩放
    pub translation: Vec2,        // 相机平移
    pub target_translation: Vec2, // 目标平移
    pub previous_zoom: f32,       // 上一帧的缩放级别，用于检测变化
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            target_zoom: 1.0,
            zoom_speed: 0.1,
            min_zoom: 0.1,
            max_zoom: 10.0,
            translation: Vec2::ZERO,
            target_translation: Vec2::ZERO,
            previous_zoom: 1.0,
        }
    }
}

/// 坐标系显示状态资源
#[derive(Resource)]
struct CoordinateSystemState {
    pub show_axes: bool,
    pub show_grid: bool,
}

impl Default for CoordinateSystemState {
    fn default() -> Self {
        Self {
            show_axes: true,
            show_grid: true,
        }
    }
}

/// 圆形管理状态资源
#[derive(Resource)]
struct CircleState {
    pub circles: Vec<Entity>,
    pub next_position: Vec2,
    pub default_radius: f32,
    pub default_color: Color,
    pub show_fill: bool,
    pub resolution: Option<u32>, // 圆形分辨率，None 表示自动
}

impl Default for CircleState {
    fn default() -> Self {
        Self {
            circles: Vec::new(),
            next_position: Vec2::new(0.0, 0.0),
            default_radius: 1.0,
            default_color: Color::srgb(0.2, 0.8, 0.2), // 绿色
            show_fill: false,
            resolution: None, // 默认使用自动分辨率
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RIM - Mathematical Visualization Tool".into(),
                resolution: (1200., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins((
            RenderPlugin,
            MathObjectPlugin,
            AnimationPlugin,
            ScenePlugin,
            InteractionPlugin,
            ExportPlugin,
        ))
        .init_resource::<UiVisibility>()
        .init_resource::<CameraState>()
        .init_resource::<CoordinateSystemState>()
        .init_resource::<CircleState>()
        .init_resource::<PerformanceState>()
        .add_systems(Startup, (setup_scene, setup_fonts, setup_coordinate_system))
        .add_systems(
            Update,
            (
                handle_ui_toggle,
                handle_mouse_input,
                update_camera_smooth,
                update_coordinate_system,
                handle_coordinate_system_toggle,
                update_performance_monitor,
                handle_performance_toggle,
            ),
        )
        .add_systems(EguiContextPass, ui_system)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // 设置2D相机
    commands.spawn(Camera2d);
}

/// 设置坐标系统 - 创建坐标轴和网格用于测试
fn setup_coordinate_system(mut commands: Commands) {
    // 创建网格
    create_grid(
        &mut commands,
        1.0, // 网格间距
        MathStyle {
            stroke_color: Color::srgba(0.3, 0.3, 0.3, 1.0),
            fill_color: None,
            stroke_width: 1.0,
            opacity: 0.3,
        },
    );

    // 创建坐标轴
    create_axes_with_labels(
        &mut commands,
        (-10.0, 10.0),   // x 范围
        (-8.0, 8.0),     // y 范围
        "x".to_string(), // x轴标签
        "y".to_string(), // y轴标签
        MathStyle {
            stroke_color: Color::WHITE,
            fill_color: None,
            stroke_width: 2.0,
            opacity: 1.0,
        },
    );
}

/// 处理UI切换的输入
fn handle_ui_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_visibility: ResMut<UiVisibility>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        ui_visibility.show_ui = !ui_visibility.show_ui;
        info!(
            "UI 显示状态切换为: {}",
            if ui_visibility.show_ui {
                "显示"
            } else {
                "隐藏"
            }
        );
    }
}

/// 处理鼠标输入（滚轮缩放）
fn handle_mouse_input(
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut camera_state: ResMut<CameraState>,
) {
    for event in scroll_events.read() {
        // 计算缩放变化
        let zoom_delta = event.y * camera_state.zoom_speed;
        camera_state.target_zoom = (camera_state.target_zoom + zoom_delta)
            .clamp(camera_state.min_zoom, camera_state.max_zoom);

        info!("目标缩放级别: {:.2}", camera_state.target_zoom);
    }
}

/// 平滑更新相机状态
fn update_camera_smooth(time: Res<Time>, mut camera_state: ResMut<CameraState>) {
    // 平滑插值到目标缩放
    let lerp_speed = 8.0; // 插值速度
    let dt = time.delta_secs();

    camera_state.zoom =
        camera_state.zoom + (camera_state.target_zoom - camera_state.zoom) * lerp_speed * dt;

    // 当差值很小时直接设置为目标值
    if (camera_state.target_zoom - camera_state.zoom).abs() < 0.001 {
        camera_state.zoom = camera_state.target_zoom;
    }
}

/// 根据相机状态更新坐标系统
fn update_coordinate_system(
    mut camera_state: ResMut<CameraState>,
    mut axes_query: Query<&mut Axes>,
    mut grid_query: Query<&mut Grid>,
) {
    // 检查缩放是否有变化
    if (camera_state.zoom - camera_state.previous_zoom).abs() > 0.001 {
        // 更新坐标轴
        for mut axes in axes_query.iter_mut() {
            axes.update_for_zoom(camera_state.zoom);
        }

        // 更新网格
        for mut grid in grid_query.iter_mut() {
            grid.update_for_zoom(camera_state.zoom);
        }

        camera_state.previous_zoom = camera_state.zoom;
    }
}

/// 处理坐标系显示切换的键盘快捷键
fn handle_coordinate_system_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut coordinate_state: ResMut<CoordinateSystemState>,
    mut axes_query: Query<&mut Visibility, (With<Axes>, Without<Grid>)>,
    mut grid_query: Query<&mut Visibility, (With<Grid>, Without<Axes>)>,
    mut export_events: EventWriter<ExportRequest>,
) {
    // A键切换坐标轴显示
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        coordinate_state.show_axes = !coordinate_state.show_axes;
        for mut visibility in axes_query.iter_mut() {
            *visibility = if coordinate_state.show_axes {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        info!(
            "坐标轴显示状态: {}",
            if coordinate_state.show_axes {
                "显示"
            } else {
                "隐藏"
            }
        );
    }

    // G键切换网格显示
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        coordinate_state.show_grid = !coordinate_state.show_grid;
        for mut visibility in grid_query.iter_mut() {
            *visibility = if coordinate_state.show_grid {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        info!(
            "网格显示状态: {}",
            if coordinate_state.show_grid {
                "显示"
            } else {
                "隐藏"
            }
        );
    }

    // S键保存截图
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        export_events.write(ExportRequest {
            format: ExportFormat::PNG,
            filename: format!(
                "rim_screenshot_{}.png",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            resolution: (1920, 1080),
        });
        info!("截图快捷键触发 - 截图请求已发送");
    }
}

/// 更新性能监控数据
fn update_performance_monitor(
    _time: Res<Time>,
    mut performance_state: ResMut<PerformanceState>,
) {
    performance_state.frame_count += 1;
    
    let now = Instant::now();
    let elapsed = now.duration_since(performance_state.last_update);
    
    // 每秒更新一次FPS和内存使用
    if elapsed >= Duration::from_secs(1) {
        // 计算FPS
        performance_state.fps = performance_state.frame_count as f32 / elapsed.as_secs_f32();
        performance_state.frame_count = 0;
        performance_state.last_update = now;
        
        // 获取内存使用（简化版本 - 在生产环境中可能需要更精确的方法）
        // 这里我们使用一个估算值，在实际项目中可以使用系统调用获取真实内存使用
        performance_state.memory_usage_mb = get_memory_usage_estimate();
        
        // 更新历史记录 - 分别获取值以避免借用检查问题
        let current_fps = performance_state.fps;
        let current_memory = performance_state.memory_usage_mb;
        let max_history_len = performance_state.max_history_len;
        
        performance_state.fps_history.push(current_fps);
        performance_state.memory_history.push(current_memory);
        
        // 限制历史记录长度
        if performance_state.fps_history.len() > max_history_len {
            performance_state.fps_history.remove(0);
        }
        if performance_state.memory_history.len() > max_history_len {
            performance_state.memory_history.remove(0);
        }
    }
}

/// 处理性能监控切换的键盘输入
fn handle_performance_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut performance_state: ResMut<PerformanceState>,
) {
    // 切换性能监控显示 (P键)
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        performance_state.show_performance = !performance_state.show_performance;
        info!(
            "性能监控显示状态: {}",
            if performance_state.show_performance {
                "显示"
            } else {
                "隐藏"
            }
        );
    }
}

/// 估算内存使用量（简化版本）
fn get_memory_usage_estimate() -> f32 {
    // 这是一个简化的估算，实际项目中可能需要使用系统API
    // 或者第三方库如 `sysinfo` 来获取精确的内存使用情况
    // 这里返回一个基于运行时间的模拟值
    let base_memory = 50.0; // 基础内存使用 50MB
    let time_factor = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() % 60) as f32;
    base_memory + (time_factor * 0.5) // 模拟内存使用的变化
}

fn setup_fonts(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    // 获取字体定义
    let mut fonts = egui::FontDefinitions::default();

    // 添加中文字体 - 使用系统默认的中文字体
    #[cfg(target_os = "macos")]
    {
        // macOS 系统字体 - 尝试 Arial Unicode (支持中文)
        let font_paths = [
            "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
            "/System/Library/Fonts/STHeiti Light.ttc",
            "/System/Library/Fonts/Hiragino Sans GB.ttc",
            "/System/Library/Fonts/PingFang.ttc",
        ];

        for font_path in &font_paths {
            if let Ok(font_data) = std::fs::read(font_path) {
                fonts.font_data.insert(
                    "chinese_font".to_owned(),
                    Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());
                break;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows 系统字体
        let font_paths = [
            "C:/Windows/Fonts/msyh.ttc",
            "C:/Windows/Fonts/simhei.ttf",
            "C:/Windows/Fonts/simsun.ttc",
        ];

        for font_path in &font_paths {
            if let Ok(font_data) = std::fs::read(font_path) {
                fonts.font_data.insert(
                    "chinese_font".to_owned(),
                    Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());
                break;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 系统字体
        let font_paths = [
            "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        ];

        for font_path in &font_paths {
            if let Ok(font_data) = std::fs::read(font_path) {
                fonts.font_data.insert(
                    "chinese_font".to_owned(),
                    Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());
                break;
            }
        }
    }

    // 应用字体配置
    ctx.set_fonts(fonts);
}

fn ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    ui_visibility: Res<UiVisibility>,
    camera_state: Res<CameraState>,
    mut coordinate_state: ResMut<CoordinateSystemState>,
    mut circle_state: ResMut<CircleState>,
    mut axes_query: Query<&mut Visibility, (With<Axes>, Without<Grid>)>,
    mut grid_query: Query<&mut Visibility, (With<Grid>, Without<Axes>)>,
    mut export_events: EventWriter<ExportRequest>,
    mut performance_state: ResMut<PerformanceState>,
) {
    // 只有当UI可见时才显示控制面板
    if ui_visibility.show_ui {
        egui::SidePanel::left("control_panel")
            .resizable(true)
            .min_width(250.0)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("RIM - 数学可视化工具");
                ui.separator();

                ui.collapsing("坐标系", |ui| {
                    ui.label("坐标轴设置");
                    if ui.button("显示/隐藏坐标轴").clicked() {
                        coordinate_state.show_axes = !coordinate_state.show_axes;
                        // 更新所有坐标轴的可见性
                        for mut visibility in axes_query.iter_mut() {
                            *visibility = if coordinate_state.show_axes {
                                Visibility::Inherited
                            } else {
                                Visibility::Hidden
                            };
                        }
                        info!(
                            "坐标轴显示状态: {}",
                            if coordinate_state.show_axes {
                                "显示"
                            } else {
                                "隐藏"
                            }
                        );
                    }
                    if ui.button("显示/隐藏网格").clicked() {
                        coordinate_state.show_grid = !coordinate_state.show_grid;
                        // 更新所有网格的可见性
                        for mut visibility in grid_query.iter_mut() {
                            *visibility = if coordinate_state.show_grid {
                                Visibility::Inherited
                            } else {
                                Visibility::Hidden
                            };
                        }
                        info!(
                            "网格显示状态: {}",
                            if coordinate_state.show_grid {
                                "显示"
                            } else {
                                "隐藏"
                            }
                        );
                    }
                    if ui.button("重置坐标轴").clicked() {
                        // TODO: 重置坐标轴到默认状态
                    }
                    ui.separator();
                    ui.label("缩放控制");
                    ui.label(format!("当前缩放: {:.2}x", camera_state.zoom));
                    ui.label(format!("目标缩放: {:.2}x", camera_state.target_zoom));
                    ui.label("使用鼠标滚轮进行缩放");
                    ui.separator();

                    // 计算当前视图范围
                    let half_width = 10.0 / camera_state.zoom; // 基于初始范围20计算
                    let half_height = 8.0 / camera_state.zoom; // 基于初始范围16计算
                    ui.label("坐标轴范围");
                    ui.label(format!("X: {:.1} 到 {:.1}", -half_width, half_width));
                    ui.label(format!("Y: {:.1} 到 {:.1}", -half_height, half_height));

                    ui.separator();
                    ui.label("显示状态");
                    ui.label(format!(
                        "坐标轴: {}",
                        if coordinate_state.show_axes {
                            "✅ 显示"
                        } else {
                            "❌ 隐藏"
                        }
                    ));
                    ui.label(format!(
                        "网格: {}",
                        if coordinate_state.show_grid {
                            "✅ 显示"
                        } else {
                            "❌ 隐藏"
                        }
                    ));
                });

                ui.collapsing("基本图形", |ui| {
                    ui.label("圆形控制");

                    // 圆形位置控制
                    ui.label("📍 位置控制 (数学坐标系)");
                    ui.horizontal(|ui| {
                        ui.label("位置 X:");
                        ui.add(
                            egui::DragValue::new(&mut circle_state.next_position.x)
                                .speed(0.1)
                                .range(-10.0..=10.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("位置 Y:");
                        ui.add(
                            egui::DragValue::new(&mut circle_state.next_position.y)
                                .speed(0.1)
                                .range(-10.0..=10.0),
                        );
                    });
                    ui.small("💡 坐标原点(0,0)在屏幕中心");

                    // 圆形半径控制
                    ui.horizontal(|ui| {
                        ui.label("半径:");
                        ui.add(
                            egui::DragValue::new(&mut circle_state.default_radius)
                                .speed(0.1)
                                .range(0.1..=5.0),
                        );
                    });

                    // 颜色选择
                    let mut color_array = [
                        circle_state.default_color.to_srgba().red,
                        circle_state.default_color.to_srgba().green,
                        circle_state.default_color.to_srgba().blue,
                    ];
                    ui.horizontal(|ui| {
                        ui.label("颜色:");
                        ui.color_edit_button_rgb(&mut color_array);
                    });
                    circle_state.default_color =
                        Color::srgb(color_array[0], color_array[1], color_array[2]);

                    // 填充选项
                    ui.checkbox(&mut circle_state.show_fill, "显示填充");

                    // 分辨率控制
                    ui.horizontal(|ui| {
                        ui.label("分辨率:");
                        let mut use_auto = circle_state.resolution.is_none();
                        ui.checkbox(&mut use_auto, "自动");

                        if use_auto {
                            circle_state.resolution = None;
                        } else if circle_state.resolution.is_none() {
                            circle_state.resolution = Some(64); // 默认高分辨率
                        }

                        if let Some(ref mut resolution) = circle_state.resolution {
                            ui.add(
                                egui::DragValue::new(resolution)
                                    .speed(1.0)
                                    .range(8..=256)
                                    .suffix(" 段"),
                            );
                        }
                    });

                    // 添加圆形按钮
                    if ui.button("🔵 添加圆形").clicked() {
                        let style = MathStyle {
                            stroke_color: circle_state.default_color,
                            fill_color: if circle_state.show_fill {
                                Some(Color::srgba(
                                    circle_state.default_color.to_srgba().red,
                                    circle_state.default_color.to_srgba().green,
                                    circle_state.default_color.to_srgba().blue,
                                    0.3, // 填充透明度
                                ))
                            } else {
                                None
                            },
                            stroke_width: 2.0,
                            opacity: 1.0,
                        };

                        let circle_entity = create_circle_with_resolution(
                            &mut commands,
                            circle_state.next_position,
                            circle_state.default_radius,
                            style,
                            circle_state.resolution,
                        );

                        circle_state.circles.push(circle_entity);
                        info!(
                            "添加圆形: 位置({:.1}, {:.1}), 半径{:.1}",
                            circle_state.next_position.x,
                            circle_state.next_position.y,
                            circle_state.default_radius
                        );

                        // 自动调整下一个圆形的位置
                        circle_state.next_position.x += 2.0;
                        if circle_state.next_position.x > 8.0 {
                            circle_state.next_position.x = -8.0;
                            circle_state.next_position.y += 2.0;
                        }
                        if circle_state.next_position.y > 6.0 {
                            circle_state.next_position.y = -6.0;
                        }
                    }

                    // 清除所有圆形按钮
                    if ui.button("🗑️ 清除所有圆形").clicked() {
                        for entity in &circle_state.circles {
                            commands.entity(*entity).despawn();
                        }
                        circle_state.circles.clear();
                        circle_state.next_position = Vec2::new(0.0, 0.0);
                        info!("已清除所有圆形");
                    }

                    ui.separator();
                    ui.label(format!("当前圆形数量: {}", circle_state.circles.len()));

                    if ui.button("添加直线").clicked() {
                        // TODO: 添加直线对象
                    }
                    if ui.button("添加函数图形").clicked() {
                        // TODO: 添加函数图形
                    }
                });

                ui.collapsing("动画控制", |ui| {
                    if ui.button("播放动画").clicked() {
                        // TODO: 播放动画
                    }
                    if ui.button("暂停动画").clicked() {
                        // TODO: 暂停动画
                    }
                    ui.separator();
                    ui.label("时间轴控制");
                    // TODO: 添加时间轴滑块
                });

                ui.collapsing("场景设置", |ui| {
                    if ui.button("新建场景").clicked() {
                        // TODO: 新建场景
                    }
                    if ui.button("保存场景").clicked() {
                        // TODO: 保存场景
                    }
                    if ui.button("加载场景").clicked() {
                        // TODO: 加载场景
                    }
                });

                ui.collapsing("导出选项", |ui| {
                    if ui.button("📸 保存截图").clicked() {
                        export_events.write(ExportRequest {
                            format: ExportFormat::PNG,
                            filename: format!(
                                "rim_screenshot_{}.png",
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs()
                            ),
                            resolution: (1920, 1080),
                        });
                        info!("截图请求已发送");
                    }
                    if ui.button("导出动画").clicked() {
                        // TODO: 导出动画
                    }
                    ui.separator();
                    ui.label("💡 截图说明");
                    ui.label("• 截图将保存到 screenshots/ 目录");
                    ui.label("• 支持PNG格式");
                    ui.label("• 自动生成时间戳文件名");
                });

                ui.collapsing("性能监控", |ui| {
                    // 性能监控开关
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut performance_state.show_performance, "显示性能信息");
                        if ui.button("📊").on_hover_text("切换性能监控显示").clicked() {
                            performance_state.show_performance = !performance_state.show_performance;
                        }
                    });

                    ui.separator();

                    // 实时性能数据
                    ui.label("🚀 实时性能");
                    ui.horizontal(|ui| {
                        ui.label(format!("FPS: {:.1}", performance_state.fps));
                        let fps_color = if performance_state.fps >= 60.0 {
                            egui::Color32::GREEN
                        } else if performance_state.fps >= 30.0 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(fps_color, format!("{:.1}", performance_state.fps));
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("内存: {:.1} MB", performance_state.memory_usage_mb));
                        let memory_color = if performance_state.memory_usage_mb < 100.0 {
                            egui::Color32::GREEN
                        } else if performance_state.memory_usage_mb < 200.0 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(memory_color, format!("{:.1} MB", performance_state.memory_usage_mb));
                    });

                    // 性能历史数据简化显示
                    if !performance_state.fps_history.is_empty() {
                        ui.separator();
                        ui.label("📈 性能趋势 (最近60秒)");
                        
                        // 显示最近几个数据点的简化图表
                        ui.horizontal(|ui| {
                            ui.label("FPS:");
                            let recent_fps = &performance_state.fps_history[performance_state.fps_history.len().saturating_sub(10)..];
                            for (i, &fps) in recent_fps.iter().enumerate() {
                                let color = if fps >= 60.0 {
                                    egui::Color32::GREEN
                                } else if fps >= 30.0 {
                                    egui::Color32::YELLOW
                                } else {
                                    egui::Color32::RED
                                };
                                ui.colored_label(color, format!("{:.0}", fps));
                                if i < recent_fps.len() - 1 {
                                    ui.label("|");
                                }
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("内存:");
                            let recent_memory = &performance_state.memory_history[performance_state.memory_history.len().saturating_sub(10)..];
                            for (i, &mem) in recent_memory.iter().enumerate() {
                                let color = if mem < 100.0 {
                                    egui::Color32::GREEN
                                } else if mem < 200.0 {
                                    egui::Color32::YELLOW
                                } else {
                                    egui::Color32::RED
                                };
                                ui.colored_label(color, format!("{:.0}", mem));
                                if i < recent_memory.len() - 1 {
                                    ui.label("|");
                                }
                            }
                        });
                    }

                    ui.separator();
                    ui.label("📋 统计信息");
                    if !performance_state.fps_history.is_empty() {
                        let avg_fps = performance_state.fps_history.iter().sum::<f32>() / performance_state.fps_history.len() as f32;
                        let max_fps = performance_state.fps_history.iter().fold(0.0f32, |a, &b| a.max(b));
                        let min_fps = performance_state.fps_history.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                        
                        ui.label(format!("平均FPS: {:.1}", avg_fps));
                        ui.label(format!("最大FPS: {:.1}", max_fps));
                        ui.label(format!("最小FPS: {:.1}", min_fps));
                    }

                    if !performance_state.memory_history.is_empty() {
                        let avg_memory = performance_state.memory_history.iter().sum::<f32>() / performance_state.memory_history.len() as f32;
                        let max_memory = performance_state.memory_history.iter().fold(0.0f32, |a, &b| a.max(b));
                        let min_memory = performance_state.memory_history.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                        
                        ui.label(format!("平均内存: {:.1} MB", avg_memory));
                        ui.label(format!("最大内存: {:.1} MB", max_memory));
                        ui.label(format!("最小内存: {:.1} MB", min_memory));
                    }

                    // 清除历史数据按钮
                    if ui.button("🗑️ 清除历史数据").clicked() {
                        performance_state.fps_history.clear();
                        performance_state.memory_history.clear();
                        info!("性能监控历史数据已清除");
                    }
                });

                ui.separator();
                ui.label("状态信息");
                ui.label(format!(
                    "{} 坐标轴{}",
                    if coordinate_state.show_axes {
                        "✅"
                    } else {
                        "❌"
                    },
                    if coordinate_state.show_axes {
                        "已显示"
                    } else {
                        "已隐藏"
                    }
                ));
                ui.label(format!(
                    "{} 网格{}",
                    if coordinate_state.show_grid {
                        "✅"
                    } else {
                        "❌"
                    },
                    if coordinate_state.show_grid {
                        "已显示"
                    } else {
                        "已隐藏"
                    }
                ));
                ui.label(format!("🔵 圆形: {} 个", circle_state.circles.len()));

                ui.separator();
                ui.label("快捷键");
                ui.label("F1 - 显示/隐藏UI");
                ui.label("A - 显示/隐藏坐标轴");
                ui.label("G - 显示/隐藏网格");
                ui.label("S - 保存截图");
                ui.label("P - 显示/隐藏性能信息");
                ui.label("鼠标滚轮 - 缩放");
            });
    } else {
        // 当UI隐藏时，显示一个小的提示
        egui::Window::new("控制提示")
            .fixed_pos([10.0, 10.0])
            .fixed_size([200.0, 100.0])
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label("⌨️ F1 显示UI");
                ui.label(format!("🔍 缩放: {:.1}x", camera_state.zoom));
                ui.label("🖱️ 滚轮缩放");
                ui.label("⌨️ P 性能信息");
            });

        // 当性能监控开启时，即使UI隐藏也显示性能信息
        if performance_state.show_performance {
            egui::Window::new("性能监控")
                .fixed_pos([10.0, 120.0])
                .fixed_size([180.0, 120.0])
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .show(contexts.ctx_mut(), |ui| {
                    ui.label("🚀 性能监控");
                    ui.separator();
                    
                    // FPS显示
                    ui.horizontal(|ui| {
                        ui.label("FPS:");
                        let fps_color = if performance_state.fps >= 60.0 {
                            egui::Color32::GREEN
                        } else if performance_state.fps >= 30.0 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(fps_color, format!("{:.1}", performance_state.fps));
                    });
                    
                    // 内存显示
                    ui.horizontal(|ui| {
                        ui.label("内存:");
                        let memory_color = if performance_state.memory_usage_mb < 100.0 {
                            egui::Color32::GREEN
                        } else if performance_state.memory_usage_mb < 200.0 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(memory_color, format!("{:.1} MB", performance_state.memory_usage_mb));
                    });
                    
                    ui.separator();
                    ui.small("P键切换显示");
                });
        }
    }
}
