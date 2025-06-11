use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_mouse_input, handle_keyboard_input));
    }
}

/// 处理鼠标输入的系统
fn handle_mouse_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    // 处理鼠标点击
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // 处理左键点击
    }

    // 处理鼠标移动
    for event in cursor_moved_events.read() {
        // 处理鼠标移动
        let _cursor_position = event.position;
    }

    // 处理滚轮
    for event in mouse_wheel_events.read() {
        // 处理缩放
        let _scroll_delta = event.y;
    }
}

/// 处理键盘输入的系统
fn handle_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // 播放/暂停动画
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            // 保存场景
        }
        if keyboard_input.just_pressed(KeyCode::KeyO) {
            // 打开场景
        }
    }
}
