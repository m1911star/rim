use crate::math_objects::{Axes, Grid, MathObject, Position2D, Style as MathStyle};
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (render_axes, render_grid, render_math_objects))
            .add_systems(PostUpdate, (spawn_axis_labels, update_axis_labels));
    }
}

/// 用于标识坐标轴标签的组件
#[derive(Component)]
pub struct AxisLabel {
    pub axis: String, // "x" 或 "y"
    pub value: f32,   // 标签的数值
}

/// 用于标识坐标轴名称标签的组件
#[derive(Component)]
pub struct AxisNameLabel {
    pub axis: String, // "x" 或 "y"
}

/// 渲染坐标轴的系统
fn render_axes(
    mut gizmos: Gizmos,
    query: Query<(&Axes, &Position2D, &MathStyle, &Visibility), With<MathObject>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let window_width = window.width();
    let window_height = window.height();

    for (axes, position, style, visibility) in query.iter() {
        // 检查可见性 - 只有当实体可见时才渲染
        if *visibility == Visibility::Hidden {
            continue;
        }

        let scale = 50.0; // 单位长度对应的像素数
        let position_vec = Vec3::new(position.x, position.y, 0.0);

        // 计算视窗范围（让坐标轴延伸到窗口边界）
        let viewport_half_width = window_width * 0.6; // 稍微留一些边距
        let viewport_half_height = window_height * 0.6;

        // 绘制 X 轴 - 延伸到窗口边界
        let x_start = Vec3::new(-viewport_half_width, 0.0, 0.0) + position_vec;
        let x_end = Vec3::new(viewport_half_width, 0.0, 0.0) + position_vec;
        gizmos.line(x_start, x_end, style.stroke_color);

        // 绘制 Y 轴 - 延伸到窗口边界
        let y_start = Vec3::new(0.0, -viewport_half_height, 0.0) + position_vec;
        let y_end = Vec3::new(0.0, viewport_half_height, 0.0) + position_vec;
        gizmos.line(y_start, y_end, style.stroke_color);

        // 绘制箭头（X轴和Y轴）
        if axes.show_arrows {
            let arrow_size = 15.0; // 增大箭头使其更明显

            // 箭头固定在屏幕绝对位置，不受坐标轴位置和缩放影响
            let arrow_margin = 30.0; // 箭头距离视窗边缘的边距

            // X轴正向箭头 - 固定在屏幕右边缘（不加position_vec）
            let x_arrow_tip = Vec3::new(viewport_half_width - arrow_margin, 0.0, 0.0);
            let x_arrow_left = x_arrow_tip - Vec3::new(arrow_size, arrow_size * 0.5, 0.0);
            let x_arrow_right = x_arrow_tip - Vec3::new(arrow_size, -arrow_size * 0.5, 0.0);

            gizmos.line(x_arrow_tip, x_arrow_left, style.stroke_color);
            gizmos.line(x_arrow_tip, x_arrow_right, style.stroke_color);

            // Y轴正向箭头 - 固定在屏幕上边缘（不加position_vec）
            let y_arrow_tip = Vec3::new(0.0, viewport_half_height - arrow_margin, 0.0);
            let y_arrow_left = y_arrow_tip - Vec3::new(arrow_size * 0.5, arrow_size, 0.0);
            let y_arrow_right = y_arrow_tip - Vec3::new(-arrow_size * 0.5, arrow_size, 0.0);

            gizmos.line(y_arrow_tip, y_arrow_left, style.stroke_color);
            gizmos.line(y_arrow_tip, y_arrow_right, style.stroke_color);
        }

        // 绘制刻度线
        if axes.show_numbers {
            // X轴刻度
            let mut x = (axes.x_range.0 / axes.tick_spacing).ceil() * axes.tick_spacing;
            while x <= axes.x_range.1 {
                if (x - 0.0f32).abs() > 0.01 {
                    // 不在原点处画刻度
                    let tick_pos = Vec3::new(x * scale, 0.0, 0.0) + position_vec;
                    let tick_start = tick_pos - Vec3::new(0.0, 8.0, 0.0); // 增大刻度线
                    let tick_end = tick_pos + Vec3::new(0.0, 8.0, 0.0);
                    gizmos.line(tick_start, tick_end, style.stroke_color);
                }
                x += axes.tick_spacing;
            }

            // Y轴刻度
            let mut y = (axes.y_range.0 / axes.tick_spacing).ceil() * axes.tick_spacing;
            while y <= axes.y_range.1 {
                if (y - 0.0f32).abs() > 0.01 {
                    // 不在原点处画刻度
                    let tick_pos = Vec3::new(0.0, y * scale, 0.0) + position_vec;
                    let tick_start = tick_pos - Vec3::new(8.0, 0.0, 0.0); // 增大刻度线
                    let tick_end = tick_pos + Vec3::new(8.0, 0.0, 0.0);
                    gizmos.line(tick_start, tick_end, style.stroke_color);
                }
                y += axes.tick_spacing;
            }
        }

        // 绘制原点标记
        let origin = position_vec;
        gizmos.circle_2d(origin.truncate(), 4.0, style.stroke_color); // 稍微增大原点
    }
}

