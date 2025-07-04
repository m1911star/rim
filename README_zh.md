# RIM - 数学可视化工具

**中文** | [English](README.md)

基于 Bevy 游戏引擎的数学可视化工具，灵感来自 Manim。

![RIM 预览](screenshots/rim_screenshot_1749743564.png)

## 🎯 项目目标

创建一个高性能的交互式数学可视化工具，结合：
- **Manim** 的优秀数学动画设计理念
- **Bevy** 的现代游戏引擎架构和高性能渲染

## ✨ 当前功能

- ✅ **坐标系统**: 完整的坐标轴和网格系统，支持自定义范围
- ✅ **交互控制**: 鼠标滚轮缩放，键盘快捷键
- ✅ **可见性管理**: 独立显示/隐藏坐标轴和网格
- ✅ **导出系统**: PNG截图导出，时间戳命名
- ✅ **UI控制面板**: 全面的侧边栏，可折叠分区
- ✅ **基本图形**: 圆形创建，支持位置、半径和颜色控制
- ✅ **性能监控**: 实时FPS和内存使用率跟踪，历史数据记录
- 🚧 **动画系统**: 基础框架已建立，正在实现中
- 📋 **函数图形**: 数学函数绘图（计划中）
- 📋 **高级图形**: 直线、矩形和复杂几何图形（计划中）

## 🎮 使用方法

### 基本操作
```rust
// 应用程序启动时会显示坐标系统
// 使用左侧边栏控制各种功能：

// 坐标系统
- 显示/隐藏坐标轴和网格
- 查看当前缩放级别和坐标范围
- 重置坐标系统到默认状态
- 数学坐标系：原点(0,0)位于屏幕中心，X轴向右为正，Y轴向上为正

// 基本图形 - 圆形
- 设置位置（X、Y坐标）
- 调整半径（0.1到5.0单位）
- 使用颜色选择器选择描边颜色
- 切换填充选项显示实心圆
- 添加多个圆形，自动定位
- 一键清除所有圆形

// 导出选项
- 拍摄PNG截图，时间戳命名
- 截图保存到screenshots/目录
```

### 键盘快捷键
| 按键 | 功能 |
|-----|------|
| `F1` | 切换UI显示 |
| `A` | 切换坐标轴显示 |
| `G` | 切换网格显示 |
| `S` | 截图 |
| `P` | 切换性能监控显示 |
| `鼠标滚轮` | 缩放 |

### 圆形控制
**基本图形**面板提供全面的圆形管理：

- **位置控制**: 使用拖拽值设置X和Y坐标（-10.0到10.0）
- **半径控制**: 使用拖拽值调整圆形大小（0.1到5.0）
- **颜色选择**: RGB颜色选择器设置描边颜色
- **填充选项**: 切换显示带透明度的填充圆形
- **分辨率控制**: 自动或手动调整圆形精度（8-256段）
- **智能定位**: 新圆形的自动位置调整
- **批量操作**: 一键清除所有圆形

### 导出功能
- **PNG截图**: 高质量图像导出
- **自动命名**: 基于时间戳的文件名
- **目录管理**: 自动创建screenshots文件夹
- **用户反馈**: 状态消息和操作确认

### 性能监控
**性能监控**面板提供实时系统性能洞察：

- **实时FPS**: 帧率监控，带颜色编码指示器（绿色：60+，黄色：30-60，红色：<30）
- **内存使用**: 实时内存消耗跟踪，以MB为单位
- **CPU占用**: 实时CPU使用率监控，带颜色编码指示器（绿色：<50%，黄色：50-80%，红色：>80%）
- **性能历史**: 显示FPS、内存和CPU的最近5个数据点的可视化趋势
- **统计分析**: 显示历史数据的平均值、最大值和最小值
- **历史管理**: 一键清除性能历史数据
- **始终可用**: 即使UI隐藏时也可显示性能叠加层
- **智能显示**: 性能数据每秒更新一次，在精度和性能之间达到最佳平衡

