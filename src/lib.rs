use loot_goblin::{EventOutcome, Game};
use turbo::solana::{
    anchor::Program,
    solana_sdk::{hash::Hash, instruction::AccountMeta, pubkey::Pubkey, system_program},
};

const TX_COOLDOWN_DUR: u32 = 0; //60 * 15;
const DIALOG_HEIGHT: i32 = 32;

turbo::cfg! {r#"
    name = "Loot Goblin"
    version = "0.0.0-alpha.0"
    author = "Turbo"
    [settings]
    resolution = [256, 256]
    [solana]
    http-rpc-url = "http://localhost:8899"
    ws-rpc-url = "ws://localhost:8900"
"#}

turbo::init! {
    struct GameState {
        screen: enum Screen {
            TitleMenu,
            GameMenu { focused: usize, is_starting: bool },
            LoadedGame { id: u8 },
        },
        cooldown_timer: u32,
    } = {
        Self {
            screen: Screen::TitleMenu,
            cooldown_timer: 0,
        }
    }
}

turbo::go! {
    let mut state = GameState::load();
    let state_ptr = &mut state as *mut GameState;

    
    set_camera(0, 0);
    clear(0x000000ff);

    let user_pubkey = solana::user_pubkey();
    let games = [0, 1, 2].map(|game_id| {
        let (pk, _) = get_game_pubkey(game_id);
        (pk, solana::rpc::get_account(&pk))
    });

    let gp = gamepad(0);
    match state.screen {
        Screen::TitleMenu => {
            sprite!("title_menu_bg", fps = 0);
            let [w, h] = resolution();
            if user_pubkey == Pubkey::default() {
                let msg = "Please connect your wallet...";
                let mw = (msg.len() * 5) as u32;
                let x = (w as i32 / 2) - (mw as i32 / 2);
                let y = (h as i32 / 2) + 20;
                rect!(w = mw + 4, h = 10, x = x, y = y, fill = 0x00000ef);
                text!(msg, x = x + 2, y = y + 2);
            } else {
                let msg = &format!("User Pubkey: {}...", &user_pubkey.to_string()[..8]);
                text!(msg, y = h as i32 - 6, font = Font::S);
                let msg = "Press A to start";
                let mw = (msg.len() * 5) as u32;
                let x = (w as i32 / 2) - (mw as i32 / 2);
                let y = (h as i32 / 2) + 20;
                rect!(w = mw + 4, h = 10, x = x, y = y, fill = 0x00000ef);
                text!("Press A to Start", x = x + 2, y = y + 2);
                if gp.a.just_pressed() {
                    state.screen = Screen::GameMenu { focused: 0, is_starting: false };
                }
            }
        }
        Screen::GameMenu { ref mut focused, ref mut is_starting } => {
            let user_str = &format!("User: {}", user_pubkey.to_string());
            text!(user_str);
            set_camera(0, 8);
            for i in 0..games.len() {
                let pre = if *focused == i {
                    ">"
                } else {
                    " "
                };
                let msg = &format!("{} Slot {} - {} ({})", pre, i + 1, games[i].1.value.as_ref().map(|_val| {
                    "Continue"
                }).unwrap_or("Empty"), games[i].0);
                text!(msg, y = 8 * i as i32);
                let [_x, y] = get_camera();
                set_camera(0, y + 8);
            }
            if gp.b.just_pressed() {
                (*state_ptr).screen = Screen::TitleMenu;
            }
            if !*is_starting {
                if gp.up.just_pressed() {
                    *focused = focused.checked_sub(1).unwrap_or(2);
                } else if gp.down.just_pressed() {
                    *focused = (*focused + 1) % 3;
                }
            }
            if gp.a.just_pressed() {
                if games[*focused].1.value.is_some() {
                    state.screen = Screen::LoadedGame { id: *focused as u8 };
                } else {
                    let hash = create_game(*focused as u8, 10);
                    turbo::println!("{:?}", hash);
                    *is_starting = true;
                }
            }
        }
        Screen::LoadedGame { id } => {
            let (_game_p, game_result) = &games[id as usize];
            static mut slot: u64 = 0;
            let did_update = if slot != game_result.context.slot.unwrap_or(0) {
                slot = game_result.context.slot.unwrap_or(0);
                true
            } else {
                false
            };
            if let Some(ref account) = game_result.value {
                // match solana::anchor::try_from_slice::<Game>(&account.data) {
                match Game::deserialize(&mut account.data.get(8..).unwrap_or(&[])) {
                    Ok(game) => {
                        if did_update {
                            turbo::println!("--------------------------------");
                            turbo::println!("goblin = {:?}", game.turn_goblin);
                            turbo::println!("turn count = {:?}", game.turn_count);
                            turbo::println!("turn phase = {:?}", game.turn_phase);
                            turbo::println!("no events = {:?}", game.turn_events);
                        }
                        match game.game_phase  {
                            Game::GAME_PHASE_NEW_GAME => {
                                text!("Game Phase: New Game", y = 0);
                            }
                            Game::GAME_PHASE_RECRUIT_GOBLINS => {
                                text!("Game Phase: Recruit Goblins", y = 0);
                                if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                    recruit_goblins(id);
                                }
                                let debug = format!("{:#?}", game);
                                text!(&debug, y = 10, font = Font::S);
                            }
                            Game::GAME_PHASE_FIND_GREEDIEST => {
                                text!("Game Phase: Find Greediest", y = 0);
                                if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                    find_greediest_goblin(id);
                                    state.cooldown_timer = TX_COOLDOWN_DUR;
                                }
                                let debug = format!("{:#?}", game);
                                text!(&debug, y = 10, font = Font::S);
                            }
                            Game::GAME_PHASE_CRAWL_STARTED => match game.turn_phase {
                                Game::TURN_PHASE_RUMMAGE => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("Rummaging!");
                                        rummage_through_loot_sack(id);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                Game::TURN_PHASE_BRIBE => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("Bribing!");
                                        bribe_hero(id);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                Game::TURN_PHASE_ITEM => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("Using item!");
                                        use_item(id);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                // Explore
                                Game::TURN_PHASE_EVENT => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("Triggering event!");
                                        trigger_event(id);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                // Fuck around
                                Game::TURN_PHASE_OUTCOME => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("Determining outcome! Option A");
                                        determine_outcome(id, 0);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                    if state.cooldown_timer == 0 && gp.b.just_pressed() {
                                        turbo::println!("Determining outcome! Option B");
                                        determine_outcome(id, 1);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                // Find out
                                Game::TURN_PHASE_AFTERMATH => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("Making aftermath decision!");
                                        make_aftermath_decision(id);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                Game::TURN_PHASE_SLAP_FIGHT => {
                                    draw_status_bar(&state, &game);
                                    draw_event_panel(&game);
                                    draw_left_panel(&game);
                                    if state.cooldown_timer == 0 && gp.a.just_pressed() {
                                        turbo::println!("SLAP FIGHT!");
                                        slap_fight(id);
                                        state.cooldown_timer = TX_COOLDOWN_DUR;
                                    }
                                }
                                _ => {
                                    text!("Turn Phase: Unknown!", y = 0);
                                }
                            }
                            Game::GAME_PHASE_CRAWL_ENDED => {
                                text!("Game Phase: Crawl Ended!", y = 0);
                            }
                            n => {
                                let debug = format!("Unknown game phase: {:#?}", n);
                                text!(&debug, y = 0);
                            }
                        }
                    }
                    Err(err) => {
                        let debug = format!("{:#?}", err);
                        text!(&debug, y = 10);
                    }
                }
            }
        }
    }

    let m = mouse(0);
    let [mx, my] = m.position;
    circ!(d = 8, x = mx - 4, y = my - 4, fill = 0xff00ffff);

    if state.cooldown_timer > 0 {
        state.cooldown_timer -= 1;
    }

    state.save();
}

