mod rules;
mod view_card;
mod view_creature;
mod view_item;
mod view_jelly;
mod view_mutation;
mod player_turn;
use player_turn::TurnState;
use macroquad::prelude::*;
use rand::Rng; 
use rand::thread_rng; 
use std::fs;

#[macroquad::main("Jelly Jam")]
async fn main() {
    let mut state = GameState::Menu;
    let mut turn_state = TurnState::Player1;

    // Load textures at the start
    let background_texture = Texture2D::from_file_with_format(include_bytes!("../assets/title/title.png"), None);
    let rules_texture = Texture2D::from_file_with_format(include_bytes!("../assets/rules/rules.png"), None);

    // Load card textures using the view_cards modules - fixed with unique variables
    let creature_textures = view_creature::load_card_creature().await;
    let item_textures = view_item::load_card_item().await;
    let jelly_textures = view_jelly::load_card_jelly().await;
    let mutation_textures = view_mutation::load_card_mutation().await;

    let mut cards = initialize_cards();
    let mut deck = Deck::new(cards);
    deck.shuffle();

    let (mut player1, mut player2) = initialize_players(&mut deck);

    // Main game loop
    loop {
        clear_background(BLACK);

        match state {
            GameState::Menu => {
                // Draw the background texture for the main menu with scaling to fit the screen
                draw_texture_ex(
                    background_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(screen_width(), screen_height())),
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

                let button_width = screen_width() * 0.6;
                let button_height = screen_height() * 0.1;
                let button_spacing = screen_height() * 0.008;

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
                            font_size: (screen_height() * 0.05) as u16,
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
                rules::draw_rules(&rules_texture, &mut state);
            }
            GameState::ViewCards => {
                view_card::view_card_types(&mut state);
            }
            GameState::ViewCreature => {
                view_creature::draw_view_creature(&creature_textures, &mut state);
            }
            GameState::ViewItem => {
                view_item::draw_view_item(&item_textures, &mut state);
            }
            GameState::ViewJelly => {
                view_jelly::draw_view_jelly(&jelly_textures, &mut state);
            }
            GameState::ViewMutation => {
                view_mutation::draw_view_mutation(&mutation_textures, &mut state);
            }
            GameState::PlayerTurn => {
                // Handle the player turn state
                turn_state = player_turn::player_turn_screen(turn_state, &mut player1, &mut player2, &mut deck).await;

                // After the turn screen is complete, switch to the next game state
                turn_state = match turn_state {
                    TurnState::Player1 => TurnState::Player2,
                    TurnState::Player2 => TurnState::Player1,
                };
            }
        }

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

use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum CardType {
    Jelly,
    Creature,
    Item,
    Mutation,
}

#[derive(Debug, Clone)]
struct Card {
    name: String,
    card_type: CardType,
    hp: i32,
    dmg: i32,
    def: i32,
    // Add more fields as needed
}

#[derive(Debug, Clone)]
struct Player {
    hand: Vec<Card>,
    play_area: Vec<Card>,
    // Add more fields as needed
}

#[derive(Debug, Clone)]
struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    fn new(cards: Vec<Card>) -> Deck {
        let mut cards = cards.into_iter().collect::<VecDeque<_>>();
        cards.make_contiguous();
        Deck { cards }
    }

    fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }
}


async fn load_card_textures(directory: &str) -> Vec<Texture2D> {
    let paths = fs::read_dir(directory)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file() && entry.path().extension().unwrap_or_default() == "png")
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    let mut textures = Vec::new();
    for path in paths {
        let texture = Texture2D::from_file_with_format(&load_file(path.to_str().unwrap()).await.unwrap(), None);
        textures.push(texture);
    }
    textures
}

fn initialize_cards() -> Vec<Card> {
    vec![
        Card { name: "Bruiser_Jelly".to_string(), card_type: CardType::Jelly, hp: 2, dmg: 2, def: 3 },
        Card { name: "Spicy_Jelly".to_string(), card_type: CardType::Jelly, hp: 2, dmg: 2, def: 3 },
        Card { name: "Shelly_Jelly".to_string(), card_type: CardType::Jelly, hp: 2, dmg: 1, def: 2 },
        Card { name: "Flutter_Jelly".to_string(), card_type: CardType::Jelly, hp: 1, dmg: 1, def: 4 },
        Card { name: "The_Jambler".to_string(), card_type: CardType::Jelly, hp: 1, dmg: 1, def: 2 },
        Card { name: "Jumper_Jelly".to_string(), card_type: CardType::Jelly, hp: 1, dmg: 0, def: 4 },
        Card { name: "Gum_Jelly".to_string(), card_type: CardType::Jelly, hp: 3, dmg: 1, def: 2 },
        Card { name: "Chilli_Jelly".to_string(), card_type: CardType::Jelly, hp: 2, dmg: 1, def: 3 },
        Card { name: "Junior_Jelly".to_string(), card_type: CardType::Jelly, hp: 2, dmg: 1, def: 3 },
        Card { name: "Sling_Jelly".to_string(), card_type: CardType::Jelly, hp: 2, dmg: 1, def: 4 },
        // Add more cards as needed
    ]
}

fn initialize_players(deck: &mut Deck) -> (Player, Player) {
    let player1_hand: Vec<Card> = (0..8).map(|_| deck.draw().unwrap()).collect();
    let player2_hand: Vec<Card> = (0..8).map(|_| deck.draw().unwrap()).collect();

    let player1 = Player { hand: player1_hand, play_area: Vec::new() };
    let player2 = Player { hand: player2_hand, play_area: Vec::new() };

    (player1, player2)
}