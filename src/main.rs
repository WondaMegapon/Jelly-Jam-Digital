mod rules;
mod view_card;
use macroquad::prelude::*;

#[macroquad::main("Jelly Jam")]
async fn main() {
    let mut state = GameState::Menu;

    // Load textures at the start
    let background_texture =
        Texture2D::from_file_with_format(include_bytes!("../assets/title/title.png"), None);

    let rules_texture =
        Texture2D::from_file_with_format(include_bytes!("../assets/rules/rules.png"), None);

    // Load card textures using the view_card module
    let card_textures = view_card::load_card_textures().await;

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
                                "Multi Play" => println!("Multi Play button clicked!"),
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
                // Draw the view cards screen
                view_card::draw_view_cards(&card_textures, &mut state);
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Deck {
    Jelly,
    Creature,
    Mutation,
    Item,
}

// For storing each card.
#[derive(Debug, Clone)]
struct Card {
    // Stats
    deck: Deck,                      // The deck it's from.
    health: Option<u32>,             // The health (optional.)
    damage: Option<u32>,             // The damage (optional.)
    defense: Option<u32>,            // The defense (optional.)
    base_effects: CardName,          // The card's base effects.
    modifier_slots: u32,             // The amount of modifier slots.
    modifier_effects: Vec<CardName>, // The card's modified effects.
}

// So there's a big container for all effects on a card.
// The big container is broken into individual phases with vectors for every effect.
// Each phase has it's own iterator that is called when that phase occurs.
// Card is played. It's effect container OnPlay is triggered. Each item is matched and if it is a positive match, then perform that function.
// Effect order is resolve order.
// Attacks happen before damages. Discard happens after damage resolves.

#[derive(Debug, Clone, Copy)]
enum Phases {
    OnDraw,      // For when a card is drawn.
    OnDiscard,   // For when a card is discarded.
    OnBounce,    // For when a card is bounced back to its hand.
    OnTurnStart, // For when a turn starts.
    OnTurnEnd,   // For when a turn ends.
    OnEnter,     // For when a card is played (enters the field).
    OnExit,      // For when a card leaves (exits the field).
    OnAttack,    // For when a card attacks.
    OnDamaged,   // For when a card is damaged.
    OnAny,       // Occurs after each other game phase.
}

fn resolve_phase_for_card(current_phase: Phases, current_card: Card) {
    // For phase specific behavior.
    match current_phase {
        Phases::OnDraw => match current_card {
            _ => {
                println!("Card {:?} has no OnDraw effect.", current_card.base_effects);
            }
        },
        Phases::OnDiscard => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnDiscard effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnBounce => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnBounce effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnTurnStart => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnTurnStart effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnTurnEnd => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnTurnEnd effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnEnter => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnEnter effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnExit => match current_card {
            _ => {
                println!("Card {:?} has no OnExit effect.", current_card.base_effects);
            }
        },
        Phases::OnAttack => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnAttack effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnDamaged => match current_card {
            _ => {
                println!(
                    "Card {:?} has no OnDamaged effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnAny => match current_card {
            _ => {}
        },
    }
    // For any behavior.
    match current_card {
        _ => {
            println!("Card {:?} has no OnAny effect.", current_card.base_effects);
        }
    }
}

// For each effect a card could have.
#[derive(Debug, Clone, Copy)]
enum CardName {
    // Creatures
    //
    Zor,          // Zor: This creature cannot attack until it takes damage.
    Oodalah,      // Oodalah: Nothing survives a hit from this card.
    Rock,         // Rock: Cards that fail an attack roll are sent back to their player's hand.
    RogueJellies, // Rogue Jellies: Damage = Health
    Torble, // Torble: Jellies hit by this card are discarded and replaced with a newly drawn Jelly. (This applies to all creatures.)
    Jammie, // Jammie: When this card lands an attack, take a card from that player's hand.
    Slime,  // Slime: Gain # stat every time this card lands an attack.
    Jim,    // Jim: Big Potential. Attach up to # mutation cards to this card.
    Kibble, // Kibble: +# from attack rolls made by this creature. (Kibble is silly)
    Taki,   // Taki: Add # point to this card's stats when put into play.

    // Items
    //
    OnPlayDiscard,  // Catching item behavior.
    GooberFruit,    // Goober Fruit: Restore 2 Health to any Jelly card or Creature card.
    JellyJabber,    // Jelly Jabber: Steal a mutation from a Jelly or Creature.
    JellyJail, // Jelly Jail: Place this card on a Jelly to restrict it from attacking until it takes damage.
    Angelly, // Angelly: When a Jelly would be discarded, restore all of its Health and keep it in play.
    PowderJelly, // Powder Jelly: Deal # damage to each card in play.
    SharpStick, // Sharp Stick: Place on a Jelly or Creature. That card gains double damage the next time it deals damage.
    Shield,     // Shield: When hit with an attack, play this card to take no damage.
    Onedesix,   // Onedesix: Play this card after any roll to re-roll it.
    StickySnatcher, // Sticky Snatcher: Steal 1 item card from another player's hand.
    NabNet, // Nab Net: Play this card when a Jelly or Creature is discarded to add it to your hand.

    // Jellies
    //
    Brusier, // Bruiser: +y Damage when at x Health
    Spicy,   // Spicy: Hazardous: When you take daamge, deal 1 damage back.
    Shelly,  // Shelly: Armored: All attacks against this card can only deal # damage.
    Flutter, // Flutter: Fast: When attacking with this card, boost the attack roll by #.
    Jambler, // Jambler: Jamble: At the start of your turn, roll a dice to determine this card's health.
    Jumper, // Jumper: Warp: Landing an attack on a Jelly or Creature card sends them back to their player's hand.
    Gum, // Gum: Mimic: Replace this card with the next card it reduces to 0 Health. (Discard the original card.)
    Chilli, // Chilli: Freeze: Jellies that hit you with an attack cannot attack on their next turn.
    Junior, // Junior: Potential: Attach up to two mutation cards to this card.
    Sling, // Sling: Sling: This jelly does not take damage caused by Hazardous.

    // Mutations
    //
    PitifulGaze, // Pitiful Gaze: +1 Defense
    Strong,      // Strong: +1 Damage
    Tough,       // Tough: +1 Health
    Fast,        // Fast: When attacking with this card, boost the attack roll by 1.
    Sucker, // Sucker: Gain +1 Health for every point of damage dealt. Cannot go over health maximum.
    Armor,  // Armor: Armored: All attacks against this card can only deal 1 damage.
    Willpower, // Willpower: When this card would be discarded, it can remain in play until after your next turn. (Withdrawing before then saves this card.)
    Icebreaker, // Icebreaker: Immune to Freeze and Jelly Jail
    Hazardous, // Hazardous: When you take damage, deal one damage back to the attacker.
    Super,     // Super: When a Jelly on your team is discarded, gain +1 Attack and +1 Defense
}