fn get_game_pubkey(game_id: u8) -> (Pubkey, u8) {
    let user_pubkey = solana::user_pubkey();
    Pubkey::find_program_address(
        &[b"game", user_pubkey.as_ref(), &[game_id]],
        &loot_goblin::ID,
    )
}

fn create_game(game_id: u8, game_rounds: u8) -> Hash {
    let instruction_name = "create_game";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::CreateGame {
        game_id,
        game_rounds,
    };
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn recruit_goblins(game_id: u8) -> Hash {
    let instruction_name = "recruit_goblins";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::RecruitGoblins {
        num_goblins: 2,
        players: vec![user_pubkey],
    };
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn find_greediest_goblin(game_id: u8) -> Hash {
    let instruction_name = "find_greediest_goblin";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::FindGreediestGoblin {};
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn rummage_through_loot_sack(game_id: u8) -> Hash {
    let instruction_name = "rummage_through_loot_sack";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::FindGreediestGoblin {};
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn bribe_hero(game_id: u8) -> Hash {
    let instruction_name = "bribe_hero";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::BribeHero {
        did_bribe: false,
        hero_index: 0,
        loot_index: 0,
    };
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn use_item(game_id: u8) -> Hash {
    let instruction_name = "use_item";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::UseItem { use_item: false };
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn trigger_event(game_id: u8) -> Hash {
    let instruction_name = "trigger_event";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::TriggerEvent {};
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn determine_outcome(game_id: u8, choice: u8) -> Hash {
    let instruction_name = "determine_outcome";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::DetermineOutcome { choice };
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn make_aftermath_decision(game_id: u8) -> Hash {
    let instruction_name = "make_aftermath_decision";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::MakeAftermathDecision { choice: 1 };
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn slap_fight(game_id: u8) -> Hash {
    let instruction_name = "slap_fight";
    let user_pubkey = solana::user_pubkey();
    let (game_pubkey, _bump) = get_game_pubkey(game_id);
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(user_pubkey, true),
        AccountMeta::new(game_pubkey, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let args = loot_goblin::instruction::SlapFight {};
    Program::new(loot_goblin::ID)
        .instruction(instruction_name)
        .accounts(accounts.clone())
        .args(args)
        .send()
}

fn to_event_outcome(outcome: u8) -> EventOutcome {
    unsafe { std::mem::transmute(outcome) }
}

unsafe fn event_outcome_str(outcome: u8) -> &'static str {
    match std::mem::transmute(outcome) {
        EventOutcome::BoostLuck => "A stroke of good fortune seems to brighten your path.",
        EventOutcome::GetAttacked => "Suddenly, an unseen assailant leaps from the shadows!",
        EventOutcome::GetItem => "You discover a curious item, shimmering with potential.",
        EventOutcome::GetLoot => "A hidden cache of loot reveals itself, ripe for the taking.",
        EventOutcome::Heal => "A soothing energy washes over you, mending your wounds.",
        EventOutcome::ItemGotStolen => "Alas, one of your treasured items has been pilfered!",
        EventOutcome::LootGotStolen => "Your hard-earned loot has been swiped by cunning hands.",
        EventOutcome::LoseItem => "In a moment of carelessness, you lose a valuable item.",
        EventOutcome::LoseLoot => "Misfortune strikes as you lose some of your gathered loot.",
        EventOutcome::OK => "The moment passes uneventfully, with calm prevailing.",
        EventOutcome::ReduceGreed => "A sense of contentment tempers your burning greed.",
        EventOutcome::SlapFight => "A comical slap fight ensues, echoing with playful thuds.",
        EventOutcome::StealItem => {
            "You slyly manage to pilfer an item from an unsuspecting source."
        }
        EventOutcome::StealLoot => {
            "Quick and cunning, you snatch some loot right under their noses."
        }
    }
}

fn bg_sprite(n: u8) -> &'static str {
    match n {
        0 => "arched_hallway",
        1 => "bright_cavern",
        2 => "cave_1",
        3 => "cave_exit",
        4 => "dark_cave",
        5 => "hallway",
        6 => "lush_cavern_1",
        7 => "lush_cavern_2",
        8 => "throne_room",
        _ => "treasure_room",
    }
}

fn turn_phase_str(n: u8) -> &'static str {
    match n {
        Game::TURN_PHASE_AFTERMATH => "Aftermath",
        Game::TURN_PHASE_BRIBE => "Bribe",
        Game::TURN_PHASE_EVENT => "Event",
        Game::TURN_PHASE_ITEM => "Item",
        Game::TURN_PHASE_OUTCOME => "Outcome",
        Game::TURN_PHASE_RUMMAGE => "Rummage",
        Game::TURN_PHASE_SLAP_FIGHT => "Slap Fight",
        _ => "Unknown",
    }
}

fn draw_top_panel(game: &Game, msg: &str) {
    set_camera(0, 0);
    let goblin_key = &format!("goblin_{}", game.turn_goblin + 1);
    sprite!(goblin_key, x = 1, y = 0);
    set_camera(38, 4);
    text!(msg);
}

fn draw_left_panel(game: &Game) {
    let top = DIALOG_HEIGHT;
    let left = 2;
    set_camera(left, top + 2);
    for i in 0..game.num_goblins {
        let y = i as i32 * 20;
        let n = i + 1;
        let key = &format!("goblin_smol_{n}");
        if i == game.turn_goblin {
            #[rustfmt::skip]
            rectv!(w = 24, h = 19, fill = 0xffffffff, x = -1, y = y, border = Border {
                radius: 5,
                size: 1,
                color: 0x000000ff,
            });
            sprite!(key, x = 2, y = y + 2);
        } else {
            // circ!(d = 18, fill = 0x222222ff, y = y);
            #[rustfmt::skip]
            rectv!(w = 24, h = 19, fill = 0x222222ff, x = -1, y = y, border = Border {
                radius: 5,
                size: 1,
                color: 0x000000ff,
            });
            sprite!(key, x = 2, y = y + 2);
        }
    }
    let left = 22;
    set_camera(left, top);
    let [_w, h] = resolution();
    let w = 103;
    let h = h - 20;
    rectv!(
        w = w + 1,
        h = h - top as u32,
        x = 0,
        fill = 0x000000ff,
        border = Border {
            radius: 3,
            size: 1,
            color: 0xffffffff,
        }
    );
    let turn_goblin = game.turn_goblin as usize;
    let goblin = game.goblins[turn_goblin];
    // set_camera(0, 45);
    // rect!(w = w, h = 1);

    set_camera(left + 4, top + 4);
    let mut y = 0;
    text!(&format!("Goblin {}", turn_goblin + 1,));
    if goblin.player == Pubkey::default() {
        rect!(w = 15, h = 6, x = 44, y = 0);
        text!("CPU", font = Font::S, x = 45, y = 1, color = 0x000000ff);
    }
    rect!(w = w - 7, h = 1, y = y + 9, fill = 0x222222ff);
    y += 13;
    let attributes = [
        ("health", &goblin.health.to_string()),
        ("luck  ", &goblin.luck.to_string()),
        ("greed ", &goblin.greed.to_string()),
        ("wealth", &goblin.loot_bag.iter().sum::<u8>().to_string()),
    ];
    for (key, val) in attributes {
        text!(&format!("{key}: {val}"), font = Font::S, y = y);
        y += 8;
    }
    rect!(w = w - 7, h = 1, y = y, fill = 0x222222ff);
    y += 5;
    text!("Loot Bag", y = y, font = Font::S);
    y += 4;
    set_camera(left + 3, top + y + 8);
    for i in 0..32 {
        let cols = 7;
        let x = (i % cols) * 14;
        let y = (i / cols) * 14;
        #[rustfmt::skip]
        rectv!(w = 14, h = 14, fill = 0x222222ff, x = x, y = y, border = Border {
            radius: 3,
            size: 1,
            color: 0x00000000,
        });
        let loot = goblin.loot_bag[i as usize];
        match loot {
            0 => {
                //
            }
            n if n <= 5 => {
                let key = &format!("loot_{n}");
                sprite!(key, x = x + 1, y = y + 1);
                text!(&loot.to_string(), font = Font::S, x = x + 9, y = y + 9);
            }
            _ => {
                sprite!("gem", x = x + 1, y = y + 1);
                text!(&loot.to_string(), font = Font::S, x = x + 8, y = y + 8);
            }
        }
        if loot > 0 {}
    }
}

unsafe fn draw_event_panel(game: &Game) {
    let [_w, h] = resolution();
    let h = h - 10;
    let x = 128;
    let y = DIALOG_HEIGHT;
    static mut bg: u8 = 255;
    if bg == 255 {
        bg = game.event % 10;
    }
    let gp = gamepad(0);
    if gp.left.just_pressed() {
        bg = bg.checked_sub(1).unwrap_or(9);
    }
    if gp.right.just_pressed() {
        bg = bg.saturating_add(1) % 10;
    }
    let tx = x + 4;
    let ty = y + 130;
    match game.turn_phase {
        Game::TURN_PHASE_RUMMAGE => {
            draw_top_panel(
                &game,
                &goblin_dialog(
                    "Not my fault they left their sack of loot unattended. A goblin must do what a goblin must do!",
                ),
            );
            set_camera(x, y);
            sprite!("camp");
            set_camera(tx - 1, ty);
            text!(&format!("{}", insert_line_breaks("The flickering campfire casts a warm glow, offering a brief respite from the adventurers' relentless journey.", 24)));
            let ty = ty + 54;
            set_camera(tx - 2, ty);
            #[rustfmt::skip]
            rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                radius: 3,
                size: 1,
                color: 0xffffffff,
            });
            text!("Rummage through loot", color = 0xffffffff, x = 6, y = 6);
            circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
            circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
            text!("A", font = Font::S, color = 0x000000ff, x = 116, y = 9);
        }
        Game::TURN_PHASE_ITEM => {
            draw_top_panel(
                &game,
                &goblin_dialog("Maybe I should see what this thing does..."),
            );
            set_camera(x, y);
            sprite!("camp");
            set_camera(tx, ty);
            if game.goblins[game.turn_goblin as usize].held_item == 0 {
                text!("No items to use :(\nPress A");
            } else {
                text!("Use an item?\nPress A or B");
            }
        }
        Game::TURN_PHASE_BRIBE => {
            draw_top_panel(&game, "\"Perhaps I can pay for\nsome assistance...\"");
            set_camera(x, y);
            sprite!("camp");
            set_camera(tx, ty);
            let did_win =
                game.rummage_success_min <= game.goblins[game.turn_goblin as usize].last_roll;
            let msg = &format!(
                "Rummage {}\nBribe a hero?",
                if did_win { "successful!" } else { "failed!" }
            );
            text!(msg);
        }
        Game::TURN_PHASE_EVENT => {
            draw_top_panel(&game, "\"I smell loot. Time to explore\"");
            set_camera(x, y);
            sprite!("camp");
            set_camera(tx, ty);
            text!("Ready to move on?\nPress A");
        }
        Game::TURN_PHASE_OUTCOME => {
            draw_top_panel(
                &game,
                &goblin_dialog(match bg {
                    // arched hallway
                    0 => "Must've been a great place for a party. Still reeks of stale mead.",
                    // bright cavern
                    1 => "Hmmm...Looks like there's something shiny at the end of this hallway",
                    // cave 1
                    2 => "Darker than a dungeon down here, ain't it?",
                    // cave exit
                    3 => "A pleasant passageway. Surely, it leads to fortune.",
                    // dark cave
                    4 => "Can barely see me own toes in here. If I'm bein' honest, maybe it's for the best...",
                    // hallway
                    5 => "This hallway gives me the creeps. Should I go anyways?",
                    // lush cavern 1
                    6 => "All these plants... I bet there's treasure hidden here!",
                    // lush cavern 2
                    7 => "So green and pretty! Must keep me eyes on the prize.",
                    // throne room
                    8 => "A throne room! Wonder if there's a crown for me noggin here.",
                    // treasure room
                    9 => "Now that's quite the pile o' loot, innit?",
                    // unknown
                    _ => "Where am I? This ain't the pub...",
                }),
            );
            set_camera(x, y);
            sprite!(bg_sprite(bg));
            set_camera(tx - 1, ty);
            text!(&format!(
                "{}",
                &insert_line_breaks(
                    match bg {
                        // arched hallway
                        0 => "An elegant arched hallway, echoing memories of grand feasts.",
                        // bright cavern
                        1 => "A cavern aglow with a mysterious light, hinting at hidden treasures.",
                        // cave 1
                        2 => "A dark, foreboding cave, shrouded in shadows and silence.",
                        // cave exit
                        3 =>
                            "The cave's exit looms ahead, promising a return to the outside world.",
                        // dark cave
                        4 => "An abyssal dark cave, where every sound and movement is amplified.",
                        // hallway
                        5 => "A long, eerie hallway, stretching into the unknown.",
                        // lush cavern 1
                        6 => "A cavern overgrown with lush vegetation, a rare sight underground.",
                        // lush cavern 2
                        7 => "A verdant cavern, its air fresh with the scent of moss and earth.",
                        // throne room
                        8 => "An abandoned throne room, its grandeur faded with time.",
                        // treasure room
                        9 => "A hidden chamber, filled to the brim with glittering treasures.",
                        _ => "An unfamiliar place, its features obscured and mysterious.",
                    },
                    24
                )
            ));
            let ty = ty + 34;
            set_camera(tx - 2, ty);
            #[rustfmt::skip]
            rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                radius: 3,
                size: 1,
                color: 0xffffffff,
            });
            text!("Risk it", color = 0xffffffff, x = 6, y = 6);
            circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
            circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
            text!("A", font = Font::S, color = 0x000000ff, x = 116, y = 9);
            let ty = ty + 20;
            set_camera(tx - 2, ty);
            #[rustfmt::skip]
            rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                radius: 3,
                size: 1,
                color: 0xffffffff,
            });
            text!("Stay safe", color = 0xffffffff, x = 6, y = 6);
            circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
            circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
            text!("B", font = Font::S, color = 0x000000ff, x = 116, y = 9);
        }
        Game::TURN_PHASE_AFTERMATH => {
            set_camera(x, y);
            sprite!(bg_sprite(bg));
            draw_top_panel(
                &game,
                &goblin_dialog(match to_event_outcome(game.event_outcome) {
                    EventOutcome::BoostLuck => "I smell loot. I must have Gob's favor.",
                    EventOutcome::GetAttacked => "Wot the 'eck! Who's pokin' me bum?",
                    EventOutcome::GetItem => "I've nabbed a fancy trinket! It's mine, I say!",
                    EventOutcome::GetLoot => "Ooh, shiny! This'll fetch a nice price.",
                    EventOutcome::Heal => "Ouchies all gone! Tough as a dragon's hind am I!",
                    EventOutcome::ItemGotStolen => {
                        "Oi! Who's the sneak thief pinchin' me treasures?"
                    }
                    EventOutcome::LootGotStolen => {
                        "Someone's pinched me precious loot! Cheeky blighter!"
                    }
                    EventOutcome::LoseItem => "Drat! Lost me thingamajig! Where'd it get off to?",
                    EventOutcome::LoseLoot => "Me loot! It's gone! This is a right mess.",
                    EventOutcome::OK => "All quiet... too quiet. But heck, I'll take it!",
                    EventOutcome::ReduceGreed => {
                        "Maybe bein' stupid filthy rich ain't all it's cracked up to be... Who said that?"
                    }
                    EventOutcome::SlapFight => "Slappin' time! Best part of the day, this is!",
                    EventOutcome::StealItem => "Hehe, this'll be my little secret, yeah?",
                    EventOutcome::StealLoot => "Yoink! This loot's better off with me.",
                }),
            );
            set_camera(tx - 1, ty);
            text!(&insert_line_breaks(
                event_outcome_str(game.event_outcome),
                23
            ));
            if game.aftermath_option == Game::AFTERMATH_OPTION_CONTINUE {
                let ty = ty + 54;
                set_camera(tx - 2, ty);
                #[rustfmt::skip]
                rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                    radius: 3,
                    size: 1,
                    color: 0xffffffff,
                });
                text!("Continue", color = 0xffffffff, x = 6, y = 6);
                circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
                circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
                text!("A", font = Font::S, color = 0x000000ff, x = 116, y = 9);
            } else if game.aftermath_option == Game::AFTERMATH_OPTION_STOP {
                let ty = ty + 54;
                set_camera(tx - 2, ty);
                #[rustfmt::skip]
                rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                    radius: 3,
                    size: 1,
                    color: 0xffffffff,
                });
                text!("Take a break", color = 0xffffffff, x = 6, y = 6);
                circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
                circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
                text!("A", font = Font::S, color = 0x000000ff, x = 116, y = 9);
            } else {
                let ty = ty + 34;
                set_camera(tx - 2, ty);
                #[rustfmt::skip]
                rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                    radius: 3,
                    size: 1,
                    color: 0xffffffff,
                });
                text!("Continue", color = 0xffffffff, x = 6, y = 6);
                circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
                circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
                text!("A", font = Font::S, color = 0x000000ff, x = 116, y = 9);
                let ty = ty + 20;
                set_camera(tx - 2, ty);
                #[rustfmt::skip]
                rectv!(w = 124, h = 18, fill = 0x000000ff, border = Border {
                    radius: 3,
                    size: 1,
                    color: 0xffffffff,
                });
                text!("Take a break", color = 0xffffffff, x = 6, y = 6);
                circ!(d = 7, x = 114, y = 8, fill = 0xffffffff);
                circ!(d = 5, x = 115, y = 9, fill = 0xffffffff);
                text!("B", font = Font::S, color = 0x000000ff, x = 116, y = 9);
            }
        }
        Game::TURN_PHASE_SLAP_FIGHT => {
            draw_top_panel(
                &game,
                "\"You want to snatch MY loot?\nYou can catch these hands!\"",
            );
            set_camera(x, y);
            sprite!("slap_fight");
            set_camera(tx, ty);
            text!("*Slapping noises*\nWho will triumph???");
        }
        _ => {}
    }
    set_camera(x, y);
    rect!(h = 6, w = 128, fill = 0x00000088);
    rect!(h = 5, w = 128, fill = 0x000000aa);
    rect!(h = 4, w = 128, fill = 0x000000dd);
    rect!(h = 3, w = 128, fill = 0x000000dd);
    rect!(w = 6, h = 128, fill = 0x00000088);
    rect!(w = 5, h = 128, fill = 0x000000aa);
    rect!(w = 4, h = 128, fill = 0x000000dd);
    rect!(w = 3, h = 128, fill = 0x000000dd);
    rect!(w = 6, h = 128, x = 128 - 6, fill = 0x000000aa);
    rect!(w = 5, h = 128, x = 128 - 5, fill = 0x000000cc);
    rect!(w = 4, h = 128, x = 128 - 4, fill = 0x000000dd);
    rect!(h = 3, w = 128, y = 128 - 3, fill = 0x000000dd);
    rect!(h = 6, w = 128, y = 128 - 6, fill = 0x000000aa);
    rect!(h = 5, w = 128, y = 128 - 5, fill = 0x000000cc);
    rect!(h = 4, w = 128, y = 128 - 4, fill = 0x000000dd);
    rect!(h = 3, w = 128, y = 128 - 3, fill = 0x000000dd);
    rectv!(
        w = 128,
        h = h - y as u32 - 10,
        fill = 0x00000000,
        border = Border {
            radius: 3,
            size: 1,
            color: 0xffffffff,
        }
    );
}

