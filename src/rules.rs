use macroquad::prelude::*;
use crate::GameState;

// Function to render the rules screen
pub fn draw_rules(rules_texture: &Texture2D, state: &mut GameState) {
    // Clear the background to avoid flickering
    clear_background(Color::new(0.0, 0.4, 0.3, 1.0));

    // Draw the rules texture, stretching it to fit the screen
    draw_texture_ex(
        *rules_texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())), // Stretch to fit the screen
            ..Default::default()
        },
    );

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

    // Draw the "View Cards" button in the bottom right corner
    let button_width = 150.0;
    let button_height = 50.0;
    let button_x = screen_width() - button_width - 10.0; // 10 pixels from the right edge
    let button_y = screen_height() - button_height - 10.0; // 10 pixels from the bottom edge

    // Draw the button
    draw_rectangle(button_x, button_y, button_width, button_height, Color::from_rgba(128, 0, 128, 255)); // Purple color
    draw_text("View Cards", button_x + 10.0, button_y + 30.0, 20.0, WHITE); // Center the text

    // Handle the "View Cards" button click
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;
        if mouse_x > button_x && mouse_x < button_x + button_width && mouse_y > button_y && mouse_y < button_y + button_height {
            *state = GameState::ViewCards; // Switch to ViewCards state
        }
    }
}
