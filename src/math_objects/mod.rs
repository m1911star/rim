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

pub mod axes;
pub mod basic_shapes;
pub mod function_graph;

pub use axes::*;
pub use basic_shapes::*;
pub use function_graph::*;

pub struct MathObjectPlugin;

impl Plugin for MathObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BasicShapesPlugin, FunctionGraphPlugin, AxesPlugin))
            .register_type::<MathObject>()
            .register_type::<Position2D>()
            .register_type::<Style>();
    }
}

/// 数学对象的基础组件
#[derive(Component, Reflect, Clone)]
pub struct MathObject {
    pub id: String,
    pub visible: bool,
    pub layer: i32,
}

/// 2D位置组件
#[derive(Component, Reflect, Clone)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl From<Vec2> for Position2D {
    fn from(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl Into<Vec2> for Position2D {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// 样式组件
#[derive(Component, Reflect, Clone)]
pub struct Style {
    pub stroke_color: Color,
    pub fill_color: Option<Color>,
    pub stroke_width: f32,
    pub opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            stroke_color: Color::WHITE,
            fill_color: None,
            stroke_width: 2.0,
            opacity: 1.0,
        }
    }
}

/// 数学对象类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MathObjectType {
    Circle,
    Line,
    Rectangle,
    FunctionGraph,
    Axes,
    Text,
}
