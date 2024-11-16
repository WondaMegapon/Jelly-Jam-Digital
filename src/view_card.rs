use crate::GameState;
use macroquad::prelude::*;

pub fn view_card_types(state: &mut GameState) {
    // Clear the background
    clear_background(Color::new(0.0, 0.4, 0.3, 1.0));

    // Draw the "Back" button
    draw_rectangle(10.0, 10.0, 100.0, 50.0, RED);
    draw_text("Back", 30.0, 40.0, 20.0, WHITE);

    // Grid layout configuration
    let button_width = 350.0;
    let button_height = 250.0;
    let padding = 20.0;

    // Calculate grid position to center it on screen
    let grid_width = (button_width * 2.0) + padding;
    let grid_height = (button_height * 2.0) + padding;
    let start_x = (screen_width() - grid_width) / 2.0;
    let start_y = (screen_height() - grid_height) / 2.0;

    // Define button positions and properties with new colors
    let buttons = [
        // Creatures - Blue
        (
            start_x,
            start_y,
            "View Creature",
            GameState::ViewCreature,
            Color::new(1.0, 0.0, 0.0, 1.0),
        ),
        // Items - Yellow
        (
            start_x + button_width + padding,
            start_y,
            "View Item",
            GameState::ViewItem,
            Color::new(1.0, 1.0, 0.0, 1.0),
        ),
        // Jellies - Teal
        (
            start_x,
            start_y + button_height + padding,
            "View Jelly",
            GameState::ViewJelly,
            Color::new(0.0, 0.8, 0.8, 1.0),
        ),
        // Mutations - Red
        (
            start_x + button_width + padding,
            start_y + button_height + padding,
            "View Mutation",
            GameState::ViewMutation,
            Color::new(0.0, 0.0, 1.0, 1.0),
        ),
    ];

    // Draw buttons and handle clicks
    let mouse_pos = mouse_position();

    for (x, y, text, next_state, color) in buttons.iter() {
        // Draw button
        draw_rectangle(*x, *y, button_width, button_height, *color);

        // Center text in button
        let text_size = 60.0;
        let text_width = measure_text(text, None, text_size as u16, 1.0).width;
        let text_x = x + (button_width - text_width) / 2.0;
        let text_y = y + (button_height + text_size) / 2.0;

        // Button text color
        let text_color = BLACK;

        draw_text(text, text_x, text_y, text_size, text_color);

        // Handle click
        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_pos.0 >= *x
                && mouse_pos.0 <= x + button_width
                && mouse_pos.1 >= *y
                && mouse_pos.1 <= y + button_height
            {
                *state = next_state.clone();
            }
        }
    }

    // Handle back button click
    if is_mouse_button_pressed(MouseButton::Left) {
        if mouse_pos.0 > 10.0 && mouse_pos.0 < 110.0 && mouse_pos.1 > 10.0 && mouse_pos.1 < 60.0 {
            *state = GameState::Menu;
        }
    }
}
