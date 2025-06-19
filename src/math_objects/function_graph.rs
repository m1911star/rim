use super::{MathObject, Position2D, Style};
use bevy::prelude::*;

pub struct FunctionGraphPlugin;

impl Plugin for FunctionGraphPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FunctionGraph>()
            .register_type::<ParametricCurve>()
            .add_systems(Update, update_function_graphs);
    }
}

/// 函数类型
pub type MathFunction = Box<dyn Fn(f32) -> f32 + Send + Sync>;

/// 函数图形组件
#[derive(Component, Reflect)]
pub struct FunctionGraph {
    pub domain_start: f32,
    pub domain_end: f32,
    pub sample_count: u32,
    #[reflect(ignore)]
    pub points: Vec<Vec2>,
}

/// 参数方程曲线组件
#[derive(Component, Reflect)]
pub struct ParametricCurve {
    pub param_start: f32,
    pub param_end: f32,
    pub sample_count: u32,
    #[reflect(ignore)]
    pub points: Vec<Vec2>,
}

impl Default for FunctionGraph {
    fn default() -> Self {
        Self {
            domain_start: -5.0,
            domain_end: 5.0,
            sample_count: 100,
            points: Vec::new(),
        }
    }
}

impl Default for ParametricCurve {
    fn default() -> Self {
        Self {
            param_start: 0.0,
            param_end: 1.0,
            sample_count: 100,
            points: Vec::new(),
        }
    }
}

/// 创建函数图形的便利函数
pub fn create_function_graph(
    commands: &mut Commands,
    func: fn(f32) -> f32,
    domain: (f32, f32),
    style: Style,
) -> Entity {
    let mut graph = FunctionGraph {
        domain_start: domain.0,
        domain_end: domain.1,
        sample_count: 100,
        points: Vec::new(),
    };

    // 采样函数点
    for i in 0..graph.sample_count {
        let t = i as f32 / (graph.sample_count - 1) as f32;
        let x = graph.domain_start + t * (graph.domain_end - graph.domain_start);
        let y = func(x);
        graph.points.push(Vec2::new(x, y));
    }

    commands
        .spawn((
            MathObject {
                id: format!("function_{}", rand::random::<u32>()),
                visible: true,
                layer: 0,
            },
            graph,
            Position2D { x: 0.0, y: 0.0 },
            style,
            Transform::default(),
        ))
        .id()
}

/// 创建参数曲线的便利函数
pub fn create_parametric_curve(
    commands: &mut Commands,
    x_func: fn(f32) -> f32,
    y_func: fn(f32) -> f32,
    param_range: (f32, f32),
    style: Style,
) -> Entity {
    let mut curve = ParametricCurve {
        param_start: param_range.0,
        param_end: param_range.1,
        sample_count: 100,
        points: Vec::new(),
    };

    // 采样参数曲线点
    for i in 0..curve.sample_count {
        let t = curve.param_start
            + (i as f32 / (curve.sample_count - 1) as f32) * (curve.param_end - curve.param_start);
        let x = x_func(t);
        let y = y_func(t);
        curve.points.push(Vec2::new(x, y));
    }

    commands
        .spawn((
            MathObject {
                id: format!("curve_{}", rand::random::<u32>()),
                visible: true,
                layer: 0,
            },
            curve,
            Position2D { x: 0.0, y: 0.0 },
            style,
            Transform::default(),
        ))
        .id()
}

/// 更新函数图形的系统
fn update_function_graphs(mut query: Query<&mut FunctionGraph, Changed<FunctionGraph>>) {
    for mut graph in query.iter_mut() {
        // 这里可以添加实时更新函数图形的逻辑
        // 比如当函数参数改变时重新采样
        if graph.points.is_empty() {
            // 重新采样
            graph.points.clear();
            for i in 0..graph.sample_count {
                let t = i as f32 / (graph.sample_count - 1) as f32;
                let x = graph.domain_start + t * (graph.domain_end - graph.domain_start);
                // 这里需要一个默认函数，比如 y = x
                let y = x;
                graph.points.push(Vec2::new(x, y));
            }
        }
    }
}

/// 常用数学函数
pub mod functions {
    /// 正弦函数
    pub fn sin(x: f32) -> f32 {
        x.sin()
    }

    /// 余弦函数
    pub fn cos(x: f32) -> f32 {
        x.cos()
    }

    /// 二次函数
    pub fn quadratic(a: f32, b: f32, c: f32) -> impl Fn(f32) -> f32 {
        move |x: f32| a * x * x + b * x + c
    }

    /// 指数函数
    pub fn exp(x: f32) -> f32 {
        x.exp()
    }

    /// 对数函数
    pub fn ln(x: f32) -> f32 {
        if x > 0.0 {
            x.ln()
        } else {
            f32::NAN
        }
    }
}
