use std::ops::SubAssign;

use macroquad::prelude::*;
use macroquad::{
    rand::ChooseRandom,
    texture::{draw_texture_ex, load_texture},
    window::{clear_background, next_frame},
};
use rodio::*;

macro_rules! play_audio {
    ($sink:ident, $file:expr $(,)?, $volume:expr $(,)?, $speed:expr $(,)?) => {
        $sink.skip_one();
        $sink.append(
            Decoder::new_wav(Cursor::new(&include_bytes!($file)))
                .unwrap()
                .amplify($volume)
                .speed($speed),
        );
    };
}

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
    pub player_placement: Vec<usize>, // This is tracking the placements for each round.
    pub player_victories: Vec<u8>,    // And this'll be for tracking wins.
    pub player_humans: Vec<u8>,       // Tracking which players are humans.

    pub texture_dictionary: Vec<Texture2D>, // For storing all of the textures.

    // And for individual turn states.
    pub player_count: u8,
    pub current_player: u8,
    pub current_round: u16,
    pub victory_threshold: u8,

    // For music.
    pub stream_output: OutputStream,
    pub stream_handler: OutputStreamHandle, // Yeah this is important to keep.
    pub sink_bass: Option<Sink>,            // Our bass. (Always playing)
    pub sink_drums: Option<Sink>,           // Drums. (Plays when it's not your turn.)
    pub sink_synth: Option<Sink>, // Synth. (Plays when it's not your turn *and* you're alive.)
}

