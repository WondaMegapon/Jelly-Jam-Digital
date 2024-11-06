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

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Rules,
    ViewCards,
}

// For storing each card.
struct Card {
    // Stats
    
    effects_base: Vec<CardEffect>, // Base Effects
    effects_modifier: Vec<Vec<CardEffect>> // Modifier Effects
}

struct CardStats {
    health: u32, // The damage points this card can sustain.
    damage: u32, // 
    defense: u32
}

// So there's a big container for all effects on a card.
// The big container is broken into individual phases with vectors for every effect.
// Each phase has it's own iterator that is called when that phase occurs.
// Card is played. It's effect container OnPlay is triggered. Each item is matched and if it is a positive match, then perform that function.
// Effect order is resolve order.
// Attacks happen before damages. Discard happens after damage resolves.

struct EffectContainer {
    on_draw: Vec<CardEffect>, // For when a card is drawn.
    on_discard: Vec<CardEffect>, // For when a card is discarded.
    on_bounce: Vec<CardEffect>, // For when a card is bounced back to its hand.

    on_turn_start: Vec<CardEffect>, // For when a turn starts.
    on_turn_end: Vec<CardEffect>, // For when a turn ends.

    on_enter: Vec<CardEffect>, // For when a card is played (enters the field).
    on_exit: Vec<CardEffect>, // For when a card leaves (exits the field).

    on_attack: Vec<CardEffect>, // For when a card attacks.
    on_damaged: Vec<CardEffect>, // For when a card is damaged.
    
    on_any: Vec<CardEffect>, // Occurs after each other game phase.
}

// Function for invoking the targetting prompt.
// Function for invoking the selection menu prompt.
// enum for selecting a stat.
enum Selector {
    Stat(SelectorStat),
    Deck(SelectorDeck),
}

enum SelectorStat {
    Random, // The selector is chosen randomly.
    User, // The selector is chosen by the user.
    Health,
    Damage,
    Defense
}

enum SelectorDeck {
    Random, // The selector is chosen randomly.
    User, // The selector is chosen by the user.
    Jelly,
    Creature,
    Mutation,
    Item
}

enum SelectorLocation {
    Random, // The selector is chosen randomly.
    User, // The selector is chosen by the user.
    Deck(SelectorDeck),
    Hand,
    Field,
    This
}

// For each effect a card could have.
enum CardEffect {
    // Creatures
    //
    OnAttackPreventAttackUntilDamaged, // Zor: This creature cannot attack until it takes damage.
    OnAttackDeathtouch, // Oodalah: Nothing survives a hit from this card.
    OnDamagedBounceFailedRolls, // Rock: Cards that fail an attack roll are sent back to their player's hand.
    OnAnyConvertStat(SelectorStat, SelectorStat), // Rogue Jellies: Damage = Health
    OnAttackReplaceWithFreshDraw(SelectorDeck), // Torble: Jellies hit by this card are discarded and replaced with a newly drawn Jelly. (This applies to all creatures.)
    OnAttackStealCardFromDamaged, // Jammie: When this card lands an attack, take a card from that player's hand.
    OnAttackStatlink(SelectorStat, u32), // Slime: Gain # stat every time this card lands an attack.
    OnPlayGainMutationSlot(u32), // Jim: Big Potential. Attach up to # mutation cards to this card.
    OnAttackModifyRoll(u32), // Kibble: +# from attack rolls made by this creature. (Kibble is silly)
    OnPlayModifyThisStat(SelectorStat, u32), // Taki: Add # point to this card's stats when put into play.

    // Items
    //
    OnPlayDiscard, // Catching item behavior.
    OnPlayRestoreStat(SelectorStat, u32), // Goober Fruit: Restore 2 Health to any Jelly card or Creature card.
    OnPlayStealCard(SelectorDeck), // Jelly Jabber: Steal a mutation from a Jelly or Creature.
    OnPlayGrantEffect(Box<CardEffect>), // Jelly Jail: Place this card on a Jelly to restrict it from attacking until it takes damage.
    // Angelly: When a Jelly would be discarded, restore all of its Health and keep it in play.
    OnPlay, // Powder Jelly: Deal # damage to each card in play.
    OnPlayGainStatus, // Sharp Stick: Place on a Jelly or Creature. That card gains double damage the next time it deals damage.
    // Shield: When hit with an attack, play this card to take no damage.
    // Onedesix: Play this card after any roll to re-roll it.
    // Sticky Snatcher: Steal 1 item card from another player's hand.
    // Nab Net: Play this card when a Jelly or Creature is discarded to add it to your hand.

    // Jellies
    //
    OnDamagedIncreaseStatAtLow(SelectorStat, u32, u32), // Bruiser: +y Damage when at x Health
    // Spicy: Hazardous: When you take daamge, deal 1 damage back.
    // Shelly: Armored: All attacks against this card can only deal 1 damage.
    // Flutter: Fast: When attacking with this card, boost the attack roll by 1.
    // Jambler: Jamble: At the start of your turn, roll a dice to determine this card's health.
    // Jumper: Warp: Landing an attack on a Jelly or Creature card sends them back to their player's hand.
    // Gum: Mimic: Replace this card with the next card it reduces to 0 Health. (Discard the original card.)
    // Chilli: Freeze: Jellies that hit you with an attack cannot attack on their next turn.
    // Junior: Potential: Attach up to two mutation cards to this card.
    // Sling: Sling: This jelly does not take damage caused by Hazardous.

    // Mutations
    //
    // Pitiful Gaze: +1 Defense
    // Strong: +1 Damage
    // Tough: +1 Health
    // Fast: When attacking with this card, boost the attack roll by 1.
    // Sucker: Gain +1 Health for every point of damage dealt. Cannot go over health maximum.
    // Armor: Armored: All attacks against this card can only deal 1 damage.
    // Willpower: When this card would be discarded, it can remain in play until after your next turn. (Withdrawing before then saves this card.)
    // Icebreaker: Immune to Freeze and Jelly Jail
    // Hazardous: When you take damage, deal one damage back to the attacker.
    // Super: When a Jelly on your team is discarded, gain +1 Attack and +1 Defense
}