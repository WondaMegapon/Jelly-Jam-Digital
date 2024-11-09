mod rules;
mod view_card;
mod view_creature;
mod view_item;
mod view_jelly;
mod view_mutation;
mod player_turn;
use player_turn::TurnState;
use macroquad::prelude::*;

#[macroquad::main("Jelly Jam")]
async fn main() {
    let mut state = GameState::Menu;
    let mut turn_state = TurnState::Player1;

    // Load textures at the start
    let background_texture =
        Texture2D::from_file_with_format(include_bytes!("../assets/title/title.png"), None);

    let rules_texture =
        Texture2D::from_file_with_format(include_bytes!("../assets/rules/rules.png"), None);

    // Load card textures using the view_cards modules - fixed with unique variables
    let creature_textures = view_creature::load_card_creature().await;
    let item_textures = view_item::load_card_item().await;
    let jelly_textures = view_jelly::load_card_jelly().await;
    let mutation_textures = view_mutation::load_card_mutation().await;

    // Main game loop
    loop {
        // Clear the screen each frame
        clear_background(BLACK);

        // Match the game state and draw the relevant screen
        match state {
            GameState::Menu => {
                // Draw the background texture for the main menu with scaling to fit the screen
                let texture_width = background_texture.width();
                let texture_height = background_texture.height();

                // Calculate the scaling factor to fit the texture to the screen
                let _scale_x = screen_width() / texture_width;
                let _scale_y = screen_height() / texture_height;

                draw_texture_ex(
                    background_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(screen_width(), screen_height())), // Stretch to fit the screen
                        ..Default::default()
                    },
                );

                // Create and draw buttons, scaling them to fit the screen
                let buttons = vec![
                    ("Single Play", GREEN),
                    ("Multi Play", SKYBLUE),
                    ("Rules", VIOLET),
                    ("Settings", ORANGE),
                    ("Quit", RED),
                ];

                let button_width = screen_width() * 0.6; // Adjust button width relative to screen
                let button_height = screen_height() * 0.1; // Adjust button height relative to screen
                let button_spacing = screen_height() * 0.008; // Adjust spacing relative to screen height

                for (i, (label, color)) in buttons.iter().enumerate() {
                    let y = screen_height() * 0.45 + (i as f32 * (button_height + button_spacing));

                    // Draw button background
                    draw_rectangle(screen_width() * 0.2, y, button_width, button_height, *color);

                    // Draw button text, centering it within the button
                    draw_text_ex(
                        label,
                        screen_width() * 0.35,
                        y + button_height / 2.0 + 10.0,
                        TextParams {
                            font: Font::default(),
                            font_size: (screen_height() * 0.05) as u16, // Adjust text size relative to screen
                            color: WHITE,
                            ..Default::default()
                        },
                    );

                    // Handle button clicks
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mouse_x = mouse_position().0;
                        let mouse_y = mouse_position().1;

                        if mouse_x > screen_width() * 0.2
                            && mouse_x < screen_width() * 0.2 + button_width
                            && mouse_y > y
                            && mouse_y < y + button_height
                        {
                            match *label {
                                "Single Play" => println!("Single Play button clicked!"),
                                "Multi Play" => {
                                    println!("Multi Play button clicked!");
                                    state = GameState::PlayerTurn; // Start main multiplayer loop
                                    turn_state = TurnState::Player1;
                                } 
                                "Rules" => {
                                    println!("Rules button clicked!");
                                    state = GameState::Rules; // Switch to the Rules state
                                }
                                "Settings" => println!("Settings button clicked!"),
                                "Quit" => {
                                    println!("Quit button clicked!");
                                    std::process::exit(0);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            GameState::Rules => {
                // Call the draw_rules function from the rules module, passing the texture and state
                rules::draw_rules(&rules_texture, &mut state);
            }
            GameState::ViewCards => {
                // Draw the buttons to view card types
                view_card::view_card_types(&mut state);
            }
            GameState::ViewCreature => {
                // Draw the view cards screen with correct textures
                view_creature::draw_view_creature(&creature_textures, &mut state);
            }
            GameState::ViewItem => {
                // Draw the view cards screen with correct textures
                view_item::draw_view_item(&item_textures, &mut state);
            }
            GameState::ViewJelly => {
                // Draw the view cards screen with correct textures
                view_jelly::draw_view_jelly(&jelly_textures, &mut state);
            }
            GameState::ViewMutation => {
                // Draw the view cards screen with correct textures
                view_mutation::draw_view_mutation(&mutation_textures, &mut state);
            }
            GameState::PlayerTurn => {
                // Handle the player turn state
                turn_state = player_turn::player_turn_screen(turn_state).await;
                
                // After the turn screen is complete, switch to the next game state
                // For now, we'll just alternate between players
                turn_state = match turn_state {
                    TurnState::Player1 => TurnState::Player2,
                    TurnState::Player2 => TurnState::Player1,
                };
            }
        }

        // Synchronize the frame
        next_frame().await;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Rules,
    ViewCards,
    ViewCreature,
    ViewItem,
    ViewJelly,
    ViewMutation,
    PlayerTurn,
    
}