impl GameData {
    // For creating a new gamedata instance.
    pub async fn new(player_count: u8, victory_threshold: u8) -> Option<GameData> {
        if player_count > 1 {
            log::info!("Initalizing cards...");
            let deck_jelly = vec![
                Card::new_jelly(CardName::Bruiser, 2, 2, 3),
                Card::new_jelly(CardName::Spicy, 2, 2, 3),
                Card::new_jelly(CardName::Shelly, 2, 1, 2),
                Card::new_jelly(CardName::Flutter, 1, 1, 4),
                Card::new_jelly(CardName::Jambler, 6, 1, 2),
                Card::new_jelly(CardName::Jumper, 1, 0, 4),
                Card::new_jelly(CardName::Gum, 3, 1, 2),
                Card::new_jelly(CardName::Chilli, 2, 1, 3),
                Card::new_jelly(CardName::Junior, 2, 1, 3),
                Card::new_jelly(CardName::Sling, 2, 1, 4),
                Card::new_jelly(CardName::Strange, 1, 1, 2),
            ];
            let deck_creature = vec![
                Card::new_creature(CardName::Zor, 3, 2, 3),
                Card::new_creature(CardName::Oodalah, 1, i8::MAX, 3),
                Card::new_creature(CardName::Rock, 4, 0, 2),
                Card::new_creature(CardName::RogueJellies, 3, 3, 2),
                Card::new_creature(CardName::Torble, 1, 0, 4),
                Card::new_creature(CardName::Jammie, 1, 1, 4),
                Card::new_creature(CardName::Slime, 3, 1, 1),
                Card::new_creature(CardName::Jim, 3, 1, 3),
                Card::new_creature(CardName::Kibble, 2, 2, 4),
                Card::new_creature(CardName::Taki, 3, 1, 3),
                Card::new_creature(CardName::Paolarm, 3, 3, 3),
                Card::new_creature(CardName::StrangeBlock, 1, 1, 2),
            ];
            let deck_mutation = vec![
                Card::new_mutation(CardName::PitifulGaze),
                Card::new_mutation(CardName::Strong),
                Card::new_mutation(CardName::Tough),
                Card::new_mutation(CardName::Fast),
                Card::new_mutation(CardName::Sucker),
                Card::new_mutation(CardName::Armor),
                Card::new_mutation(CardName::Willpower),
                Card::new_mutation(CardName::Icebreaker),
                Card::new_mutation(CardName::Hazardous),
                Card::new_mutation(CardName::Super),
            ];
            let deck_item = vec![
                Card::new_item(CardName::GooberFruit),
                Card::new_item(CardName::JellyJabber),
                Card::new_item(CardName::JellyJail),
                Card::new_item(CardName::Angelly),
                Card::new_item(CardName::PowderJelly),
                Card::new_item(CardName::SharpStick),
                Card::new_item(CardName::Shield),
                Card::new_item(CardName::Onedeesix),
                Card::new_item(CardName::StickySnatcher),
                Card::new_item(CardName::NabNet),
            ];
            // Loading card textures.
            log::info!("Loading textures...");
            let mut card_textures = vec![
                Texture2D::empty();
                &deck_jelly.len()
                    + &deck_creature.len()
                    + &deck_mutation.len()
                    + &deck_item.len()
            ];
            for card in &deck_jelly {
                card_textures[card.base_effects as usize] = card.load_texture().await;
            }
            log::info!("Loaded {} jellies...", &deck_jelly.len());
            for card in &deck_creature {
                card_textures[card.base_effects as usize] = card.load_texture().await;
            }
            log::info!("Loaded {} creatures...", &deck_creature.len());
            for card in &deck_mutation {
                card_textures[card.base_effects as usize] = card.load_texture().await;
            }
            log::info!("Loaded {} mutations...", &deck_mutation.len());
            for card in &deck_item {
                card_textures[card.base_effects as usize] = card.load_texture().await;
            }
            log::info!("Loaded {} items...", &deck_item.len());
            log::info!("Done loading!");
            let (_stream, stream_handle) = OutputStream::try_default().unwrap(); // Creating our sinks.
            let mut output = GameData {
                deck_jelly: deck_jelly,
                deck_creature: deck_creature,
                deck_mutation: deck_mutation,
                deck_item: deck_item,
                deck_loot: Vec::new(),
                deck_prize_pool: Vec::new(),
                player_hands: vec![Vec::new(); player_count.into()],
                player_fields: vec![Vec::new(); player_count.into()],
                player_placement: Vec::new(),
                player_victories: vec![0; player_count.into()],
                player_humans: vec![0],
                texture_dictionary: card_textures,
                player_count: player_count,
                current_player: player_count - 1,
                current_round: 0,
                victory_threshold: victory_threshold,
                stream_output: _stream,
                stream_handler: stream_handle,
                sink_bass: None,
                sink_drums: None,
                sink_synth: None,
            };
            output.sink_bass = Some(Sink::try_new(&output.stream_handler).unwrap());
            output.sink_drums = Some(Sink::try_new(&output.stream_handler).unwrap());
            output.sink_synth = Some(Sink::try_new(&output.stream_handler).unwrap());
            output.sink_drums.as_mut().unwrap().set_volume(0.0);
            output.sink_synth.as_mut().unwrap().set_volume(0.0);
            Some(output)
        } else {
            None
        }
    }

    // HELPER FUNCTIONS
    //
    fn move_card(source: Card, destination: &mut Vec<Card>) {
        destination.push(source); // Yeah just move it over.
    }

    fn draw_card(source: &mut Vec<Card>, destination: &mut Vec<Card>) {
        if source.len() > 0 {
            let fresh_card = &mut source.pop().unwrap();
            fresh_card.current_health = fresh_card.base_health;
            fresh_card.current_damage = fresh_card.base_damage;
            fresh_card.current_defense = fresh_card.base_defense;
            destination.push(fresh_card.clone()); // Move it out of one and into the other.
        } else {
            log::info!("No cards left in source!"); // Scream and cry.
        }
    }

    // TODO!!!
    // These two selection functions are where you can put a draw call/render call in.
    fn select_card(source: &mut Vec<Card>, filter: impl Fn(&&Card) -> bool) -> Option<Card> {
        //
        // Up here call some sort of `fn Draw` function, then have it hover here until a player makes a selection, then let it keep running.
        // All you should really have to do is draw the current card selection and then click it.
        // Sorry, very sleepy right now.
        //
        let filtered_selection = source.iter().filter(filter).cloned().collect::<Vec<Card>>();
        if filtered_selection.len() > 0 {
            let selection =
                &filtered_selection[(macroquad::rand::rand() as usize) % filtered_selection.len()]; // Just random number generation for now. Replace the macroquad:rand:rand() with the eventual selection the player makes.
            log::info!("Selected {:?}.", selection);
            Some(source.swap_remove(source.iter().position(|card| card == selection).unwrap()))
        } else {
            log::info!("Guess not...");
            None
        }
    }

