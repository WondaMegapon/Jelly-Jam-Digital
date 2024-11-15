mod player_turn_alert;
mod rules;
mod view_card;
mod view_creature;
mod view_item;
mod view_jelly;
mod view_mutation;
mod multiplay;

use macroquad::prelude::*;
use player_turn_alert::AlertState;
use rand::Rng;
use std::collections::HashMap;
use std::process;

#[macroquad::main("Jelly Jam")]
async fn main() {
    let mut state = GameState::Menu;
    let mut turn_state = AlertState::Player1;

    // Load textures at the start
    let background_texture = Texture2D::from_file_with_format(include_bytes!("../assets/title/title.png"), None);
    let rules_texture = Texture2D::from_file_with_format(include_bytes!("../assets/rules/rules.png"), None);

    // Load card textures using the view_cards modules - fixed with unique variables
    let creature_textures = view_creature::load_card_creature().await;
    let item_textures = view_item::load_card_item().await;
    let jelly_textures = view_jelly::load_card_jelly().await;
    let mutation_textures = view_mutation::load_card_mutation().await;

    // Create a HashMap to store all card textures
    let mut card_textures: HashMap<CardEffect, Texture2D> = HashMap::new();

    // Add creature textures to the HashMap
    for (effect, texture) in creature_textures.iter() {
        card_textures.insert(*effect, texture.clone());
    }

    // Add item textures to the HashMap
    for (effect, texture) in item_textures.iter() {
        card_textures.insert(*effect, texture.clone());
    }

    // Add jelly textures to the HashMap
    for (effect, texture) in jelly_textures.iter() {
        card_textures.insert(*effect, texture.clone());
    }

    // Add mutation textures to the HashMap
    for (effect, texture) in mutation_textures.iter() {
        card_textures.insert(*effect, texture.clone());
    }

    // Initialize player hands
    let mut player1_hand = vec![];
    let mut player2_hand = vec![];

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
                                    turn_state = AlertState::Player1;
                                }
                                "Rules" => {
                                    println!("Rules button clicked!");
                                    state = GameState::Rules; // Switch to the Rules state
                                }
                                "Settings" => println!("Settings button clicked!"),
                                "Quit" => {
                                    println!("Quit button clicked!");
                                    process::exit(0);
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
                state = multiplay::multiplay_loop(&mut player1_hand, &mut player2_hand, &card_textures).await;
            }
            GameState::TurnAlert => {
                // Handle the player turn state
                turn_state = player_turn_alert::player_turn_alert(turn_state).await;

                // After the turn screen is complete, switch to the next game state
                // For now, we'll just alternate between players
                turn_state = match turn_state {
                    AlertState::Player1 => AlertState::Player2,
                    AlertState::Player2 => AlertState::Player1,
            };
        }
    }

        // Synchronize the frame
        next_frame().await;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Rules,
    ViewCards,
    ViewCreature,
    ViewItem,
    ViewJelly,
    ViewMutation,
    PlayerTurn,
    TurnAlert,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    Player1,
    Player2,
}

#[derive(Debug, Clone)]
pub struct Card {
    deck: Deck,
    health: u32,
    damage: u32,
    defense: u32,
    base_effects: Vec<CardEffect>,
    modifier_slots: u32,
    modifier_effects: Vec<CardEffect>,
}

#[derive(Debug, Clone, Copy)]
pub enum Deck {
    Jelly,
    Creature,
    Mutation,
    Item,
}

#[derive(Debug, Clone, Copy)]
pub enum CardEffect {
    Zor,
    Oodalah,
    Rock,
    RogueJellies,
    Torble,
    Jammie,
    Slime,
    Jim,
    Kibble,
    Taki,
    Paolarm,
    GooberFruit,
    JellyJabber,
    JellyJail,
    Angelly,
    PowderJelly,
    SharpStick,
    Shield,
    Onedesix,
    StickySnatcher,
    NabNet,
    Brusier,
    Spicy,
    Shelly,
    Flutter,
    Jambler,
    Jumper,
    Gum,
    Chilli,
    Junior,
    Sling,
    PitifulGaze,
    Strong,
    Tough,
    Fast,
    Sucker,
    Armor,
    Willpower,
    Icebreaker,
    Hazardous,
    Super,
}