/// 渲染网格的系统
fn render_grid(
    mut gizmos: Gizmos,
    query: Query<(&Grid, &Position2D, &MathStyle, &Visibility), With<MathObject>>,
    _axes_query: Query<&Axes>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let window_width = window.width();
    let window_height = window.height();

    for (grid, position, style, visibility) in query.iter() {
        // 检查可见性 - 只有当实体可见时才渲染
        if *visibility == Visibility::Hidden {
            continue;
        }

        let scale = 50.0;
        let position_vec = Vec3::new(position.x, position.y, 0.0);

        // 计算视窗范围来确定网格绘制范围（覆盖整个可见区域）
        let viewport_half_width = window_width * 0.7; // 稍微扩展一些确保覆盖全部
        let viewport_half_height = window_height * 0.7;

        let grid_x_range = (-viewport_half_width / scale, viewport_half_width / scale);
        let grid_y_range = (-viewport_half_height / scale, viewport_half_height / scale);

        // 创建网格颜色（带透明度）
        let grid_color = Color::srgba(
            style.stroke_color.to_srgba().red,
            style.stroke_color.to_srgba().green,
            style.stroke_color.to_srgba().blue,
            grid.opacity,
        );

        // 垂直网格线
        let mut x = (grid_x_range.0 / grid.spacing).ceil() * grid.spacing;
        while x <= grid_x_range.1 {
            let line_start = Vec3::new(x * scale, grid_y_range.0 * scale, 0.0) + position_vec;
            let line_end = Vec3::new(x * scale, grid_y_range.1 * scale, 0.0) + position_vec;
            gizmos.line(line_start, line_end, grid_color);
            x += grid.spacing;
        }

        // 水平网格线
        let mut y = (grid_y_range.0 / grid.spacing).ceil() * grid.spacing;
        while y <= grid_y_range.1 {
            let line_start = Vec3::new(grid_x_range.0 * scale, y * scale, 0.0) + position_vec;
            let line_end = Vec3::new(grid_x_range.1 * scale, y * scale, 0.0) + position_vec;
            gizmos.line(line_start, line_end, grid_color);
            y += grid.spacing;
        }

        // 次网格线（更细的网格）
        if grid.show_minor_grid && grid.minor_spacing > 0.0 {
            let minor_color = Color::srgba(
                style.stroke_color.to_srgba().red,
                style.stroke_color.to_srgba().green,
                style.stroke_color.to_srgba().blue,
                grid.opacity * 0.3, // 次网格更透明
            );

            // 垂直次网格线
            let mut x = (grid_x_range.0 / grid.minor_spacing).ceil() * grid.minor_spacing;
            while x <= grid_x_range.1 {
                // 只绘制不与主网格重叠的线
                if (x % grid.spacing).abs() > 0.01 {
                    let line_start =
                        Vec3::new(x * scale, grid_y_range.0 * scale, 0.0) + position_vec;
                    let line_end = Vec3::new(x * scale, grid_y_range.1 * scale, 0.0) + position_vec;
                    gizmos.line(line_start, line_end, minor_color);
                }
                x += grid.minor_spacing;
            }

            // 水平次网格线
            let mut y = (grid_y_range.0 / grid.minor_spacing).ceil() * grid.minor_spacing;
            while y <= grid_y_range.1 {
                // 只绘制不与主网格重叠的线
                if (y % grid.spacing).abs() > 0.01 {
                    let line_start =
                        Vec3::new(grid_x_range.0 * scale, y * scale, 0.0) + position_vec;
                    let line_end = Vec3::new(grid_x_range.1 * scale, y * scale, 0.0) + position_vec;
                    gizmos.line(line_start, line_end, minor_color);
                }
                y += grid.minor_spacing;
            }
        }
    }
}