    fn select_hand(source: &Vec<Vec<Card>>, filter: impl Fn(usize) -> bool) -> usize {
        //
        // Same thing in here, put the same `fn Draw` function, except instead of grabbing a card from the source set, it's selecting a player's hand.
        // Doesn't need to be too fancy, as long as it works.
        //
        loop {
            let selection = (macroquad::rand::rand() as usize) % source.len(); // Replace the macroquad:rand:rand() with the eventual selection the player makes.
            log::info!("Selected {:?}.", selection);
            if filter(selection) {
                return selection;
            }
            log::info!("Please pick a different one.");
        }
    }

    // FOR THE GAME LOOP ITSELF
    pub async fn run(&mut self) {
        // YAY GAME START!
        // Shuffling our decks.
        self.deck_jelly.shuffle();
        self.deck_creature.shuffle();
        self.deck_mutation.shuffle();
        self.deck_item.shuffle();

        // Giving each player their starting hand.
        for (index, player) in self.player_hands.iter_mut().enumerate() {
            log::info!("Drawing starting hand for Player {}", index);
            GameData::draw_card(&mut self.deck_jelly, player);
            GameData::draw_card(&mut self.deck_jelly, player);
            GameData::draw_card(&mut self.deck_creature, player);
            GameData::draw_card(&mut self.deck_mutation, player);
            GameData::draw_card(&mut self.deck_item, player);
        }

        // Moving all of the special cards over to the loot deck.
        log::info!("Moving everything to the loot pile...");
        self.deck_loot.append(&mut self.deck_creature);
        self.deck_loot.append(&mut self.deck_mutation);
        self.deck_loot.append(&mut self.deck_item);
        self.deck_loot.shuffle();

        // Moving our loot deck to the prize pool.
        log::info!("Here are the prize cards.");
        for _iterator in 0..(self.player_count - 1) {
            GameData::draw_card(&mut self.deck_loot, &mut self.deck_prize_pool);
            log::info!(
                "A {:?} is in the prize pool!",
                &self.deck_prize_pool.last().unwrap()
            );
        }

        // For each round.
        log::info!("Now let's get the game started.");
        'round: loop {
            // Pre-round.
            log::info!("Starting Round {:?}.", self.current_round + 1);
            // Everybody plays two living cards.
            for _iterator in 0..self.player_count {
                log::info!("Player {}, play two cards.", self.current_player);
                for _iterator in 0..2 {
                    self.draw(0.2).await;
                    let mut selected_card = GameData::select_card(
                        &mut self.player_hands[self.current_player as usize],
                        |card| card.color == CardColor::Jelly || card.color == CardColor::Creature,
                    );
                    if selected_card.is_some() {
                        self.draw(0.2).await;
                        log::info!("Any mutations?");
                        let mutation_card = GameData::select_card(
                            &mut self.player_hands[self.current_player as usize],
                            |card| card.color == CardColor::Mutation,
                        );
                        if mutation_card.is_some() {
                            self.draw(0.2).await;
                            GameData::move_card(
                                mutation_card.unwrap(),
                                &mut selected_card.as_mut().unwrap().modifier_effects,
                            );
                        }
                        GameData::move_card(
                            selected_card.unwrap(),
                            &mut self.player_fields[self.current_player as usize],
                        );
                    } else {
                        log::info!("...No cards to play?");
                    }
                    self.draw(0.2).await;
                }
                self.current_player = (self.current_player + 1) % (self.player_hands.len() as u8);
            }

            // Now for the primary game loop.
            'turn: loop {
                // Start turn.
                log::info!("Starting Player {}'s turn!", self.current_player);
                // Reading out the current board state.
                log::info!("Here's the current battlefield. {:?}", self.player_fields);
                log::info!(
                    "And your hand. {:?}",
                    self.player_hands[self.current_player as usize]
                );
                self.draw(0.3).await; // Good to render before anything happens.

                // Middle turn.
                if self.player_fields[self.current_player as usize].len() > 0 {
                    let mut has_performed_action = false;
                    // Handling items. (NOT MUTATIONS!)
                    if !has_performed_action {
                        log::info!("Select items.");
                        let item = GameData::select_card(
                            &mut self.player_fields[self.current_player as usize],
                            |card| card.color == CardColor::Item,
                        );
                        if item.is_some() {
                            has_performed_action = true;
                            // TODO: Perform item action.
                            self.deck_loot.push(item.unwrap());
                            self.deck_loot.shuffle();
                        }
                    }
                    // Handling combat.
                    if !has_performed_action
                        && self.player_fields[self.current_player as usize].len() > 0
                    {
                        self.draw(0.2).await;
                        log::info!("Select attackers.");
                        let attacker = GameData::select_card(
                            &mut self.player_fields[self.current_player as usize],
                            |card| card.current_damage.is_some_and(|x| x > 0),
                        );
                        if attacker.as_ref().is_some() {
                            self.draw(0.2).await;
                            log::info!("Select who you're attacking.");
                            let target_field = GameData::select_hand(&self.player_fields, |hand| {
                                hand != self.current_player as usize
                                    && self.player_fields[hand].len() > 0
                            });
                            log::info!("Select which card you're attacking.");
                            let defender = &mut GameData::select_card(
                                &mut self.player_fields[target_field],
                                |card| card.current_health.is_some(),
                            );
                            self.draw(0.2).await;
                            let roll = (macroquad::rand::rand() % 6 + 1) as i8;
                            if roll >= defender.as_ref().unwrap().current_defense.unwrap() {
                                log::info!("{}! Successful roll!", roll);
                                defender
                                    .as_mut()
                                    .unwrap()
                                    .current_health
                                    .as_mut()
                                    .unwrap()
                                    .sub_assign(attacker.as_ref().unwrap().current_damage.unwrap());
                                if defender
                                    .as_ref()
                                    .unwrap()
                                    .current_health
                                    .is_some_and(|x| x <= 0)
                                {
                                    // Discard.
                                    log::info!("{:?} defeated...", defender.as_ref().unwrap());

                                    // Special cases for jellies.
                                    while defender.as_mut().unwrap().modifier_effects.len() > 0 {
                                        self.deck_loot.push(
                                            defender
                                                .as_mut()
                                                .unwrap()
                                                .modifier_effects
                                                .pop()
                                                .unwrap(),
                                        );
                                        self.deck_loot.shuffle();
                                    }
                                    if defender.as_ref().unwrap().color == CardColor::Jelly {
                                        self.deck_jelly.push(defender.clone().unwrap());
                                        self.deck_jelly.shuffle();
                                    } else {
                                        self.deck_loot.push(defender.clone().unwrap());
                                        self.deck_loot.shuffle();
                                    }

                                    // Handling placement if that was their last card.
                                    if self.player_fields[target_field].len() <= 0 {
                                        log::info!("Player {} has been knocked out.", target_field);
                                        self.player_placement.push(target_field);
                                    }
                                } else {
                                    // Return to field.
                                    self.player_fields[target_field]
                                        .push(defender.clone().unwrap());
                                }
                            } else {
                                log::info!("{}. Failed roll.", roll);
                                self.player_fields[target_field].push(defender.clone().unwrap());
                            }
                            // Putting the card back.
                            has_performed_action = true;
                            self.player_fields[self.current_player as usize]
                                .push(attacker.unwrap());
                        } else {
                            log::info!("No valid attackers.");
                        }
                        self.draw(0.2).await;
                    }
                    // Handling withdrawing.
                    if !has_performed_action {
                        self.draw(0.2).await;
                        log::info!("Select deserters.");
                        let mut deserter = GameData::select_card(
                            &mut self.player_fields[self.current_player as usize],
                            |_x| true,
                        );
                        if deserter.is_some() {
                            self.draw(0.2).await;
                            // has_performed_action = true;
                            while deserter.as_mut().unwrap().modifier_effects.len() > 0 {
                                self.deck_loot.push(
                                    deserter.as_mut().unwrap().modifier_effects.pop().unwrap(),
                                );
                                self.deck_loot.shuffle();
                            }
                            self.player_hands[self.current_player as usize].push(deserter.unwrap());
                            // And the reprecussions of doing that.
                            if self.player_fields[self.current_player as usize].len() <= 0 {
                                log::info!(
                                    "Player {} has been knocked out.",
                                    self.current_player as usize
                                );
                                self.player_placement.push(self.current_player as usize);
                            }
                        }
                    }
                } else {
                    log::info!(
                        "Player {} has no cards on their field, so we're skipping their turn.",
                        self.current_player
                    )
                }

                // End turn.
                // Discard down to eight.
                while self.player_hands[self.current_player as usize].len() > 8 {
                    self.draw(0.2).await;
                    log::info!("Please discard down to eight cards.");
                    let mut selected_card = GameData::select_card(
                        &mut self.player_hands[self.current_player as usize],
                        |_iterator| true,
                    );
                    while selected_card.as_mut().unwrap().modifier_effects.len() > 0 {
                        self.deck_loot.push(
                            selected_card
                                .as_mut()
                                .unwrap()
                                .modifier_effects
                                .pop()
                                .unwrap(),
                        );
                        self.deck_loot.shuffle();
                    }
                    if selected_card.as_ref().unwrap().color == CardColor::Jelly {
                        self.deck_jelly.push(selected_card.unwrap());
                        self.deck_jelly.shuffle();
                    } else {
                        self.deck_loot.push(selected_card.unwrap());
                        self.deck_loot.shuffle();
                    }
                    self.draw(0.2).await;
                }
                // And quietly performing the board check.
                if self
                    .player_fields
                    .iter()
                    .filter(|field| field.len() > 0)
                    .count()
                    <= 1
                {
                    // Marking placements.
                    self.player_placement.push(self.current_player as usize);

                    // Moving the last card over.
                    while self.player_fields[self.current_player as usize].len() > 0 {
                        let mut last_card = self.player_fields[self.current_player as usize].pop();
                        if last_card.is_some() {
                            self.draw(0.2).await;
                            log::info!("Moving {:?} back to hand.", last_card.as_ref().unwrap());
                            while last_card.as_mut().unwrap().modifier_effects.len() > 0 {
                                GameData::move_card(
                                    last_card.as_mut().unwrap().modifier_effects.pop().unwrap(),
                                    &mut self.player_hands[self.current_player as usize],
                                );
                            }
                            GameData::move_card(
                                last_card.unwrap(),
                                &mut self.player_hands[self.current_player as usize],
                            );
                        }
                    }
                    break 'turn;
                }
                // Moving along.
                self.current_player = (self.current_player + 1) % (self.player_hands.len() as u8);
            }
            // Post-round
            self.player_placement.reverse(); // Reversing the placement.
            log::info!("Round end. Player {} won!", self.player_placement[0]);
            self.player_victories[self.player_placement[0]] += 1; // Incrementing score.

