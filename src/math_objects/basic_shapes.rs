use super::{MathObject, Position2D, Style};
use bevy::prelude::*;

pub struct BasicShapesPlugin;

impl Plugin for BasicShapesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Circle>()
            .register_type::<Line>()
            .register_type::<Rectangle>()
            .add_systems(Update, update_circle_mesh);
    }
}

/// 圆形组件
#[derive(Component, Reflect, Clone)]
pub struct Circle {
    pub radius: f32,
    pub segments: u32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 1.0,
            segments: 64,
        }
    }
}

/// 直线组件
#[derive(Component, Reflect, Clone)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

/// 矩形组件
#[derive(Component, Reflect, Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 2.0,
            height: 1.0,
        }
    }
}

/// 创建圆形的便利函数
pub fn create_circle(commands: &mut Commands, position: Vec2, radius: f32, style: Style) -> Entity {
    commands
        .spawn((
            MathObject {
                id: format!("circle_{}", rand::random::<u32>()),
                visible: true,
                layer: 0,
            },
            Circle {
                radius,
                segments: 64,
            },
            Position2D::from(position),
            style,
            Transform::from_translation(position.extend(0.0)),
        ))
        .id()
}

/// 创建直线的便利函数
pub fn create_line(commands: &mut Commands, start: Vec2, end: Vec2, style: Style) -> Entity {
    commands
        .spawn((
            MathObject {
                id: format!("line_{}", rand::random::<u32>()),
                visible: true,
                layer: 0,
            },
            Line { start, end },
            Position2D::from((start + end) * 0.5),
            style,
            Transform::from_translation(((start + end) * 0.5).extend(0.0)),
        ))
        .id()
}

/// 更新圆形网格的系统
fn update_circle_mesh(mut query: Query<(&Circle, &Position2D, &mut Transform), Changed<Circle>>) {
    for (_circle, position, mut transform) in query.iter_mut() {
        let pos_vec: Vec2 = position.clone().into();
        transform.translation = pos_vec.extend(0.0);
    }
}
