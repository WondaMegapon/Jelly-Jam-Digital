use crate::GameState;
use include_dir::include_dir;
use macroquad::prelude::*;

// Function to load card textures
pub async fn load_card_textures() -> Vec<Texture2D> {
    let card_paths = include_dir!("./assets/cards/");
    // let card_paths = vec![
    //     include_bytes!("../assets/cards/jelly/Junior_Jelly.png"),
    //     include_bytes!("../assets/cards/jelly/Sling_Jelly.png"),
    //     include_bytes!("../assets/cards/mutation/Super.png"),
    //     include_bytes!("../assets/cards/item/Nab_Net.png"),
    //     include_bytes!("../assets/cards/creature/Kibble.png"),
    //     include_bytes!("../assets/cards/creature/Taki.png"),
    //     // Add more card paths as needed
    // ];

    let mut card_textures = Vec::new();

    for directories in card_paths.dirs() {
        for path in directories.files() {
            if path.path().extension().unwrap().to_ascii_lowercase() == "png" {
                let texture = Texture2D::from_file_with_format(path.contents(), None);
                card_textures.push(texture);
            }
        }
    }

    card_textures
}

// Function to render the view cards screen
pub fn draw_view_cards(card_textures: &[Texture2D], state: &mut GameState) {
    // Clear the background for the View Cards screen
    clear_background(BLACK);

    // Grid layout settings
    let cols = 3; // Number of columns
    let spacing = 20.0; // Space between cards

    // Calculate card dimensions based on screen size
    let screen_width = screen_width();
    let card_width = (screen_width - (spacing * (cols as f32 + 1.0))) / cols as f32; // Width of each card
    let card_height = card_width * 1.5; // Maintain aspect ratio (1.5:1)

    // Calculate starting position
    let start_x = spacing; // Starting X position with spacing
    let start_y = 100.0; // Starting Y position

    // Draw each card in the grid
    for (i, texture) in card_textures.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;

        let x = start_x + col as f32 * (card_width + spacing);
        let y = start_y + row as f32 * (card_height + spacing);

        // Draw the card texture
        draw_texture_ex(
            *texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(card_width, card_height)), // Scale card to the calculated dimensions
                ..Default::default()
            },
        );
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