            // Stating placements.
            log::info!("The placements are {:?}", self.player_placement);

            // If anybody has passed the threshold.
            log::info!("The current scores are {:?}", self.player_victories);
            if self
                .player_victories
                .iter()
                .any(|victory| victory >= &self.victory_threshold)
            {
                break 'round;
            }

            // Handling prizes.
            for iterator in 0..(self.player_placement.len() - 1) {
                self.draw(0.2).await;
                log::info!(
                    "Player {}, please pick a prize card.",
                    self.player_placement[iterator]
                );
                GameData::move_card(
                    GameData::select_card(&mut self.deck_prize_pool, |_x| true).unwrap(),
                    &mut self.player_hands[self.player_placement[iterator]],
                );
                self.draw(0.2).await;
            }
            self.player_placement.clear(); // Clearing the placement.

            // Moving our loot deck to the prize pool.
            log::info!("Here are the prize cards.");
            for _iterator in 0..(self.player_count - 1) {
                self.draw(0.1).await;
                GameData::draw_card(&mut self.deck_loot, &mut self.deck_prize_pool);
                log::info!(
                    "A {:?} is in the prize pool!",
                    &self.deck_prize_pool.last().unwrap()
                );
            }

            // Drawing new Jellies.
            for (index, player) in self.player_hands.iter_mut().enumerate() {
                log::info!("Drawing new jellies for Player {}", index);
                if !player
                    .iter()
                    .any(|card| card.color == CardColor::Jelly || card.color == CardColor::Creature)
                {
                    log::info!("And an extra...");
                    GameData::draw_card(&mut self.deck_jelly, player);
                }
                GameData::draw_card(&mut self.deck_jelly, player);
                log::info!("Current hand is {:?}.", player);
            }