## 🚀 快速开始

### 环境要求
- Rust 1.85+ 
- Bevy 0.16.1

### 安装与运行
```bash
# 克隆项目
git clone https://github.com/m1911star/rim.git
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

### 创建圆形
```rust
use math_objects::{create_circle, create_circle_with_resolution, Style as MathStyle};

// 创建自动分辨率的圆形
create_circle(
    &mut commands,
    Vec2::new(0.0, 0.0),  // 位置
    1.5,                  // 半径
    MathStyle {
        stroke_color: Color::BLUE,
        fill_color: Some(Color::srgba(0.0, 0.0, 1.0, 0.3)),
        stroke_width: 2.0,
        opacity: 1.0,
    }
);

// 创建高分辨率圆形以获得平滑渲染
create_circle_with_resolution(
    &mut commands,
    Vec2::new(3.0, 0.0),  // 位置
    2.0,                  // 半径
    MathStyle {
        stroke_color: Color::RED,
        fill_color: None,
        stroke_width: 2.0,
        opacity: 1.0,
    },
    Some(128),            // 高分辨率（128段）
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
- **图像**: PNG 高质量导出 (✅ 已实现), SVG 导出 (计划中)
- **动画**: GIF, MP4 视频导出 (计划中)
- **3D模型**: OBJ, STL 格式导出 (计划中)
- **截图系统**: 实时捕获和自动文件管理 (✅ 已实现)

## 🛠️ 开发路线图

### Phase 1: 基础框架 (当前阶段)
- [x] 项目结构搭建
- [x] 核心模块定义
- [x] 基础UI界面
- [x] 基础坐标系统
- [x] 坐标系可见性控制
- [x] 鼠标滚轮缩放功能
- [x] 键盘快捷键系统
- [x] 截图导出功能
- [ ] 基础图形渲染
- [ ] 简单交互系统

### Phase 2: 核心功能 (v0.2.0)
- [ ] 交互式坐标轴控制 (拖拽、缩放)
- [ ] 基础几何形状 (圆形、直线、矩形)
- [ ] 简单的函数图形绘制
- [ ] 颜色主题系统
- [ ] 增强导出选项 (SVG, GIF)

### Phase 3: 高级功能 (v0.3.0)
- [ ] 动画系统基础框架
- [ ] 视频导出功能 (MP4)
- [ ] 场景保存与加载
- [ ] 更多数学函数支持

### Phase 4: 完整版本 (v1.0.0)
- [ ] 完整的动画制作工具
- [ ] 高级视频导出功能
- [ ] 插件系统
- [ ] LaTeX 渲染支持

## 🔧 技术栈

- **核心引擎**: Bevy 0.16.1
- **UI框架**: bevy_egui
- **截图系统**: Bevy 内置截图 API
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

本项目采用 GNU 通用公共许可证 v3.0 许可。详情请查看 [LICENSE](LICENSE) 文件。

本程序是自由软件：您可以根据自由软件基金会发布的 GNU 通用公共许可证第3版的条款重新分发和/或修改它。

本程序的分发希望它有用，但不提供任何保证；甚至不提供适销性或特定用途适用性的默示保证。有关更多详细信息，请参阅 GNU 通用公共许可证。

## 🙏 致谢

- [Bevy](https://bevyengine.org/) - 优秀的 Rust 游戏引擎
- [Manim](https://www.manim.community/) - 数学动画制作的灵感来源
- [egui](https://github.com/emilk/egui) - 出色的即时模式 GUI 库

---

**让数学变得更美丽，让学习变得更有趣！** ✨

## 📸 截图

### 主界面
![RIM 主界面](screenshots/rim_screenshot_1749743564.png)

## 🌟 Star 历史

[![Star History Chart](https://api.star-history.com/svg?repos=m1911star/rim&type=Date)](https://star-history.com/#m1911star/rim&Date)

---

**注**: 本项目目前处于早期开发阶段，API 可能会有较大变动。 