mod rules;

use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Rules,
}

#[macroquad::main("Jelly Jam")]
async fn main() {
    let mut state = GameState::Menu;

    // Load textures at the start
    let background_texture = load_texture("C:\\Users\\CraCr\\Documents\\Projects\\JellyJam\\jelly_jam\\assets\\title.png")
        .await
        .expect("Failed to load background texture. Make sure the file exists.");
    
    let rules_texture = load_texture("C:\\Users\\CraCr\\Documents\\Projects\\JellyJam\\jelly_jam\\assets\\rules.png")
        .await
        .expect("Failed to load rules texture. Make sure the file exists.");

    // Main game loop
    loop {
        // Clear the screen each frame
        clear_background(BLACK);

        // Match the game state and draw the relevant screen
        match state {
            GameState::Menu => {
                // Draw the background texture for the main menu
                draw_texture(background_texture, 0.0, 0.0, WHITE);

                let buttons = vec![
                    ("Single Play", GREEN),
                    ("Multi Play", SKYBLUE),
                    ("Rules", VIOLET),
                    ("Settings", ORANGE),
                    ("Quit", RED),
                ];

                let button_width = screen_width() * 0.5;
                let button_height = 80.0;
                let button_spacing = 10.0;

                for (i, (label, color)) in buttons.iter().enumerate() {
                    let y = screen_height() * 0.4 + (i as f32 * (button_height + button_spacing));

                    // Draw button background
                    draw_rectangle(screen_width() * 0.25, y, button_width, button_height, *color);

                    // Draw button text
                    draw_text_ex(
                        label,
                        screen_width() * 0.35,
                        y + button_height / 2.0 + 10.0,
                        TextParams {
                            font: Font::default(),
                            font_size: 40,
                            color: WHITE,
                            ..Default::default()
                        },
                    );

                    // Handle button clicks
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mouse_x = mouse_position().0;
                        let mouse_y = mouse_position().1;

                        if mouse_x > screen_width() * 0.25
                            && mouse_x < screen_width() * 0.25 + button_width
                            && mouse_y > y
                            && mouse_y < y + button_height
                        {
                            match *label {
                                "Single Play" => println!("Single Play button clicked!"),
                                "Multi Play" => println!("Multi Play button clicked!"),
                                "Rules" => {
                                    println!("Rules button clicked!");
                                    state = GameState::Rules;  // Switch to the Rules state
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
        }

        // Synchronize the frame
        next_frame().await;
    }
}
