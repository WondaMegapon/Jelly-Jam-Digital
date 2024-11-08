use crate::GameState;
use macroquad::prelude::*;
use std::fs;

// Function to load card textures
pub async fn load_card_creature() -> Vec<Texture2D> {
    let mut creature_textures = Vec::new();

    let paths = fs::read_dir("./assets/cards/creature/").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap_or_default() == "png" {
            // Load texture asynchronously
            let texture = load_texture(path.to_str().unwrap()).await.unwrap();
            creature_textures.push(texture);
        }
    }

    creature_textures
}

// Function to render the view cards screen
pub fn draw_view_creature(creature_textures: &[Texture2D], state: &mut GameState) {
    clear_background(BLACK);

    let screen_width = screen_width();
    let screen_height = screen_height();
    let spacing = 20.0;

    // Set a maximum grid width to prevent the grid from expanding too much
    let max_grid_width = screen_width.min(800.0); // Cap width at 800 or screen width, whichever is smaller
    let card_aspect_ratio = 1.5;
    let total_cards = creature_textures.len();

    // Calculate the number of columns based on the capped grid width
    let cols = (max_grid_width / (max_grid_width / 4.0 + spacing)).round() as usize;
    let cols = cols.min(total_cards); // Ensure we don't create more columns than needed
    let rows = ((total_cards as f32 / cols as f32).ceil()) as usize;

    // Calculate the optimal card width and height within the max grid width
    let card_width = (max_grid_width - spacing * (cols as f32 + 1.0)) / cols as f32;
    let card_height = card_width * card_aspect_ratio;

    // Center the grid within the screen
    let start_x = (screen_width - (cols as f32 * (card_width + spacing) - spacing)) / 2.0;
    let start_y = (screen_height - (rows as f32 * (card_height + spacing) - spacing)) / 2.0;

    // Draw each card in the grid
    for (i, texture) in creature_textures.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;

        let x = start_x + col as f32 * (card_width + spacing);
        let y = start_y + row as f32 * (card_height + spacing);

        draw_texture_ex(
            *texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(card_width, card_height)),
                ..Default::default()
            },
        );
    }

    // Draw "Back" button
    draw_rectangle(10.0, 10.0, 100.0, 50.0, RED);
    draw_text("Back", 30.0, 40.0, 20.0, WHITE);

    // Handle "Back" button click
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        if mouse_x > 10.0 && mouse_x < 110.0 && mouse_y > 10.0 && mouse_y < 60.0 {
            *state = GameState::ViewCards;
        }
    }
}

