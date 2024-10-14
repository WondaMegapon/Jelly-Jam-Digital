use macroquad::prelude::*;
use crate::GameState; // Import GameState from main.rs

// Function to render the rules screen
pub fn draw_rules(rules_texture: &Texture2D, state: &mut GameState) {
    // Clear the background to avoid flickering
    clear_background(Color::new(0.0, 0.4, 0.3, 1.0));

    // Draw the rules texture
    draw_texture(*rules_texture, 0.0, 0.0, WHITE);

    // Draw a simple "Back" button
    draw_rectangle(10.0, 10.0, 100.0, 50.0, RED);
    draw_text("Back", 30.0, 40.0, 20.0, WHITE);

    // Handle the "Back" button to go back to the Menu
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;
        if mouse_x > 10.0 && mouse_x < 110.0 && mouse_y > 10.0 && mouse_y < 60.0 {
            *state = GameState::Menu; // Switch back to Menu
        }
    }
}

