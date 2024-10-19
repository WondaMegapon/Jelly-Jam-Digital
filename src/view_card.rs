use macroquad::prelude::*;
use crate::GameState; // Import GameState from main.rs

pub async fn draw_view_cards(state: &mut GameState) {
    // Clear the background for the View Cards screen
    clear_background(BLACK);

    // Define paths to card images
    let card_paths = vec![
        "assets/cards/card1.png",
        "assets/cards/card2.png",
        "assets/cards/card3.png",
        "assets/cards/card4.png",
        "assets/cards/card5.png",
        // Add more card paths as needed
    ];

    let card_textures: Vec<Texture2D> = card_paths
        .iter()
        .map(|path| {
            load_texture(path)
                .await
                .expect(&format!("Failed to load texture from {}", path))
        })
        .collect();

    // Grid layout settings
    let cols = 3; // Number of columns
    let rows = (card_textures.len() + cols - 1) / cols; // Calculate rows needed
    let card_width = 100.0; // Width of each card
    let card_height = 150.0; // Height of each card
    let spacing = 20.0; // Space between cards

    // Calculate starting position
    let start_x = (screen_width() - (cols as f32 * (card_width + spacing))) / 2.0;
    let start_y = 100.0; // Starting Y position

    // Draw each card in the grid
    for (i, texture) in card_textures.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;

        let x = start_x + col as f32 * (card_width + spacing);
        let y = start_y + row as f32 * (card_height + spacing);

        // Draw the card texture
        draw_texture(*texture, x, y, WHITE);
    }

    // Draw a simple "Back" button
    draw_rectangle(10.0, 10.0, 100.0, 50.0, RED);
    draw_text("Back", 30.0, 40.0, 20.0, WHITE);

    // Handle the "Back" button to go back to the Rules state
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;
        if mouse_x > 10.0 && mouse_x < 110.0 && mouse_y > 10.0 && mouse_y < 60.0 {
            *state = GameState::Rules; // Switch back to Rules
        }
    }
}
