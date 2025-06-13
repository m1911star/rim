# 开发指南

本文档为 RIM 数学可视化工具的开发指南，详细介绍项目架构、开发流程和技术细节。

## 🏗️ 项目架构详解

### ECS 架构设计

RIM 基于 Bevy 的 ECS (Entity-Component-System) 架构：

```rust

// Systems: 系统处理组件逻辑
fn update_circle_mesh(
    query: Query<(&Circle, &Position2D, &mut Transform), Changed<Circle>>,
) {
    // 更新圆形的渲染
}
```

### 核心组件系统

#### 1. MathObject - 数学对象基础组件
```rust
#[derive(Component, Reflect, Clone)]
pub struct MathObject {
    pub id: String,      // 唯一标识符
    pub visible: bool,   // 是否可见
    pub layer: i32,      // 渲染层级
}
```

#### 2. 几何组件


#### 3. 样式组件
```rust
#[derive(Component, Reflect, Clone)]
pub struct Style {
    pub stroke_color: Color,
    pub fill_color: Option<Color>,
    pub stroke_width: f32,
    pub opacity: f32,
}
```

#### 4. 动画组件
```rust
#[derive(Component, Reflect, Clone)]
pub struct MathAnimation {
    pub duration: f32,
    pub elapsed: f32,
    pub is_playing: bool,
    pub loop_animation: bool,
}
```

## 🎨 渲染管线

### 渲染流程
1. **几何生成**: 根据数学对象生成几何数据
2. **样式应用**: 应用颜色、线宽等样式
3. **变换处理**: 应用位置、旋转、缩放变换
4. **层级排序**: 按layer字段排序渲染顺序
5. **GPU渲染**: 提交到GPU进行最终渲染

### 自定义渲染器
```rust
// 为新的数学对象添加渲染支持
impl Renderable for MyMathObject {
    fn generate_mesh(&self) -> Mesh {
        // 生成几何网格
    }
    
    fn apply_style(&self, style: &Style) -> Material {
        // 应用材质样式
    }
}
```

## 🎬 动画系统

### 动画类型

#### 1. 变换动画
```rust
// 移动动画
fn create_move_animation(
    entity: Entity,
    from: Vec2,
    to: Vec2,
    duration: f32,
) -> MathAnimation {
    // 实现移动动画
}

// 缩放动画
fn create_scale_animation(
    entity: Entity,
    from_scale: f32,
    to_scale: f32,
    duration: f32,
) -> MathAnimation {
    // 实现缩放动画
}
```

#### 2. 绘制动画
```rust
// 路径绘制动画（类似Manim的Write）
fn create_draw_animation(
    entity: Entity,
    duration: f32,
) -> MathAnimation {
    // 实现路径绘制动画
}
```

#### 3. 缓动函数
```rust
pub enum EasingFunction {
    Linear,
    EaseInOut,
    EaseIn,
    EaseOut,
    Bounce,
    Elastic,
}

pub fn apply_easing(t: f32, easing: EasingFunction) -> f32 {
    match easing {
        EasingFunction::Linear => t,
        EasingFunction::EaseInOut => smooth_step(t),
        // ... 其他缓动函数
    }
}
```

## 🔧 开发工作流

### 1. 添加新的数学对象

```rust
// 1. 定义组件
#[derive(Component, Reflect, Clone)]
pub struct Ellipse {
    pub a: f32,  // 长半轴
    pub b: f32,  // 短半轴
}

// 2. 实现默认值
impl Default for Ellipse {
    fn default() -> Self {
        Self { a: 2.0, b: 1.0 }
    }
}

// 3. 创建便利函数
pub fn create_ellipse(
    commands: &mut Commands,
    position: Vec2,
    a: f32,
    b: f32,
    style: Style,
) -> Entity {
    commands.spawn((
        MathObject {
            id: format!("ellipse_{}", generate_id()),
            visible: true,
            layer: 0,
        },
        Ellipse { a, b },
        Position2D::from(position),
        style,
        Transform::from_translation(position.extend(0.0)),
    )).id()
}

// 4. 添加更新系统
fn update_ellipse_mesh(
    mut query: Query<(&Ellipse, &Position2D, &mut Transform), Changed<Ellipse>>,
) {
    for (ellipse, position, mut transform) in query.iter_mut() {
        // 更新椭圆的渲染
    }
}

// 5. 在插件中注册
impl Plugin for BasicShapesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ellipse>()
           .add_systems(Update, update_ellipse_mesh);
    }
}
```

### 2. 添加新的动画类型

