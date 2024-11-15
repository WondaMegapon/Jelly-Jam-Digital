use macroquad::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq)]
pub enum TurnState {
    Player1,
    Player2,
}

// Initialize Rodio audio
pub struct PlayerTurnAudio {
    sink: Arc<Sink>,
    audio_data: Vec<u8>,
    _stream: OutputStream, // Ensure the audio stream persists
}

impl PlayerTurnAudio {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to initialize audio stream");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

        // Include the audio file directly in the binary
        let audio_data = include_bytes!("../assets/sounds/music/Player_Change.mp3").to_vec();

        Self {
            sink: Arc::new(sink),
            audio_data,
            _stream, // Hold the stream to prevent dropping
        }
    }

    pub fn play(&self) {
        let cursor = Cursor::new(self.audio_data.clone());
        if let Ok(source) = Decoder::new(cursor) {
            self.sink.append(source);
            self.sink.play();
        } else {
            println!("Failed to decode audio");
        }
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    // Manually restart the music if it ends
    pub fn loop_audio(&self) {
        if self.sink.empty() {
            self.play(); // Restart the music if it has ended
        }
    }
}

// Function to display the player turn screen
pub async fn player_turn_screen(turn: TurnState, player1: &mut Player, player2: &mut Player, deck: &mut Deck) -> TurnState {
    // Load audio
    let audio = PlayerTurnAudio::new();
    audio.play();

    let mut selected_card: Option<usize> = None;

    loop {
        // Manually restart audio if it ends
        audio.loop_audio();

        // Clear the background for the turn screen
        clear_background(BLACK);

        // Set the turn message and color
        let (text, color) = match turn {
            TurnState::Player1 => ("Player 1's Turn", BLUE),
            TurnState::Player2 => ("Player 2's Turn", RED),
        };

        // Draw the turn message
        draw_text_ex(
            text,
            screen_width() / 2.0 - measure_text(text, None, 40, 1.0).width / 2.0,
            screen_height() / 2.0 - 20.0,
            TextParams {
                font_size: 40,
                color,
                ..Default::default()
            },
        );

        // Draw "Press any button"
        let prompt = "Press Any Button";
        draw_text_ex(
            prompt,
            screen_width() / 2.0 - measure_text(prompt, None, 30, 1.0).width / 2.0,
            screen_height() / 2.0 + 40.0,
            TextParams {
                font_size: 30,
                color: WHITE,
                ..Default::default()
            },
        );

        // Draw player's hand
        let player_hand = match turn {
            TurnState::Player1 => &mut player1.hand,
            TurnState::Player2 => &mut player2.hand,
        };

        let player_play_area = match turn {
            TurnState::Player1 => &mut player1.play_area,
            TurnState::Player2 => &mut player2.play_area,
        };

        // Draw cards in hand
        for (i, card) in player_hand.iter().enumerate() {
            let x = 50.0 + i as f32 * 100.0;
            let y = screen_height() - 150.0;
            draw_rectangle(x, y, 90.0, 140.0, GRAY);
            draw_text_ex(
                &card.name,
                x + 10.0,
                y + 10.0,
                TextParams {
                    font_size: 20,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }

        // Draw cards in play area
        for (i, card) in player_play_area.iter().enumerate() {
            let x = 50.0 + i as f32 * 100.0;
            let y = 50.0;
            draw_rectangle(x, y, 90.0, 140.0, GRAY);
            draw_text_ex(
                &card.name,
                x + 10.0,
                y + 10.0,
                TextParams {
                    font_size: 20,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }

        // Handle card selection
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_x = mouse_position().0;
            let mouse_y = mouse_position().1;

            // Check if a card in hand is selected
            for (i, card) in player_hand.iter().enumerate() {
                let x = 50.0 + i as f32 * 100.0;
                let y = screen_height() - 150.0;
                if mouse_x > x && mouse_x < x + 90.0 && mouse_y > y && mouse_y < y + 140.0 {
                    selected_card = Some(i);
                    break;
                }
            }

            // Check if a card in play area is selected
            for (i, card) in player_play_area.iter().enumerate() {
                let x = 50.0 + i as f32 * 100.0;
                let y = 50.0;
                if mouse_x > x && mouse_x < x + 90.0 && mouse_y > y && mouse_y < y + 140.0 {
                    // Handle battle logic here
                    let opponent_play_area = match turn {
                        TurnState::Player1 => &mut player2.play_area,
                        TurnState::Player2 => &mut player1.play_area,
                    };

                    if let Some(selected_index) = selected_card {
                        let attacker = &player_play_area[selected_index];
                        let mut defender_index = None;

                        for (j, defender) in opponent_play_area.iter().enumerate() {
                            // Simple attack logic: roll a dice to determine success
                            let attack_roll = rand::thread_rng().gen_range(1..=6);
                            if attack_roll >= defender.def {
                                defender.hp -= attacker.dmg;
                                if defender.hp <= 0 {
                                    println!("{} defeated {}!", attacker.name, defender.name);
                                    defender_index = Some(j);
                                } else {
                                    println!("{} attacked {} for {} damage!", attacker.name, defender.name, attacker.dmg);
                                }
                            } else {
                                println!("{} attack on {} failed!", attacker.name, defender.name);
                            }
                        }

                        if let Some(defender_index) = defender_index {
                            opponent_play_area.remove(defender_index);
                        }

                        selected_card = None;
                    }
                    break;
                }
            }
        }

        // Handle drawing cards
        if is_key_pressed(KeyCode::D) && player_hand.len() < 8 {
            if let Some(card) = deck.draw() {
                player_hand.push(card);
            }
        }

        // If any key is pressed, proceed to main gameplay loop
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::A) {
            audio.stop(); // Stop the music
            return turn;
        }

        next_frame().await;
    }
}