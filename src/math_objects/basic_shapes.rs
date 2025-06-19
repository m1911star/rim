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

use super::{MathObject, Position2D, Style};
use bevy::prelude::*;

pub struct BasicShapesPlugin;

impl Plugin for BasicShapesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MathCircle>()
            .register_type::<Line>()
            .register_type::<Rectangle>()
            .add_systems(Update, update_circle_mesh);
    }
}

/// 圆形组件
#[derive(Component, Reflect, Clone)]
pub struct MathCircle {
    pub radius: f32,
    pub color: Color,
    pub filled: bool,
    /// 圆形的分辨率（线段数量），None 表示使用自动分辨率
    pub resolution: Option<u32>,
}

impl Default for MathCircle {
    fn default() -> Self {
        Self {
            radius: 1.0,
            color: Color::WHITE,
            filled: true,
            resolution: None, // 默认使用自动分辨率
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
    create_circle_with_resolution(commands, position, radius, style, None)
}

/// 创建带指定分辨率的圆形
pub fn create_circle_with_resolution(
    commands: &mut Commands,
    position: Vec2,
    radius: f32,
    style: Style,
    resolution: Option<u32>,
) -> Entity {
    commands
        .spawn((
            MathObject {
                id: format!("circle_{}", rand::random::<u32>()),
                visible: true,
                layer: 0,
            },
            MathCircle {
                radius,
                color: style.stroke_color,
                filled: style.fill_color.is_some(),
                resolution,
            },
            Position2D::from(position),
            style,
            Transform::from_translation(position.extend(0.0)),
            Visibility::Visible,
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
            Visibility::Visible,
        ))
        .id()
}

/// 更新圆形网格的系统
fn update_circle_mesh(
    mut query: Query<(&MathCircle, &Position2D, &mut Transform), Changed<MathCircle>>,
) {
    for (_circle, position, mut transform) in query.iter_mut() {
        let pos_vec: Vec2 = position.clone().into();
        transform.translation = pos_vec.extend(0.0);
    }
}
