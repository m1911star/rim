use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MathAnimation>()
            .add_systems(Update, update_animations);
    }
}

/// 数学动画组件
#[derive(Component, Reflect, Clone)]
pub struct MathAnimation {
    pub duration: f32,
    pub elapsed: f32,
    pub is_playing: bool,
    pub loop_animation: bool,
}

impl Default for MathAnimation {
    fn default() -> Self {
        Self {
            duration: 1.0,
            elapsed: 0.0,
            is_playing: false,
            loop_animation: false,
        }
    }
}

/// 动画类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType {
    Transform,
    Fade,
    Draw,
    Write,
    Morph,
}

/// 更新动画的系统
fn update_animations(mut query: Query<&mut MathAnimation>, time: Res<Time>) {
    for mut animation in query.iter_mut() {
        if animation.is_playing {
            animation.elapsed += time.delta_secs();

            if animation.elapsed >= animation.duration {
                if animation.loop_animation {
                    animation.elapsed = 0.0;
                } else {
                    animation.is_playing = false;
                }
            }
        }
    }
}