/// 渲染数学对象的通用系统
fn render_math_objects(_query: Query<(&MathObject, &Position2D, &MathStyle), Without<Axes>>) {
    // 这里可以添加其他数学对象的渲染逻辑
    // 比如圆形、直线、函数图形等
}

/// 生成坐标轴标签的系统
fn spawn_axis_labels(
    mut commands: Commands,
    query: Query<(Entity, &Axes, &Position2D), Added<Axes>>,
) {
    for (axes_entity, axes, position) in query.iter() {
        let scale = 50.0;

        // 为坐标轴实体添加子实体来显示文本标签
        commands.entity(axes_entity).with_children(|parent| {
            // X轴标签 - 位于X轴正端点附近
            parent.spawn((
                Text2d::new(&axes.x_label),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_translation(Vec3::new(axes.x_range.1 * scale + 25.0, -15.0, 1.0)),
                Visibility::Inherited,
                AxisNameLabel {
                    axis: "x".to_string(),
                },
            ));

            // Y轴标签 - 位于Y轴正端点附近
            parent.spawn((
                Text2d::new(&axes.y_label),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_translation(Vec3::new(-15.0, axes.y_range.1 * scale + 25.0, 1.0)),
                Visibility::Inherited,
                AxisNameLabel {
                    axis: "y".to_string(),
                },
            ));

            // 原点标识
            parent.spawn((
                Text2d::new("O"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_translation(Vec3::new(-15.0, -15.0, 1.0)),
                Visibility::Inherited,
                AxisNameLabel {
                    axis: "origin".to_string(),
                },
            ));
        });
    }
}

/// 更新坐标轴数字标签的系统
fn update_axis_labels(
    mut commands: Commands,
    axes_query: Query<(Entity, &Axes, &Position2D), Changed<Axes>>,
    label_query: Query<Entity, With<AxisLabel>>,
    mut name_label_query: Query<(&mut Transform, &AxisNameLabel), Without<Axes>>,
) {
    for (axes_entity, axes, _position) in axes_query.iter() {
        let scale = 50.0;

        // 删除旧的数字标签
        for label_entity in label_query.iter() {
            commands.entity(label_entity).despawn();
        }

        // 更新坐标轴名称标签的位置
        for (mut transform, name_label) in name_label_query.iter_mut() {
            match name_label.axis.as_str() {
                "x" => {
                    transform.translation = Vec3::new(axes.x_range.1 * scale + 25.0, -15.0, 1.0);
                }
                "y" => {
                    transform.translation = Vec3::new(-15.0, axes.y_range.1 * scale + 25.0, 1.0);
                }
                "origin" => {
                    // 原点标签位置保持不变
                    transform.translation = Vec3::new(-15.0, -15.0, 1.0);
                }
                _ => {}
            }
        }

        // 创建新的数字标签
        commands.entity(axes_entity).with_children(|parent| {
            // X轴数字标签
            if axes.show_numbers {
                let mut x = (axes.x_range.0 / axes.tick_spacing).ceil() * axes.tick_spacing;
                while x <= axes.x_range.1 {
                    if (x - 0.0f32).abs() > 0.01 {
                        // 格式化数字显示
                        let text = if axes.tick_spacing >= 1.0 {
                            format!("{:.0}", x)
                        } else if axes.tick_spacing >= 0.1 {
                            format!("{:.1}", x)
                        } else {
                            format!("{:.2}", x)
                        };

                        parent.spawn((
                            Text2d::new(text),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                            Transform::from_translation(Vec3::new(x * scale, -25.0, 1.0)),
                            Visibility::Inherited,
                            AxisLabel {
                                axis: "x".to_string(),
                                value: x,
                            },
                        ));
                    }
                    x += axes.tick_spacing;
                }

                // Y轴数字标签
                let mut y = (axes.y_range.0 / axes.tick_spacing).ceil() * axes.tick_spacing;
                while y <= axes.y_range.1 {
                    if (y - 0.0f32).abs() > 0.01 {
                        // 格式化数字显示
                        let text = if axes.tick_spacing >= 1.0 {
                            format!("{:.0}", y)
                        } else if axes.tick_spacing >= 0.1 {
                            format!("{:.1}", y)
                        } else {
                            format!("{:.2}", y)
                        };

                        parent.spawn((
                            Text2d::new(text),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                            Transform::from_translation(Vec3::new(-30.0, y * scale, 1.0)),
                            Visibility::Inherited,
                            AxisLabel {
                                axis: "y".to_string(),
                                value: y,
                            },
                        ));
                    }
                    y += axes.tick_spacing;
                }
            }
        });
    }
}