fn draw_status_bar(state: &GameState, game: &Game) {
    let [w, h] = resolution();
    set_camera(0, h as i32 - 20);
    rect!(w = w, h = 1, y = 4, fill = 0x888888ff);
    for i in 0..32 {
        let x = i as i32 * 8;
        let fill = if i + 1 == game.turn_count {
            0xffc832ff
        } else {
            0x888888ff
        };
        circ!(d = 5, fill = fill, x = x, y = 2);
    }
    set_camera(0, h as i32 - 10);
    rect!(w = w, h = 11, y = -1, fill = 0x76428aff);
    rect!(w = w - 56, h = 11, x = 56, y = -1, fill = 0x3f3f74ff);
    text!(
        &format!("Day > {}", game.turn_count),
        font = Font::S,
        color = 0xd77bbaff,
        x = 4,
        y = 3
    );
    text!(
        &format!("Events > {}", game.turn_events),
        font = Font::S,
        color = 0x5b6ee1ff,
        x = w as i32 - 54,
        y = 3
    );
    let progress = state.cooldown_timer as f32 / TX_COOLDOWN_DUR as f32;
    let w = (progress * w as f32) as u32;
    rect!(w = w, h = 11, fill = 0x000000cc);
}

fn insert_line_breaks(input: &str, max_line_length: usize) -> String {
    let mut result = String::new();
    let mut current_line_length = 0;

    for word in input.split_whitespace() {
        let word_length = word.chars().count();

        // Check if adding this word would exceed the line length
        if current_line_length + word_length > max_line_length {
            result.push('\n');
            current_line_length = 0;
        }

        // Add a space before the word if it's not at the start of a line
        if current_line_length > 0 {
            result.push(' ');
            current_line_length += 1;
        }

        result.push_str(word);
        current_line_length += word_length;
    }

    result
}

fn goblin_dialog(input: &str) -> String {
    format!("\"{}\"", insert_line_breaks(input, 40))
}
