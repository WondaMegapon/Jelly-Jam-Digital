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
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to initialize audio stream");
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
            log::info!("Failed to decode audio");
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
pub async fn player_turn_screen(turn: TurnState) -> TurnState {
    // Load audio
    let audio = PlayerTurnAudio::new();
    audio.play();

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

        // If any key is pressed, proceed to main gameplay loop
        if is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::A)
        {
            audio.stop(); // Stop the music
            return turn;
        }

        next_frame().await;
    }
}
