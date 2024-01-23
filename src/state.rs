use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use turbo::{borsh, solana::solana_sdk};

use crate::{EventLocationData, ALL_EVENT_LOCATION_DATA};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Adventure {
    pub creator: Pubkey,
    pub save_slot: u8,
    pub state: AdventureState,
}
impl Adventure {
    pub fn new(_p1_pubkey: Pubkey) -> Self {
        let goblins = HashMap::from([(Player::P1, Goblin::new())]);
        let settings = Settings::new();
        Self {
            creator: Pubkey::default(),
            save_slot: 0,
            state: AdventureState::Preparing(goblins, settings),
        }
    }
    pub fn start_adventure(&mut self) -> Result<(), ()> {
        if let AdventureState::Preparing(goblins, settings) = &self.state {
            let turn = Turn::new(*settings.goblin_order.get(&0).unwrap());
            let mut goblins = goblins.clone();
            let mut settings = settings.clone();
            settings.update_goblin_order(&mut goblins);
            turbo::println!("{:#?}\n{:#?}", goblins, settings);
            self.state = AdventureState::Started(
                goblins,
                settings,
                turn,
                AdventurePhase::Camp(CampPhase::new()),
            );
            return Ok(());
        }
        return Err(());
    }
    pub fn rummage_for_loot(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(_goblins, _settings, _turn, phase) = &mut self.state {
            if let AdventurePhase::Camp(camp_phase) = phase {
                if camp_phase.rummage_result == None {
                    let possible_loot = [
                        Rarity::Common,
                        Rarity::Uncommon,
                        Rarity::Rare,
                        Rarity::Legendary,
                        Rarity::Epic,
                    ];
                    camp_phase.rummage_result = Some(RummageResult::Success {
                        loot: Loot {
                            rarity: possible_loot
                                [turbo::sys::rand() as usize % possible_loot.len()],
                        },
                        did_take: None,
                    });
                    return Ok(());
                }
            }
        }
        return Err(());
    }
    pub fn rummage_take_loot(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(ref mut goblins, _settings, turn, phase) = &mut self.state {
            if let AdventurePhase::Camp(camp_phase) = phase {
                match &mut camp_phase.rummage_result {
                    Some(RummageResult::Success {
                        loot,
                        ref mut did_take,
                    }) => {
                        if did_take.is_none() {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            goblin.greed += 1;
                            goblin.loot.push(loot.clone());
                            *did_take = Some(true);
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }
        }
        return Err(());
    }
    pub fn rummage_leave_loot(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(ref mut goblins, _settings, turn, phase) = &mut self.state {
            if let AdventurePhase::Camp(camp_phase) = phase {
                match &mut camp_phase.rummage_result {
                    Some(RummageResult::Success {
                        loot: _,
                        ref mut did_take,
                    }) => {
                        if did_take.is_none() {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            if goblin.greed > 0 {
                                goblin.greed -= 1;
                            }
                            *did_take = Some(false);
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }
        }
        return Err(());
    }
    pub fn event_start(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(_goblins, _settings, _turn, phase) = &mut self.state {
            if let AdventurePhase::Camp(_camp_phase) = phase {
                let locations = ALL_EVENT_LOCATION_DATA;
                let location_index = turbo::sys::rand() as usize % locations.len();
                let location = &locations[location_index];
                let scenarios = location.scenarios;
                let scenarios_index = turbo::sys::rand() as usize % scenarios.len();
                *phase = AdventurePhase::Event(EventPhase {
                    location: location_index,
                    scenario: scenarios_index,
                    outcome: None,
                });
                return Ok(());
            }
        }
        return Err(());
    }
    pub fn keep_going(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(_goblins, _settings, turn, phase) = &mut self.state {
            if let AdventurePhase::Event(_event_phase) = phase {
                turn.num_events += 1;
                let locations = ALL_EVENT_LOCATION_DATA;
                let location_index = turbo::sys::rand() as usize % locations.len();
                let location = &locations[location_index];
                let scenarios = location.scenarios;
                let scenarios_index = turbo::sys::rand() as usize % scenarios.len();
                *phase = AdventurePhase::Event(EventPhase {
                    location: location_index,
                    scenario: scenarios_index,
                    outcome: None,
                });
                return Ok(());
            }
        }
        return Err(());
    }
    pub fn take_a_break(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(_goblins, settings, turn, phase) = &mut self.state {
            if let AdventurePhase::Event(_event_phase) = phase {
                let mut curr_player_index = settings
                    .goblin_order
                    .iter()
                    .find_map(|(i, player)| {
                        if *player == turn.player {
                            return Some(*i);
                        }
                        return None;
                    })
                    .unwrap_or(0);
                curr_player_index += 1;
                curr_player_index %= settings.goblin_order.len() as u8;
                turn.player = settings.goblin_order[&curr_player_index];
                *phase = AdventurePhase::Camp(CampPhase::new());
                return Ok(());
            }
        }
        return Err(());
    }
    pub fn event_make_choice(&mut self, action_index: usize) -> Result<(), ()> {
        if let AdventureState::Started(goblins, _settings, turn, phase) = &mut self.state {
            if let AdventurePhase::Event(event_phase) = phase {
                let data = EventLocationData::get(event_phase.location);
                let scenario = &data.scenarios[event_phase.scenario];
                let outcomes = scenario.actions[action_index].outcomes;
                let effect = turbo::sys::rand() as usize % outcomes.len();
                let goblin = goblins.get_mut(&turn.player).unwrap();
                goblin.greed += 1;
                event_phase.outcome = Some(EventPhaseOutcome {
                    choice: action_index,
                    effect: effect,
                    accepted: false,
                });
                return Ok(());
            }
        }
        return Err(());
    }
    pub fn event_handle_outcome(&mut self) -> Result<(), ()> {
        if let AdventureState::Started(goblins, settings, turn, phase) = &mut self.state {
            if let AdventurePhase::Event(event_phase) = phase {
                // TODO: apply side-effects such as gaining loot, getting attacked, etc
                if let Some(ref mut outcome) = event_phase.outcome {
                    let data = EventLocationData::get(event_phase.location);
                    let outcomes =
                        data.scenarios[event_phase.scenario].actions[outcome.choice].outcomes;
                    let result = outcomes[outcome.effect % outcomes.len()].effect;
                    match result {
                        EventResult::GetLoot => {
                            let possible_loot = [
                                Rarity::Common,
                                Rarity::Uncommon,
                                Rarity::Rare,
                                Rarity::Legendary,
                                Rarity::Epic,
                            ];
                            let loot = Loot {
                                rarity: possible_loot
                                    [turbo::sys::rand() as usize % possible_loot.len()],
                            };
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            // goblin.greed += 1;
                            goblin.loot.push(loot.clone());
                            //
                        }
                        EventResult::GetItem => {
                            //
                            let possible_loot = [
                                Rarity::Common,
                                Rarity::Uncommon,
                                Rarity::Rare,
                                Rarity::Legendary,
                                Rarity::Epic,
                            ];
                            let loot = Loot {
                                rarity: possible_loot
                                    [turbo::sys::rand() as usize % possible_loot.len()],
                            };
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            // goblin.greed += 1;
                            goblin.loot.push(loot.clone());
                        }
                        EventResult::StealLoot => {
                            //
                            let possible_loot = [
                                Rarity::Common,
                                Rarity::Uncommon,
                                Rarity::Rare,
                                Rarity::Legendary,
                                Rarity::Epic,
                            ];
                            let loot = Loot {
                                rarity: possible_loot
                                    [turbo::sys::rand() as usize % possible_loot.len()],
                            };
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            // goblin.greed += 1;
                            goblin.loot.push(loot.clone());
                        }
                        EventResult::StealItem => {
                            //
                            let possible_loot = [
                                Rarity::Common,
                                Rarity::Uncommon,
                                Rarity::Rare,
                                Rarity::Legendary,
                                Rarity::Epic,
                            ];
                            let loot = Loot {
                                rarity: possible_loot
                                    [turbo::sys::rand() as usize % possible_loot.len()],
                            };
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            // goblin.greed += 1;
                            goblin.loot.push(loot.clone());
                        }
                        EventResult::Heal => {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            let amount = 1;
                            for _ in 0..amount {
                                if goblin.health > 0 {
                                    goblin.health += 1;
                                }
                            }
                        }
                        EventResult::BoostLuck => {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            let amount = 1;
                            for _ in 0..amount {
                                goblin.luck += 1;
                            }
                        }
                        EventResult::ReduceGreed => {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            let amount = 2;
                            for _ in 0..amount {
                                if goblin.greed > 0 {
                                    goblin.greed -= 1;
                                }
                            }
                        }
                        EventResult::LoseLoot => {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            let _ = goblin.loot.pop();
                        }
                        EventResult::LoseItem => {
                            //
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            let _ = goblin.loot.pop();
                        }
                        EventResult::LootGotStolen => {
                            let loot = {
                                let goblin = goblins.get_mut(&turn.player).unwrap();
                                goblin.loot.pop().clone()
                            };
                            if let Some(loot) = loot {
                                let i = turn.player.index() + 1 % settings.goblin_order.len();
                                let i = i as u8;
                                let next_player = settings.goblin_order[&i];
                                let goblin = goblins.get_mut(&next_player).unwrap();
                                goblin.loot.push(loot);
                            }
                        }
                        EventResult::ItemGotStolen => {
                            //
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            let _ = goblin.loot.pop();
                        }
                        EventResult::SlapFight => {
                            //
                        }
                        EventResult::GetAttacked => {
                            let goblin = goblins.get_mut(&turn.player).unwrap();
                            if goblin.health > 0 {
                                goblin.health -= 1;
                            }
                        }
                        EventResult::OK => {
                            //
                        }
                    };
                    outcome.accepted = true;
                    return Ok(());
                }
            }
        }
        return Err(());
    }
}

pub type GoblinMap = HashMap<Player, Goblin>;

pub type GoblinOrder = HashMap<u8, Player>;

pub type GoblinOwners = HashMap<Pubkey, Player>;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum AdventureState {
    Preparing(GoblinMap, Settings),
    Started(GoblinMap, Settings, Turn, AdventurePhase),
    Complete(GoblinMap, Settings),
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub num_rounds: u8,
    pub goblin_order: GoblinOrder,
    pub goblin_owners: GoblinOwners,
    pub heroes: HashMap<HeroKind, usize>,
}
impl Settings {
    pub fn new() -> Self {
        Self {
            num_rounds: 10,
            goblin_order: HashMap::from([(0, Player::P1)]),
            goblin_owners: HashMap::new(),
            heroes: HashMap::from([
                (HeroKind::Thief, 0),
                (HeroKind::Wizard, 0),
                (HeroKind::Warrior, 0),
                (HeroKind::Merchant, 0),
            ]),
        }
    }
    pub fn update_goblin_order(&mut self, goblins: &mut GoblinMap) {
        self.goblin_order = HashMap::new();
        let players = &[Player::P1, Player::P2, Player::P3, Player::P4];
        let mut i = 0;
        for player in players {
            if goblins.contains_key(player) {
                self.goblin_order.insert(i, *player);
                i += 1;
            }
        }
        let greed = 4;
        for (k, v) in self.goblin_order.iter() {
            let goblin = goblins.get_mut(v).unwrap();
            goblin.greed = greed - *k;
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Turn {
    pub nonce: u8,
    pub num_events: u8,
    pub player: Player,
}
impl Turn {
    pub fn new(player: Player) -> Self {
        Self {
            nonce: 0,
            num_events: 0,
            player,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum AdventurePhase {
    Camp(CampPhase),
    Event(EventPhase),
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EventPhase {
    pub location: usize,
    pub scenario: usize,
    pub outcome: Option<EventPhaseOutcome>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EventPhaseOutcome {
    pub choice: usize,
    pub effect: usize,
    pub accepted: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct CampPhase {
    pub rummage_result: Option<RummageResult>,
    pub bribe_result: Option<BribeResult>,
}
impl CampPhase {
    pub fn new() -> Self {
        Self {
            rummage_result: None,
            bribe_result: None,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum RummageResult {
    Fail,
    Success { loot: Loot, did_take: Option<bool> },
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct BribeResult {
    pub hero: HeroKind,
    pub got: ItemKind,
    pub confirmed: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Goblin {
    pub health: u8,
    pub luck: u8,
    pub greed: u8,
    pub items: Vec<ItemKind>,
    pub loot: Vec<Loot>,
}
impl Goblin {
    pub const MAX_ITEMS_LEN: usize = 1;
    pub const MAX_LOOT_LEN: usize = 32;
    pub const SIZE: usize = //
        1 + 8 + // owner
        1 + // health
        1 + // luck
        1 + // greed
        Self::MAX_ITEMS_LEN * ItemKind::SIZE + // items
        Self::MAX_LOOT_LEN * Rarity::SIZE; // loot
    pub fn new() -> Self {
        Self {
            health: 2,
            luck: 0,
            greed: 0,
            items: vec![],
            loot: vec![],
        }
    }
}

#[derive(
    BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Player {
    P1,
    P2,
    P3,
    P4,
}
impl Player {
    pub const SIZE: usize = 1;
    pub fn index(&self) -> usize {
        match self {
            Self::P1 => 0,
            Self::P2 => 1,
            Self::P3 => 2,
            Self::P4 => 3,
        }
    }
}

#[derive(
    BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum HeroKind {
    Thief,
    Wizard,
    Warrior,
    Merchant,
    Ninja,
}
impl HeroKind {
    pub const SIZE: usize = 1;
}

#[derive(
    BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum ItemKind {
    Foo,
    Bar,
    Baz,
    Qux,
}
impl ItemKind {
    pub const SIZE: usize = 1;
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Loot {
    pub rarity: Rarity,
}

#[derive(
    BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    Epic,
}
impl Rarity {
    pub const SIZE: usize = 1;
}

#[derive(
    BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum EventResult {
    GetLoot,       // "Found some valuable loot"
    GetItem,       // "Discovered a useful item"
    StealLoot,     // "Successfully stole loot"
    StealItem,     // "Snatched a handy item"
    Heal,          // "Recovered some health"
    BoostLuck,     // "Feeling luckier now"
    ReduceGreed,   // "Greed slightly reduced"
    LoseLoot,      // "Lost some of your loot"
    LoseItem,      // "Misplaced an item"
    LootGotStolen, // "Your loot was stolen"
    ItemGotStolen, // "An item was pilfered"
    SlapFight,     // "Engaged in a slap fight"
    GetAttacked,   // "Faced a sudden attack"
    OK,            // "Nothing eventful occurs"
}

impl EventResult {
    pub const LEN: usize = 14;
    pub const ALL: &'static [Self] = &[
        Self::GetLoot,
        Self::GetItem,
        Self::StealLoot,
        Self::StealItem,
        Self::Heal,
        Self::BoostLuck,
        Self::ReduceGreed,
        Self::LoseLoot,
        Self::LoseItem,
        Self::LootGotStolen,
        Self::ItemGotStolen,
        Self::SlapFight,
        Self::GetAttacked,
        Self::OK,
    ];
    pub fn desc(&self) -> &'static str {
        match self {
            Self::GetLoot => "Found some valuable loot.",
            Self::GetItem => "Discovered an item.",
            Self::StealLoot => "Stole some loot.",
            Self::StealItem => "Snatched a handy item",
            Self::Heal => "Recovered some health",
            Self::BoostLuck => "Your luck increased.",
            Self::ReduceGreed => "You feel less greedy.",
            Self::LoseLoot => "Lost some of your loot.",
            Self::LoseItem => "You misplaced an item.",
            Self::LootGotStolen => "Your loot was stolen.",
            Self::ItemGotStolen => "One of your items was stolen.",
            Self::SlapFight => "Slap fight initiated!",
            Self::GetAttacked => "You are ambushed by a hidden foe!",
            Self::OK => "Nothing eventful occurs.",
        }
    }
    pub fn is_good(&self) -> bool {
        match self {
            Self::GetLoot => true,
            Self::GetItem => true,
            Self::StealLoot => true,
            Self::StealItem => true,
            Self::Heal => true,
            Self::BoostLuck => true,
            Self::ReduceGreed => true,
            Self::LoseLoot => false,
            Self::LoseItem => false,
            Self::LootGotStolen => false,
            Self::ItemGotStolen => false,
            Self::SlapFight => false,
            Self::GetAttacked => false,
            Self::OK => true,
        }
    }
    #[rustfmt::skip]
    pub fn goblin_dialog(&self) -> &'static str {
        match self {
            Self::BoostLuck => "I smell loot. I must have Gob's favor.",
            Self::GetAttacked => "Wot the 'eck! Who's pokin' me bum?",
            Self::GetItem => "I've nabbed a fancy trinket! It's mine, I say!",
            Self::GetLoot => "Ooh, shiny! This'll fetch a nice price.",
            Self::Heal => "Ouchies all gone! Tough as a dragon's hind am I!",
            Self::ItemGotStolen => "Oi! Who's the sneak thief pinchin' me treasures?",
            Self::LootGotStolen => "Someone's pinched me precious loot! Cheeky blighter!",
            Self::LoseItem => "Drat! Lost me thingamajig! Where'd it get off to?",
            Self::LoseLoot => "Me loot! It's gone! This is a right mess.",
            Self::OK => "All quiet... too quiet. But heck, I'll take it!",
            Self::ReduceGreed => "Maybe bein' stupid filthy rich ain't all it's cracked up to be... Who said that?",
            Self::SlapFight => "Slappin' time! Best part of the day, this is!",
            Self::StealItem => "Hehe, this'll be my little secret, yeah?",
            Self::StealLoot => "Yoink! This loot's better off with me.",
        }
    }
}

// #[derive(
//     BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
// )]
// pub enum EventLocation {
//     ArchedHall,
//     BrightCavern,
//     GenericCave,
//     CaveWithExit,
//     DarkCave,
//     Hallway,
//     LushCavern,
//     ThroneRoom,
//     TreasureRoom,
// }
// impl EventLocation {
//     pub const ALL: &'static [EventLocation] = &[
//         Self::ArchedHall,
//         Self::BrightCavern,
//         Self::GenericCave,
//         Self::CaveWithExit,
//         Self::DarkCave,
//         Self::Hallway,
//         Self::LushCavern,
//         Self::ThroneRoom,
//         Self::TreasureRoom,
//     ];
//     pub fn from_index(i: usize) -> Self {
//         Self::ALL[i % Self::ALL.len()]
//     }
//     pub fn index(&self) -> usize {
//         match self {
//             Self::ArchedHall => 0,
//             Self::BrightCavern => 1,
//             Self::GenericCave => 2,
//             Self::CaveWithExit => 3,
//             Self::DarkCave => 4,
//             Self::Hallway => 5,
//             Self::LushCavern => 6,
//             Self::ThroneRoom => 7,
//             Self::TreasureRoom => 8,
//         }
//     }
//     pub fn next(&self) -> usize {
//         (self.index() + 1) % Self::ALL.len()
//     }
//     pub fn prev(&self) -> usize {
//         let i = self.index();
//         if i == 0 {
//             return Self::ALL.len() - 1;
//         }
//         return i - 1;
//     }
// }