impl Card {
    pub fn apply_effect(&mut self, effect: CardEffect) {
        // Apply the effect to the card
        match effect {
            CardEffect::Zor => self.damage = 0, // Zor cannot attack until it takes damage
            CardEffect::Oodalah => self.health = 0, // Oodalah one-hit kill
            CardEffect::Rock => {} // Rock bounces back failed attacks
            CardEffect::RogueJellies => self.damage = self.health, // Rogue Jellies damage = health
            CardEffect::Torble => {} // Torble replaces Jellies/Creatures with new ones
            CardEffect::Jammie => {} // Jammie steals a card
            CardEffect::Slime => {} // Slime gains health on attack
            CardEffect::Jim => {} // Jim can attach mutations
            CardEffect::Kibble => self.damage -= 2, // Kibble reduces its own attack
            CardEffect::Taki => {} // Taki boosts a stat
            CardEffect::Paolarm => {} // Pao-larm can only target immobile Jellies/Creatures
            CardEffect::GooberFruit => self.health += 2, // Goober Fruit restores health
            CardEffect::JellyJabber => {} // Jelly Jabber steals a mutation
            CardEffect::JellyJail => {} // Jelly Jail restricts attacks
            CardEffect::Angelly => self.health = self.max_health, // Angelly restores health
            CardEffect::PowderJelly => {} // Powder Jelly deals damage to all
            CardEffect::SharpStick => {} // Sharp Stick doubles damage next attack
            CardEffect::Shield => {} // Shield prevents damage
            CardEffect::Onedesix => {} // Onedesix rerolls a failed attack
            CardEffect::StickySnatcher => {} // Sticky Snatcher steals an item
            CardEffect::NabNet => {} // Nab Net adds discarded card to hand
            CardEffect::Brusier => {} // Bruiser boosts damage at low health
            CardEffect::Spicy => {} // Spicy deals damage back when hit
            CardEffect::Shelly => self.defense += 1, // Shelly reduces damage
            CardEffect::Flutter => {} // Flutter boosts attack roll
            CardEffect::Jambler => self.health = rand::thread_rng().gen_range(1..=6), // Jambler rolls health
            CardEffect::Jumper => {} // Jumper sends attacked card back to hand
            CardEffect::Gum => {} // Gum mimics next card
            CardEffect::Chilli => {} // Chilli freezes attackers
            CardEffect::Junior => {} // Junior can attach mutations
            CardEffect::Sling => {} // Sling ignores Hazardous
            CardEffect::PitifulGaze => self.defense += 1, // Pitiful Gaze boosts defense
            CardEffect::Strong => self.damage += 1, // Strong boosts damage
            CardEffect::Tough => self.health += 1, // Tough boosts health
            CardEffect::Fast => {} // Fast boosts attack roll
            CardEffect::Sucker => {} // Sucker gains health for damage dealt
            CardEffect::Armor => self.defense += 1, // Armor reduces damage
            CardEffect::Willpower => {} // Willpower prevents discard
            CardEffect::Icebreaker => {} // Icebreaker ignores Freeze and Jelly Jail
            CardEffect::Hazardous => {} // Hazardous deals damage back when hit
            CardEffect::Super => {} // Super boosts stats on Jelly discard
        }
    }
}

pub fn resolve_attack(attacker: &mut Card, defender: &mut Card) {
    // Roll a dice to determine if the attack is successful
    let attack_roll = rand::thread_rng().gen_range(1..=6);
    if attack_roll >= defender.defense {
        // Attack is successful
        defender.health = defender.health.saturating_sub(attacker.damage);
        println!("Attack successful! Defender health: {}", defender.health);

        // Apply any effects from the attack
        for effect in &attacker.base_effects {
            match effect {
                CardEffect::Kibble => defender.damage -= 2, // Kibble reduces attack
                CardEffect::SharpStick => attacker.damage *= 2, // Sharp Stick doubles damage next attack
                _ => {}
            }
        }

        // Apply any effects from the defender
        for effect in &defender.base_effects {
            match effect {
                CardEffect::Spicy => attacker.health = attacker.health.saturating_sub(1), // Spicy deals damage back
                CardEffect::Shelly => defender.damage = defender.damage.min(1), // Shelly reduces damage
                CardEffect::Chilli => attacker.can_attack = false, // Chilli freezes attackers
                _ => {}
            }
        }
    } else {
        // Attack is unsuccessful
        println!("Attack failed!");
    }

    // Check if the defender is discarded
    if defender.health == 0 {
        println!("Defender discarded!");
        // Apply any effects from the defender's discard
        for effect in &defender.base_effects {
            match effect {
                CardEffect::Angelly => defender.health = defender.max_health, // Angelly restores health
                CardEffect::Willpower => {} // Willpower prevents discard
                _ => {}
            }
        }
    }
}