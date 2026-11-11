use chaikin::{ChaikinState, draw_control_point_circles, draw_lines, draw_points};
use macroquad::prelude::*;

#[macroquad::main("RAID-2 - CHAIKIN")]
async fn main() {
    let mut state = ChaikinState::new();
    let mut mouse_pressed_last_frame = false;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Enter) {
            state.start_animation();
        }

        if is_key_pressed(KeyCode::C) {
            state.clear();
        }

        let mouse_pos = mouse_position();
        let mouse_pressed = is_mouse_button_down(MouseButton::Left);
        let mouse_just_pressed = mouse_pressed && !mouse_pressed_last_frame;

        if mouse_just_pressed {
            if !state.animation_running {
                state.add_point(mouse_pos.0, mouse_pos.1);
            }
        }

        mouse_pressed_last_frame = mouse_pressed;
        state.update();

        clear_background(BLACK);

        if state.current_points.len() > 1 {
            draw_lines(&state.current_points, WHITE, 2.0);
        }

        if !state.control_points.is_empty() {
            draw_control_point_circles(&state.control_points);
        }

        if state.animation_running && state.current_points.len() > 2 {
            draw_points(&state.current_points, GREEN, 2.0);
        }

        next_frame().await;
    }
}