            // Incrementing our Round counter.
            self.current_round += 1;
        }
        // SOMEBODY WON!
        log::info!("And we have a winner!");
        self.draw(10.0).await;
    }

    // For drawing everything to the screen.
    pub async fn draw(&mut self, debug_wait_seconds: f32) {
        clear_background(macroquad::color::Color::from_rgba(0, 128, 128, 255)); // Emptying the current buffer.

        // Music's here, too.
        {
            // Handling volume.
            let current_drums_volume = self.sink_drums.as_ref().unwrap().volume();
            let current_synth_volume = self.sink_synth.as_ref().unwrap().volume();
            self.sink_drums.as_mut().unwrap().set_volume(
                current_drums_volume * 0.9
                    + !self.player_humans.contains(&self.current_player) as u32 as f32 * 0.1,
            );
            self.sink_synth.as_mut().unwrap().set_volume(
                current_synth_volume * 0.9
                    + (!self.player_humans.contains(&self.current_player)
                        && self.player_fields[self.player_humans[0] as usize].len() > 0)
                        as u32 as f32
                        * 0.1,
            );

            // Gotta replesh the thingies.
            if self.sink_synth.as_mut().unwrap().len() < 2 {
                self.sink_bass.as_mut().unwrap().append(
                    Decoder::new_wav(std::io::Cursor::new(&include_bytes!(
                        "../assets/sounds/music/insane_bass.wav"
                    )))
                    .unwrap(),
                );
                self.sink_drums.as_mut().unwrap().append(
                    Decoder::new_wav(std::io::Cursor::new(&include_bytes!(
                        "../assets/sounds/music/insane_drums.wav"
                    )))
                    .unwrap(),
                );
                self.sink_synth.as_mut().unwrap().append(
                    Decoder::new_wav(std::io::Cursor::new(&include_bytes!(
                        "../assets/sounds/music/insane_synth.wav"
                    )))
                    .unwrap(),
                );
            }
        }

        // Time to draw everything.
        //

        // Helper text.
        draw_text_ex(
            &format!(
                "Round {} - Player {}'s Turn - Current Wins {:?} (Best of {})",
                self.current_round + 1,
                self.current_player,
                self.player_victories,
                self.victory_threshold
            ),
            10.0,
            screen_height() - 6.0,
            TextParams {
                font_size: 24,
                font_scale: 1.0,
                color: macroquad::color::WHITE,
                ..Default::default()
            },
        );

        // Prize pool first, since it's unimportant.
        for (card_index, card) in self.deck_prize_pool.iter().enumerate() {
            Card::draw(
                self.texture_dictionary[card.base_effects as usize],
                screen_width() - 100.0,
                screen_height() - (card_index as f32 * 136.0) - 136.0,
                90.0,
            )
            .await;
        }

        // Drawing each field.
        for field in 0..(self.player_fields.len()) {
            for (card_index, card) in self.player_fields[field].iter().enumerate() {
                for (mutator_index, mutator_card) in card.modifier_effects.iter().enumerate() {
                    Card::draw(
                        self.texture_dictionary[mutator_card.base_effects as usize],
                        (card_index as f32 * 110.0)
                            + (field as f32 * 300.0)
                            + ((mutator_index + 1) as f32 * 2.0)
                            + 10.0,
                        ((mutator_index + 1) as f32 * 60.0) + 10.0,
                        130.0,
                    )
                    .await;
                }
                Card::draw(
                    self.texture_dictionary[card.base_effects as usize],
                    (card_index as f32 * 110.0) + (field as f32 * 300.0) + 10.0,
                    10.0,
                    130.0,
                )
                .await;
            }
        }

        // And the hand.
        for (card_index, card) in self.player_hands[self.current_player as usize]
            .iter()
            .enumerate()
        {
            Card::draw(
                self.texture_dictionary[card.base_effects as usize],
                card_index as f32 * 210.0 + 10.0,
                screen_height() - 310.0,
                200.0,
            )
            .await;
        }

        ::std::thread::sleep(std::time::Duration::new(
            0,
            (debug_wait_seconds * 1000000000.0) as u32,
        ));

        next_frame().await; // Drawing that next frame.
    }
}