```rust
// 1. 定义动画组件
#[derive(Component)]
pub struct FadeAnimation {
    pub from_opacity: f32,
    pub to_opacity: f32,
    pub current_opacity: f32,
}

// 2. 实现动画系统
fn update_fade_animations(
    mut query: Query<(&mut FadeAnimation, &MathAnimation, &mut Style)>,
) {
    for (mut fade, animation, mut style) in query.iter_mut() {
        if animation.is_playing {
            let t = animation.elapsed / animation.duration;
            fade.current_opacity = lerp(fade.from_opacity, fade.to_opacity, t);
            style.opacity = fade.current_opacity;
        }
    }
}
```

### 3. 扩展UI功能

```rust
// 在ui_system中添加新的UI元素
fn ui_system(mut contexts: EguiContexts) {
    egui::SidePanel::left("control_panel")
        .show(contexts.ctx_mut(), |ui| {
            ui.collapsing("新功能", |ui| {
                if ui.button("添加椭圆").clicked() {
                    // 添加椭圆的逻辑
                }
                
                ui.separator();
                ui.label("椭圆参数");
                ui.add(egui::Slider::new(&mut ellipse_a, 0.1..=5.0).text("长半轴"));
                ui.add(egui::Slider::new(&mut ellipse_b, 0.1..=5.0).text("短半轴"));
            });
        });
}
```

## 🧪 测试策略

### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_creation() {
        let mut app = App::new();
        app.add_plugins(MathObjectPlugin);
        
        let mut commands = app.world.commands();
        let entity = create_circle(
            &mut commands,
            Vec2::ZERO,
            1.0,
            Style::default(),
        );
        
        assert!(app.world.get::<Circle>(entity).is_some());
    }
}
```

### 集成测试
```rust
#[test]
fn test_animation_playback() {
    let mut app = App::new();
    app.add_plugins((MathObjectPlugin, AnimationPlugin));
    
    // 创建测试场景
    // 验证动画播放逻辑
}
```

## 📊 性能优化

### 1. 批量渲染
- 合并相同材质的对象
- 使用instanced rendering减少draw calls

### 2. 几何优化
- 使用适当的细分级别
- 动态LOD (Level of Detail)

### 3. 内存管理
- 复用几何数据
- 及时清理不需要的资源

## 🔍 调试工具

### 1. 使用Bevy Inspector
```rust
// 在main.rs中添加
app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
```

### 2. 自定义调试信息
```rust
fn debug_system(
    query: Query<(Entity, &MathObject, &Position2D)>,
) {
    for (entity, math_obj, pos) in query.iter() {
        println!("Entity {:?}: {} at ({}, {})", 
                entity, math_obj.id, pos.x, pos.y);
    }
}
```

## 📝 代码风格指南

### 命名约定
- 结构体: `PascalCase`
- 函数和变量: `snake_case`
- 常量: `UPPER_SNAKE_CASE`
- 模块: `snake_case`

### 文档注释
```rust
/// 创建一个圆形对象
/// 
/// # 参数
/// - `commands`: Bevy命令缓冲区
/// - `position`: 圆心位置
/// - `radius`: 半径
/// - `style`: 渲染样式
/// 
/// # 返回值
/// 返回创建的实体ID
/// 
/// # 示例
/// ```rust
/// let circle = create_circle(&mut commands, Vec2::ZERO, 1.0, Style::default());
/// ```
pub fn create_circle(
    commands: &mut Commands,
    position: Vec2,
    radius: f32,
    style: Style,
) -> Entity {
    // 实现...
}
```

## 🚀 构建和部署

### 开发构建
```bash
cargo run --features dev
```

### 发布构建
```bash
cargo build --release
```

### Web部署
```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rim.wasm
```

## 📚 学习资源

### Bevy相关
- [Bevy官方文档](https://bevy-cheatbook.github.io/)
- [Bevy示例集合](https://github.com/bevyengine/bevy/tree/main/examples)

### 数学可视化
- [Manim文档](https://docs.manim.community/)
- [3Blue1Brown视频](https://www.youtube.com/c/3blue1brown)

### Rust相关
- [Rust程序设计语言](https://doc.rust-lang.org/book/)
- [Rust异步编程](https://rust-lang.github.io/async-book/)

---

## 🤝 贡献流程

1. Fork项目
2. 创建功能分支: `git checkout -b feature/new-feature`
3. 提交更改: `git commit -am 'Add new feature'`
4. 推送分支: `git push origin feature/new-feature`
5. 创建Pull Request

欢迎所有形式的贡献！🎉 