# RIM - 数学可视化工具

**中文** | [English](README.md)

基于 Bevy 游戏引擎的数学可视化工具，灵感来自 Manim。

## 🎯 项目目标

创建一个高性能的交互式数学可视化工具，结合：
- **Manim** 的优秀数学动画设计理念
- **Bevy** 的现代游戏引擎架构和高性能渲染

## ✨ 当前功能

### 🎯 坐标系统
- ✅ **坐标轴渲染** - 支持带箭头的 X 和 Y 轴
- ✅ **轴标签** - 可自定义轴标签名称（默认为 "x" 和 "y"）
- ✅ **刻度线** - 自动生成刻度线和数字标记
- ✅ **网格系统** - 主网格线和次网格线支持
- ✅ **原点标识** - 清晰的原点标记

### 🎨 渲染系统
- ✅ **Gizmos 渲染** - 使用 Bevy 内置 Gizmos 系统进行高效渲染
- ✅ **颜色和透明度** - 支持自定义颜色和透明度设置
- ✅ **可见性控制** - 可以控制对象的显示和隐藏

### 🖱️ 用户界面
- ✅ **直观界面** - 简洁友好的用户界面
- ✅ **控制面板** - 左侧面板包含各种控制选项
- ✅ **坐标系设置** - 专门的坐标系配置区域
- ✅ **状态显示** - 实时显示系统状态

## 🚀 快速开始

### 环境要求
- Rust 1.85+ 
- Bevy 0.16.1

### 安装与运行
```bash
# 克隆项目
git clone https://github.com/your-username/rim.git
cd rim

# 运行程序
cargo run
```

## 🎯 使用示例

### 创建坐标轴
```rust
use math_objects::{create_axes_with_labels, Style as MathStyle};

// 创建自定义坐标轴
create_axes_with_labels(
    &mut commands,
    (-10.0, 10.0),      // X 轴范围
    (-8.0, 8.0),        // Y 轴范围
    "时间".to_string(),   // X 轴标签
    "速度".to_string(),   // Y 轴标签
    MathStyle {
        stroke_color: Color::WHITE,
        stroke_width: 2.0,
        opacity: 1.0,
        ..default()
    }
);
```

### 创建网格
```rust
use math_objects::{create_grid, Style as MathStyle};

// 创建网格
create_grid(
    &mut commands,
    1.0, // 网格间距
    MathStyle {
        stroke_color: Color::srgba(0.3, 0.3, 0.3, 1.0),
        opacity: 0.3,
        ..default()
    }
);
```

## 🎨 计划中的核心功能

### 1. 数学对象 (MathObjects)
- **基础图形**: Circle, Line, Rectangle, Polygon
- **函数图形**: FunctionGraph, ParametricCurve, VectorField
- **3D对象**: Sphere, Cube, Surface, Polyhedron
- **坐标系统**: Axes, Grid, NumberLine, ComplexPlane

### 2. 动画系统 (Animation)
- **变换动画**: Transform, Rotate, Scale, Fade
- **路径动画**: Follow, DrawBoundingBox, Write
- **群组动画**: AnimationGroup, Succession
- **缓动函数**: 内置和自定义插值函数

### 3. 渲染引擎 (Render)
- **几何渲染**: 高性能2D/3D图形渲染
- **文本渲染**: LaTeX数学公式支持
- **材质系统**: 自定义材质，渐变效果
- **相机控制**: 多视角，缩放平移

### 4. 交互控制 (Interaction)
- **鼠标操作**: 拖拽、点击、缩放
- **键盘快捷键**: 常用操作绑定
- **触摸支持**: 移动设备适配

### 5. 场景管理 (Scene)
- **场景组织**: 多场景管理，图层控制
- **时间轴**: 关键帧动画，时间控制
- **状态管理**: 撤销/重做功能

### 6. 导出功能 (Export)
- **图像**: PNG, SVG高质量导出
- **动画**: GIF, MP4视频导出
- **3D模型**: OBJ, STL格式导出

## 🛠️ 开发路线图

### Phase 1: 基础框架 (当前阶段)
- [x] 项目结构搭建
- [x] 核心模块定义
- [x] 基础UI界面
- [x] 基础坐标系统
- [ ] 基础图形渲染
- [ ] 简单交互系统

### Phase 2: 核心功能 (v0.2.0)
- [ ] 交互式坐标轴控制 (拖拽、缩放)
- [ ] 基础几何形状 (圆形、直线、矩形)
- [ ] 简单的函数图形绘制
- [ ] 颜色主题系统

### Phase 3: 高级功能 (v0.3.0)
- [ ] 动画系统基础框架
- [ ] 图像导出功能
- [ ] 场景保存与加载
- [ ] 更多数学函数支持

### Phase 4: 完整版本 (v1.0.0)
- [ ] 完整的动画制作工具
- [ ] 视频导出功能
- [ ] 插件系统
- [ ] LaTeX 渲染支持

## 🔧 技术栈

- **核心引擎**: Bevy 0.16.1
- **UI框架**: bevy_egui
- **数学计算**: nalgebra
- **几何渲染**: lyon
- **序列化**: serde
- **错误处理**: anyhow, thiserror

## 🎯 设计理念

### 参考 Manim 的优点
- **数学专业性**: 专为数学可视化设计
- **动画优雅性**: 流畅自然的数学动画
- **API设计**: 直观的数学对象API
- **高质量输出**: 适合教学和演示

### 结合 Bevy 的优势
- **高性能**: Rust + ECS架构的性能优势
- **实时交互**: 游戏引擎级别的实时渲染
- **跨平台**: 桌面、移动、Web全平台支持
- **现代架构**: 模块化、可扩展的设计

## 🤝 贡献指南

欢迎各种形式的贡献：
- 🐛 报告Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码

请查看 [DEVELOPMENT.md](DEVELOPMENT.md) 了解开发指南。

## 📄 许可证

本项目采用 MIT 或 Apache 2.0 双重许可证。详情请查看 LICENSE 文件。

## 🙏 致谢

- [Bevy](https://bevyengine.org/) - 优秀的 Rust 游戏引擎
- [Manim](https://www.manim.community/) - 数学动画制作的灵感来源
- [egui](https://github.com/emilk/egui) - 出色的即时模式 GUI 库

---

**让数学变得更美丽，让学习变得更有趣！** ✨

## 📸 截图

*随着项目开发将添加截图*

## 🌟 Star 历史

[![Star History Chart](https://api.star-history.com/svg?repos=your-username/rim&type=Date)](https://star-history.com/#your-username/rim&Date)

---

**注**: 本项目目前处于早期开发阶段，API 可能会有较大变动。 