// For storing each card.
#[derive(Clone, PartialEq, Eq)]
pub struct Card {
    // Stats
    color: CardColor,
    base_health: Option<i8>,     // The health (optional.)
    base_damage: Option<i8>,     // The damage (optional.)
    base_defense: Option<i8>,    // The defense (optional.)
    current_health: Option<i8>,  // The health (optional.)
    current_damage: Option<i8>,  // The damage (optional.)
    current_defense: Option<i8>, // The defense (optional.)
    base_effects: CardName,      // The card's base effects.
    modifier_slots: u8,          // The amount of modifier slots.
    modifier_effects: Vec<Card>, // The card's modified effects.
}

impl Card {
    pub async fn load_texture(&self) -> Texture2D {
        load_texture(&format!(
            "./assets/cards/{:?}/{:?}.png",
            self.color, self.base_effects
        ))
        .await
        .unwrap()
    }

    pub async fn draw(texture: Texture2D, x: f32, y: f32, size: f32) {
        draw_texture_ex(
            texture,
            x,
            y,
            macroquad::color::WHITE,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(Vec2::new(size, size * 1.4)),
                ..Default::default()
            },
        );
    }

    fn new_jelly(card_name: CardName, health: i8, damage: i8, defense: i8) -> Card {
        Card::new_living(card_name, health, damage, defense, CardColor::Jelly)
    }

    fn new_creature(card_name: CardName, health: i8, damage: i8, defense: i8) -> Card {
        Card::new_living(card_name, health, damage, defense, CardColor::Creature)
    }

    fn new_mutation(card_name: CardName) -> Card {
        Card::new_usable(card_name, CardColor::Mutation)
    }

    fn new_item(card_name: CardName) -> Card {
        Card::new_usable(card_name, CardColor::Item)
    }

    fn new_usable(card_name: CardName, color: CardColor) -> Card {
        Card {
            color: color,
            base_health: None,
            base_damage: None,
            base_defense: None,
            current_health: None,
            current_damage: None,
            current_defense: None,
            base_effects: card_name,
            modifier_slots: 0,
            modifier_effects: Vec::new(),
        }
    }

    fn new_living(
        card_name: CardName,
        health: i8,
        damage: i8,
        defense: i8,
        color: CardColor,
    ) -> Card {
        Card {
            color: color,
            base_health: Some(health),
            base_damage: Some(damage),
            base_defense: Some(defense),
            current_health: Some(health),
            current_damage: Some(damage),
            current_defense: Some(defense),
            base_effects: card_name,
            modifier_slots: 1,
            modifier_effects: Vec::new(),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // "{}[{:?}{}{}]\x1b[0m",
            // match self.color {
            //     CardColor::Jelly => "\x1b[0;36m",
            //     CardColor::Creature => "\x1b[0;31m",
            //     CardColor::Mutation => "\x1b[0;34m",
            //     CardColor::Item => "\x1b[0;33m",
            // },
            "[{:?}{}{}]",
            self.base_effects,
            if self.current_health.is_some() {
                format!(
                    ", {:?}/{:?}/{:?}",
                    self.current_health.unwrap_or(0),
                    self.current_damage.unwrap_or(0),
                    self.current_defense.unwrap_or(0)
                )
            } else {
                "".to_string()
            },
            if self.modifier_slots > 0 {
                format!("; {:?}:{:?}", self.modifier_slots, self.modifier_effects)
            } else {
                "".to_string()
            }
        )
    }
}

