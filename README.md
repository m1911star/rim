# RIM - Mathematical Visualization Tool

[中文](README_zh.md) | **English**

A high-performance mathematical visualization tool built with Bevy game engine, inspired by Manim.

## 🎯 Project Goals

Create a high-performance interactive mathematical visualization tool that combines:
- **Manim's** excellent mathematical animation design philosophy
- **Bevy's** modern game engine architecture and high-performance rendering

## ✨ Current Features

### 🎯 Coordinate System
- ✅ **Axis Rendering** - X and Y axes with customizable arrows
- ✅ **Axis Labels** - Customizable axis labels (default "x" and "y")
- ✅ **Tick Marks** - Automatic tick marks and numerical labels
- ✅ **Grid System** - Major and minor grid lines support
- ✅ **Origin Marker** - Clear origin point indicator

### 🎨 Rendering System
- ✅ **Gizmos Rendering** - Efficient rendering using Bevy's built-in Gizmos system
- ✅ **Colors & Transparency** - Custom color and opacity settings
- ✅ **Visibility Control** - Show/hide objects dynamically

### 🖱️ User Interface
- ✅ **Intuitive Interface** - Clean and user-friendly interface
- ✅ **Control Panel** - Left sidebar with various control options
- ✅ **Coordinate Settings** - Dedicated coordinate system configuration
- ✅ **Status Display** - Real-time system status information

## 🚀 Quick Start

### Prerequisites
- Rust 1.85+ 
- Bevy 0.16.1

### Installation & Running
```bash
# Clone the repository
git clone https://github.com/your-username/rim.git
cd rim

# Run the application
cargo run
```

## 🎯 Usage Examples

### Creating Coordinate Axes
```rust
use math_objects::{create_axes_with_labels, Style as MathStyle};

// Create custom coordinate axes
create_axes_with_labels(
    &mut commands,
    (-10.0, 10.0),        // X-axis range
    (-8.0, 8.0),          // Y-axis range
    "Time".to_string(),   // X-axis label
    "Velocity".to_string(), // Y-axis label
    MathStyle {
        stroke_color: Color::WHITE,
        stroke_width: 2.0,
        opacity: 1.0,
        ..default()
    }
);
```

### Creating Grid
```rust
use math_objects::{create_grid, Style as MathStyle};

// Create grid
create_grid(
    &mut commands,
    1.0, // Grid spacing
    MathStyle {
        stroke_color: Color::srgba(0.3, 0.3, 0.3, 1.0),
        opacity: 0.3,
        ..default()
    }
);
```

## 🎨 Planned Core Features

### 1. Mathematical Objects (MathObjects)
- **Basic Shapes**: Circle, Line, Rectangle, Polygon
- **Function Graphs**: FunctionGraph, ParametricCurve, VectorField
- **3D Objects**: Sphere, Cube, Surface, Polyhedron
- **Coordinate Systems**: Axes, Grid, NumberLine, ComplexPlane

### 2. Animation System
- **Transform Animations**: Transform, Rotate, Scale, Fade
- **Path Animations**: Follow, DrawBoundingBox, Write
- **Group Animations**: AnimationGroup, Succession
- **Easing Functions**: Built-in and custom interpolation

### 3. Rendering Engine
- **Geometry Rendering**: High-performance 2D/3D graphics
- **Text Rendering**: LaTeX mathematical formula support
- **Material System**: Custom materials, gradient effects
- **Camera Control**: Multi-view, zoom and pan

### 4. Interactive Controls
- **Mouse Operations**: Drag, click, zoom
- **Keyboard Shortcuts**: Common operation bindings
- **Touch Support**: Mobile device compatibility

### 5. Scene Management
- **Scene Organization**: Multi-scene management, layer control
- **Timeline**: Keyframe animation, time control
- **State Management**: Undo/redo functionality

### 6. Export Features
- **Images**: High-quality PNG, SVG export
- **Animations**: GIF, MP4 video export
- **3D Models**: OBJ, STL format export

## 🛠️ Development Roadmap

### Phase 1: Foundation (Current)
- [x] Project structure setup
- [x] Core module definitions
- [x] Basic UI interface
- [x] Basic coordinate system
- [ ] Basic shape rendering
- [ ] Simple interaction system

### Phase 2: Core Features (v0.2.0)
- [ ] Interactive coordinate control (drag, zoom)
- [ ] Basic geometric shapes (circle, line, rectangle)
- [ ] Simple function graph plotting
- [ ] Color theme system

### Phase 3: Advanced Features (v0.3.0)
- [ ] Animation system foundation
- [ ] Image export functionality
- [ ] Scene save/load
- [ ] Extended mathematical function support

### Phase 4: Full Release (v1.0.0)
- [ ] Complete animation creation tools
- [ ] Video export functionality
- [ ] Plugin system
- [ ] LaTeX rendering support

## 🔧 Tech Stack

- **Core Engine**: Bevy 0.16.1
- **UI Framework**: bevy_egui
- **Mathematics**: nalgebra
- **Geometry Rendering**: lyon
- **Serialization**: serde
- **Error Handling**: anyhow, thiserror

## 🎯 Design Philosophy

### Inspired by Manim's Strengths
- **Mathematical Professionalism**: Designed specifically for mathematical visualization
- **Animation Elegance**: Smooth and natural mathematical animations
- **API Design**: Intuitive mathematical object API
- **High-Quality Output**: Perfect for teaching and presentations

### Enhanced by Bevy's Advantages
- **High Performance**: Rust + ECS architecture performance benefits
- **Real-time Interaction**: Game engine-level real-time rendering
- **Cross-platform**: Desktop, mobile, and web support
- **Modern Architecture**: Modular and extensible design

## 🤝 Contributing

We welcome contributions of all kinds:
- 🐛 Bug reports
- 💡 Feature suggestions
- 📝 Documentation improvements
- 🔧 Code contributions

Please see [DEVELOPMENT.md](DEVELOPMENT.md) for development guidelines.

## 📄 License

This project is dual-licensed under MIT or Apache 2.0. See LICENSE files for details.

## 🙏 Acknowledgments

- [Bevy](https://bevyengine.org/) - Excellent Rust game engine
- [Manim](https://www.manim.community/) - Inspiration for mathematical animation
- [egui](https://github.com/emilk/egui) - Outstanding immediate mode GUI library

---

**Making mathematics more beautiful, making learning more engaging!** ✨

## 📸 Screenshots

*Screenshots will be added as the project develops*

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=your-username/rim&type=Date)](https://star-history.com/#your-username/rim&Date)

---

**Note**: This project is in early development stage. APIs may change significantly. 