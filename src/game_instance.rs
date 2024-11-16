use macroquad::rand::ChooseRandom;

#[derive(Debug)]
pub struct Player {}

#[derive(Debug)]
pub struct GameData {
    // All of the base data we'd want.
    pub deck_jelly: Vec<Card>, // This'll store all the jelly cards at game start.
    pub deck_creature: Vec<Card>, // All the creature cards.
    pub deck_mutation: Vec<Card>, // Mutation cards.
    pub deck_item: Vec<Card>,  // Item cards.

    // Spare deck for game rule purposes.
    pub deck_loot: Vec<Card>, // This'll be where the Creature/Mutation/Item cards go after initalization.
    pub deck_prize_pool: Vec<Card>, // The winning cards will be placed here.

    // Handling player data.
    pub player_hands: Vec<Vec<Card>>, // This'll store all of the hands of each player.
    pub player_fields: Vec<Vec<Card>>, // This'll store the battlefields of each player.
    pub player_victories: Vec<u8>,    // And this'll be for tracking wins.

    // And for individual turn states.
    pub current_player: u8,
}

impl GameData {
    // For creating a new gamedata instance.
    pub fn new(player_count: u8) -> Option<GameData> {
        if player_count > 1 {
            Some(GameData {
                deck_jelly: vec![Card::new_living(CardName::Brusier, 2, 2, 3)],
                deck_creature: vec![Card::new_living(CardName::Zor, 3, 2, 3)],
                deck_mutation: vec![Card::new_usable(CardName::PitifulGaze)],
                deck_item: vec![Card::new_usable(CardName::GooberFruit)],
                deck_loot: Vec::new(),
                deck_prize_pool: Vec::new(),
                player_hands: vec![Vec::new(); player_count.into()],
                player_fields: vec![Vec::new(); player_count.into()],
                player_victories: vec![0; player_count.into()],
                current_player: 0,
            })
        } else {
            None
        }
    }

    fn draw_card(source: &mut Vec<Card>, destination: &mut Vec<Card>) {
        if (source.len() > 0) {
            destination.push(source.pop().unwrap());
        } else {
            println!("No cards left in deck!");
        }
    }

    pub async fn run(&mut self) {
        // YAY GAME START!
        // Shuffling our decks.
        self.deck_jelly.shuffle();
        self.deck_creature.shuffle();
        self.deck_mutation.shuffle();
        self.deck_item.shuffle();

        for (index, player) in self.player_hands.iter_mut().enumerate() {
            println!("Drawing starting hand for Player {}", index);
            GameData::draw_card(&mut self.deck_jelly, player);
            GameData::draw_card(&mut self.deck_jelly, player);
            GameData::draw_card(&mut self.deck_creature, player);
            GameData::draw_card(&mut self.deck_mutation, player);
            GameData::draw_card(&mut self.deck_item, player);
            println!("Current hand is {:?}.", player);
        }

        // 'round: loop {
        //     'turn: loop {}
        // }

        let mut command_input = String::new();
        std::io::stdin()
            .read_line(&mut command_input)
            .expect("Resuming.");
    }
}

// For storing each card.
#[derive(Debug, Clone)]
struct Card {
    // Stats
    health: Option<u8>,              // The health (optional.)
    damage: Option<u8>,              // The damage (optional.)
    defense: Option<u8>,             // The defense (optional.)
    base_effects: CardName,          // The card's base effects.
    modifier_slots: u8,              // The amount of modifier slots.
    modifier_effects: Vec<CardName>, // The card's modified effects.
}

impl Card {
    fn new_usable(card_name: CardName) -> Card {
        Card {
            health: None,
            damage: None,
            defense: None,
            base_effects: card_name,
            modifier_slots: 0,
            modifier_effects: Vec::new(),
        }
    }

    fn new_living(card_name: CardName, health: u8, damage: u8, defense: u8) -> Card {
        Card {
            health: Some(health),
            damage: Some(damage),
            defense: Some(defense),
            base_effects: card_name,
            modifier_slots: 1,
            modifier_effects: Vec::new(),
        }
    }
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
        Phases::OnDraw => match current_card.base_effects {
            _ => {
                println!("Card {:?} has no OnDraw effect.", current_card.base_effects);
            }
        },
        Phases::OnDiscard => match current_card.base_effects {
            _ => {
                println!(
                    "Card {:?} has no OnDiscard effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnBounce => match current_card.base_effects {
            _ => {
                println!(
                    "Card {:?} has no OnBounce effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnTurnStart => match current_card.base_effects {
            _ => {
                println!(
                    "Card {:?} has no OnTurnStart effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnTurnEnd => match current_card.base_effects {
            _ => {
                println!(
                    "Card {:?} has no OnTurnEnd effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnEnter => match current_card.base_effects {
            _ => {
                println!(
                    "Card {:?} has no OnEnter effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnExit => match current_card.base_effects {
            _ => {
                println!("Card {:?} has no OnExit effect.", current_card.base_effects);
            }
        },
        Phases::OnAttack => match current_card.base_effects {
            CardName::Oodalah => {
                // Nothing survives a hit from this card.
            }
            _ => {
                println!(
                    "Card {:?} has no OnAttack effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnDamaged => match current_card.base_effects {
            CardName::Zor => {
                // This creature cannot attack until it takes damage.
            }
            CardName::Rock => {
                // Cards that fail an attack roll are sent back to their player's hand.
            }
            _ => {
                println!(
                    "Card {:?} has no OnDamaged effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnAny => match current_card.base_effects {
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