// For keeping track of decked data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardColor {
    Jelly,
    Creature,
    Mutation,
    Item,
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
                log::info!("Card {:?} has no OnDraw effect.", current_card.base_effects);
            }
        },
        Phases::OnDiscard => match current_card.base_effects {
            _ => {
                log::info!(
                    "Card {:?} has no OnDiscard effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnBounce => match current_card.base_effects {
            _ => {
                log::info!(
                    "Card {:?} has no OnBounce effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnTurnStart => match current_card.base_effects {
            _ => {
                log::info!(
                    "Card {:?} has no OnTurnStart effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnTurnEnd => match current_card.base_effects {
            _ => {
                log::info!(
                    "Card {:?} has no OnTurnEnd effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnEnter => match current_card.base_effects {
            _ => {
                log::info!(
                    "Card {:?} has no OnEnter effect.",
                    current_card.base_effects
                );
            }
        },
        Phases::OnExit => match current_card.base_effects {
            _ => {
                log::info!("Card {:?} has no OnExit effect.", current_card.base_effects);
            }
        },
        Phases::OnAttack => match current_card.base_effects {
            CardName::Oodalah => {
                // Nothing survives a hit from this card.
            }
            _ => {
                log::info!(
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
                log::info!(
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
            log::info!("Card {:?} has no OnAny effect.", current_card.base_effects);
        }
    }
}

// For each effect a card could have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Paolarm, // Paolarm: Can only target Jellies/Creatures that cannot currently move. (Sleepy?)
    StrangeBlock, // Strange Block: +1 on rolls against all Creature cards.

    // Items
    //
    GooberFruit,    // Goober Fruit: Restore 2 Health to any Jelly card or Creature card.
    JellyJabber,    // Jelly Jabber: Steal a mutation from a Jelly or Creature.
    JellyJail, // Jelly Jail: Place this card on a Jelly to restrict it from attacking until it takes damage.
    Angelly, // Angelly: When a Jelly would be discarded, restore all of its Health and keep it in play.
    PowderJelly, // Powder Jelly: Deal # damage to each card in play.
    SharpStick, // Sharp Stick: Place on a Jelly or Creature. That card gains double damage the next time it deals damage.
    Shield,     // Shield: When hit with an attack, play this card to take no damage.
    Onedeesix,  // Onedeesix: Play this card after any roll to re-roll it.
    StickySnatcher, // Sticky Snatcher: Steal 1 item card from another player's hand.
    NabNet, // Nab Net: Play this card when a Jelly or Creature is discarded to add it to your hand.

    // Jellies
    //
    Bruiser, // Bruiser: +y Damage when at x Health
    Spicy,   // Spicy: Hazardous: When you take daamge, deal 1 damage back.
    Shelly,  // Shelly: Armored: All attacks against this card can only deal # damage.
    Flutter, // Flutter: Fast: When attacking with this card, boost the attack roll by #.
    Jambler, // Jambler: Jamble: At the start of your turn, roll a dice to determine this card's health.
    Jumper, // Jumper: Warp: Landing an attack on a Jelly or Creature card sends them back to their player's hand.
    Gum, // Gum: Mimic: Replace this card with the next card it reduces to 0 Health. (Discard the original card.)
    Chilli, // Chilli: Freeze: Jellies that hit you with an attack cannot attack on their next turn.
    Junior, // Junior: Potential: Attach up to two mutation cards to this card.
    Sling, // Sling: Sling: This jelly does not take damage caused by Hazardous.
    Strange, // Puzzle: +1 on rolls against all Jelly cards.

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
