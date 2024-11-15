use std::collections::HashMap;
use macroquad::prelude::*;
use rand::Rng;
use crate::{Card, CardEffect, Deck, GameState, TurnState};

pub async fn multiplay_loop(
    player1_hand: &mut Vec<Card>,
    player2_hand: &mut Vec<Card>,
    card_textures: &HashMap<CardEffect, Texture2D>,
) -> GameState {
    // Initialize player decks
    let mut player1_deck = Vec::new();
    let mut player2_deck = Vec::new();

    // Initialize player play areas
    let mut player1_play_area = Vec::new();
    let mut player2_play_area = Vec::new();

    // Initialize loot pile
    let mut loot_pile = Vec::new();

    // Initialize player scores
    let mut player1_score = 0;
    let mut player2_score = 0;

    // Initialize turn state
    let mut turn_state = TurnState::Player1;

    // Main game loop
    loop {
        // Clear the screen each frame
        clear_background(BLACK);

        // Check if a player has won
        if player1_score == 3 || player2_score == 3 {
            // Display the winner and reset the game
            let winner = if player1_score == 3 { "Player 1" } else { "Player 2" };
            draw_text_ex(
                &format!("{} wins!", winner),
                screen_width() / 2.0 - 50.0,
                screen_height() / 2.0,
                TextParams {
                    font: Font::default(),
                    font_size: 40,
                    color: WHITE,
                    ..Default::default()
                },
            );
            if is_key_pressed(KeyCode::Space) {
                // Reset the game
                player1_hand.clear();
                player2_hand.clear();
                player1_deck.clear();
                player2_deck.clear();
                player1_play_area.clear();
                player2_play_area.clear();
                loot_pile.clear();
                player1_score = 0;
                player2_score = 0;
                turn_state = TurnState::Player1;
                return GameState::Menu;
            }
        }

        // Draw player hands
        draw_hand(player1_hand, 50.0, 50.0, card_textures);
        draw_hand(player2_hand, 50.0, screen_height() - 150.0, card_textures);

        // Draw player play areas
        draw_play_area(&player1_play_area, 200.0, 50.0, card_textures);
        draw_play_area(&player2_play_area, 200.0, screen_height() - 150.0, card_textures);

        // Handle player turns
        match turn_state {
            TurnState::Player1 => {
                // Player 1's turn
                if player1_play_area.len() < 2 {
                    // Player 1 places cards
                    if is_key_pressed(KeyCode::A) {
                        if let Some(card) = player1_hand.pop() {
                            player1_play_area.push(card);
                        }
                    }
                } else {
                    // Player 1 takes turn
                    player_turn(player1_hand, &mut player1_play_area, &mut player2_play_area, &loot_pile, card_textures).await;
                    turn_state = TurnState::Player2;
                }
            }
            TurnState::Player2 => {
                // Player 2's turn
                if player2_play_area.len() < 2 {
                    // Player 2 places cards
                    if is_key_pressed(KeyCode::L) {
                        if let Some(card) = player2_hand.pop() {
                            player2_play_area.push(card);
                        }
                    }
                } else {
                    // Player 2 takes turn
                    player_turn(player2_hand, &mut player2_play_area, &mut player1_play_area, &loot_pile, card_textures).await;
                    turn_state = TurnState::Player1;
                }
            }
        }

        // Synchronize the frame
        next_frame().await;
    }
}

async fn player_turn(
    hand: &mut Vec<Card>,
    play_area: &mut Vec<Card>,
    opponent_play_area: &mut Vec<Card>,
    loot_pile: &Vec<Card>,
    card_textures: &HashMap<CardEffect, Texture2D>,
) {
    // Player draws a card
    if hand.len() < 8 {
        if let Some(card) = loot_pile.pop() {
            hand.push(card);
        }
    }

    // Player can choose to attack, use item, or withdraw cards
    // For simplicity, we'll just handle attacks for now
    if is_key_pressed(KeyCode::Space) {
        if let Some(attacker) = play_area.pop() {
            if let Some(defender) = opponent_play_area.pop() {
                crate::resolve_attack(&mut attacker, &mut defender);
                if defender.health > 0 {
                    opponent_play_area.push(defender);
                }
                play_area.push(attacker);
            }
        }
    }

    // Check if the player has no cards left
    if play_area.is_empty() {
        // Player claims a card from the center
        if let Some(card) = loot_pile.pop() {
            hand.push(card);
        }
    }
}

fn draw_hand(hand: &Vec<Card>, x: f32, y: f32, card_textures: &HashMap<CardEffect, Texture2D>) {
    for (i, card) in hand.iter().enumerate() {
        if let Some(texture) = card_textures.get(&card.base_effects <sup> </sup>) {
            draw_texture_ex(
                *texture,
                x + (i as f32 * 100.0),
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(100.0, 150.0)),
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_play_area(play_area: &Vec<Card>, x: f32, y: f32, card_textures: &HashMap<CardEffect, Texture2D>) {
    for (i, card) in play_area.iter().enumerate() {
        if let Some(texture) = card_textures.get(&card.base_effects <sup> </sup>) {
            draw_texture_ex(
                *texture,
                x + (i as f32 * 100.0),
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(100.0, 150.0)),
                    ..Default::default()
                },
            );
        }
    }
}