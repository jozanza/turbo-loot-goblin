use std::collections::VecDeque;

pub mod data;
pub use data::*;
pub mod state;
pub use state::*;
pub mod ui;
pub use ui::*;

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
        adventure: Option<Adventure>,
        gui: struct GUI {
            commands: VecDeque<enum Command {
                GoblinList(enum GoblinListEvent {
                    OpenGoblinDialog(Player),
                    OpenGoblinLootInspector(Player),
                }),
                GoblinDialog(enum GoblinDialogEvent {
                    Close,
                    FastForward,
                    Next,
                }),
                GoblinLootInspector(enum GoblinLootInspectorEvent {
                    Close,
                    SelectLoot(usize),
                }),
                PhaseActionSection(enum PhaseActionSectionEvent {
                    Camp(enum CampPhaseAction {
                        RummageStart,
                        RummageEnd,
                        RummageConfirmFailure,
                        RummageConfirmSuccess(bool),
                        Bribe,
                        ContinueStart,
                        ContinueEnd,
                        BackToDefaultMenu,
                    })
                    Event(enum EventPhaseAction {
                        TakeRisk,
                        TakeRiskEnd,
                        PlayItSafe,
                        PlayItSafeEnd,
                        ConfirmOutcome(bool),
                        KeepGoingStart,
                        KeepGoingEnd,
                        TakeABreakStart,
                        TakeABreakEnd,

                    })
                })
            }>,
            phase_actions_section: struct PhaseActionsSection {
                camp: enum CampActionMenu {
                    Default,
                    RummageResult,
                    BribeResult,
                }
            },
            goblin_list: struct GoblinList {},
            loot_inspector: Option<struct GoblinLootInspector {
                player: Player,
                selected: Option<usize>,
            }>,
            goblin_dialog: Option<struct GoblinDialog {
                player: Player,
                message: String,
                max_len: usize,
                on_close: Option<Command>,
            }>,
        }
    } = {
        Self {
            screen: Screen::TitleMenu,
            cooldown_timer: 0,
            adventure: None,
            gui: GUI {
                commands: VecDeque::new(),
                phase_actions_section: PhaseActionsSection {
                    camp: CampActionMenu::Default,
                },
                goblin_list: GoblinList {},
                loot_inspector: None,
                goblin_dialog: None,
            }
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    set_camera(0, 0);
    clear(0x000000ff);

    if None == state.adventure {
        sprite!("title_bg_2");
        if mouse(0).left.just_released() {
            let _ = state.adventure.insert(Adventure::new(solana::user_pubkey()));
        }
        draw_cursor();
        state.save();
        return;
    }
    if state.adventure.is_some() {
        let mut go_to_title = false;
        if let Some(ref mut adventure) = state.adventure {
            match &mut &mut adventure.state {
                AdventureState::Preparing(ref mut goblins, ref mut settings) => {
                    sprite!("parchment_bg");
                    let [sw, sh] = resolution();
                    let (x, y) = (8, 4);
                    rect!(w = sw, h = 16, fill = BG);
                    text!("NEW GAME > SETTINGS", font = Font::L, x = x, y = y, color = FG);
                    let y = y + 24;

                    // Goblins
                    text!("Players", x = x, y = y, color = FG);
                    let y = y + 12;
                    text!("How many goblins in your party?", x = x, y = y, font = Font::S, color = FG);
                    let y = y + 12;
                    let w = (sw - (x * 2) as u32) / 4;
                    let players = &[Player::P1, Player::P2, Player::P3, Player::P4];
                    for (i, player) in players.iter().enumerate() {
                        let x = x + (i as i32 % 4) * w as i32;
                        let y = y + (i as i32 / 4) * w as i32;
                        div(w - 1, w, x, y);
                        sprite!(&format!("goblin_{}", i + 1), x = x + 12, y = y + 16);
                        if *player != Player::P1 {
                            if goblins.contains_key(&player) {
                                if button(Font::M, x + 1, y + 61, " Remove  ") {
                                    turbo::println!("Remove player!");
                                    goblins.remove(player);
                                };
                            } else {
                                rect!(w = 32, h = 32, x = x + 12, y = y + 16, fill = 0x000000ee);
                                                                    //  Recruit
                                if ibutton(Font::M, x + 1, y + 61, " Recruit ") {
                                    turbo::println!("Add player!");
                                    goblins.insert(*player, Goblin::new());
                                };
                            }
                            text!(&format!("{:?}", player), x = x + 4, y = y + 4, color = FG);
                        } else {
                            // ibutton(Font::M, x + 1, y + 61, "   YOU   ");
                            text!(&format!("{:?}", player), x = x + 4, y = y + 4, color = FG);
                        }
                    }
                    let y = y + 96;

                    // Rounds
                    text!("Rounds", x = x, y = y, color = FG);
                    let y = y + 12;
                    text!("How long should your adventure last?", x = x, y = y, font = Font::S, color = FG);
                    let y = y + 12;
                    div(32, 32, x, y);
                    text!(&format!("{:0>3}", settings.num_rounds), x = x + 9, y = y + 13, color = FG);
                    let x = x + 33;
                    if ibutton(Font::M, x, y, "+") {
                        turbo::println!("INCREASE!");
                        settings.num_rounds += 1;
                    };
                    let y = y + 16;
                    if ibutton(Font::M, x, y, "-") {
                        turbo::println!("DECREASE!");
                        settings.num_rounds -= 1;
                    };

                    // Next
                    let sh = sh as i32;
                    let x = 4 + 4;
                    let y = sh - 32;
                    if button(Font::L, x, y, "    BACK    ") {
                        turbo::println!("BACK");
                        go_to_title = true;
                    };
                    if ibutton(Font::L, x + 128, y, "   START >  ") {
                        turbo::println!("START");
                        if adventure.start_adventure().is_ok() {
                            let msg = ENTERING_CAMP_DIALOG[rand() as usize % ENTERING_CAMP_DIALOG.len()];
                            state.gui.open_goblin_dialog(Player::P1, msg, None);
                        }
                    };
                }
                AdventureState::Started(goblins, settings, turn, phase) => {
                    // Phase Actions Section
                    match phase {
                        // Event Phase
                        AdventurePhase::Event(event_phase) => {
                            let data = EventLocationData::get(event_phase.location);
                            let num_locations = ALL_EVENT_LOCATION_DATA.len();
                            let num_scenarios = data.scenarios.len();
                            // Debug
                            if gamepad(0).left.just_pressed() {
                                event_phase.location = (event_phase.location - 1).min(num_locations - 1) % num_locations;
                                event_phase.scenario %= EventLocationData::get(event_phase.location).scenarios.len();
                            } else if gamepad(0).right.just_pressed() {
                                event_phase.location = (event_phase.location + 1) % num_locations;
                                event_phase.scenario %= EventLocationData::get(event_phase.location).scenarios.len();
                            } else if gamepad(0).up.just_pressed() {
                                event_phase.scenario = (event_phase.scenario - 1).min(num_scenarios - 1) % num_scenarios;
                            } else if gamepad(0).down.just_pressed() {
                                event_phase.scenario = (event_phase.scenario + 1) % num_scenarios;
                            }

                            let image = data.images[0];
                            sprite!(image);
                            if let Some(event) = state.gui.phase_actions_section.draw_event_actions(&event_phase) {
                                if !state.gui.is_overlay_open() {
                                    let event = PhaseActionSectionEvent::Event(event);
                                    // turbo::println!("event {:?}", event);
                                    state.gui.dispatch(Command::PhaseActionSection(event));
                                }
                            }
                        }
                        // Camp Phase
                        AdventurePhase::Camp(camp_phase) => {
                            let data = &CAMP_LOCATION_DATA;
                            let image = data.images[0];
                            sprite!(image);
                            if let Some(event) = state.gui.phase_actions_section.draw_camp_actions(&camp_phase) {
                                if !state.gui.is_overlay_open() {
                                    let event = PhaseActionSectionEvent::Camp(event);
                                    // turbo::println!("event {:?}", event);
                                    state.gui.dispatch(Command::PhaseActionSection(event));
                                }
                            }
                        }
                    }

                    // Goblin List
                    if let Some(event) = state.gui.goblin_list.draw(&goblins, &settings.goblin_order, &turn) {
                        if !state.gui.is_overlay_open() {
                            // turbo::println!("event {:?}", event);
                            match event {
                                GoblinListEvent::OpenGoblinDialog(player) if player != turn.player => {
                                    // don't open goblin dialogs for non-active goblins
                                }
                                _ => state.gui.dispatch(Command::GoblinList(event)),
                            }
                        }
                    }

                    // Goblin Loot Inspector
                    if let Some(ref mut inspector) = state.gui.loot_inspector {
                        if let Some(event) = inspector.draw(&goblins) {
                            turbo::println!("event {:?}", event);
                            state.gui.dispatch(Command::GoblinLootInspector(event));
                        }
                    }

                    // Goblin Dialog
                    if let Some(ref mut dialog) = state.gui.goblin_dialog {
                        if let Some(event) = dialog.draw() {
                            // turbo::println!("event {:?}", event);
                            state.gui.dispatch(Command::GoblinDialog(event));
                        }
                    }

                    // Actions can be a side-effect of GUI commands
                    enum Action {
                        CampRummageForLoot,
                        CampRummageTakeLoot,
                        CampRummageLeaveLoot,
                        EventStart,
                        EventTakeRisk,
                        EventPlayItSafe,
                        EventHandleOutcome,
                        KeepGoing,
                        TakeABreak,
                    }
                    let mut action = None;

                    // Consume GUI Commands
                    let mut cmd = state.gui.commands.pop_front();
                    while cmd != None {
                        turbo::println!("{:?}", cmd);
                        match cmd.take() {
                            Some(Command::PhaseActionSection(e)) => match e {
                                PhaseActionSectionEvent::Event(e) => match e {
                                    EventPhaseAction::PlayItSafe => {
                                        let event = PhaseActionSectionEvent::Event(EventPhaseAction::PlayItSafeEnd);
                                        let cmd = Command::PhaseActionSection(event);
                                        let msg = match phase {
                                            AdventurePhase::Event(event_phase) => {
                                                let data = EventLocationData::get(event_phase.location);
                                                let actions = &data.scenarios[event_phase.scenario].actions;
                                                actions[1].dialog[0]
                                            }
                                            _ => "Better not risk it..."
                                        };
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    EventPhaseAction::PlayItSafeEnd => {
                                        action = Some(Action::EventPlayItSafe);
                                    }
                                    EventPhaseAction::TakeRisk => {
                                        let event = PhaseActionSectionEvent::Event(EventPhaseAction::TakeRiskEnd);
                                        let cmd = Command::PhaseActionSection(event);
                                        let msg = match phase {
                                            AdventurePhase::Event(event_phase) => {
                                                let data = EventLocationData::get(event_phase.location);
                                                let actions = &data.scenarios[event_phase.scenario].actions;
                                                actions[0].dialog[0]
                                            }
                                            _ => "Fuck it, we ball!"
                                        };
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    EventPhaseAction::TakeRiskEnd => {
                                        action = Some(Action::EventTakeRisk);
                                    }
                                    EventPhaseAction::ConfirmOutcome(should_handle_outcome) => {
                                        if should_handle_outcome {
                                            action = Some(Action::EventHandleOutcome);
                                        } else {
                                            let event = PhaseActionSectionEvent::Event(EventPhaseAction::ConfirmOutcome(true));
                                            let cmd = Command::PhaseActionSection(event);
                                            let msg = match phase {
                                                AdventurePhase::Event(event_phase) => {
                                                    if let Some(EventPhaseOutcome { choice, effect, accepted: _ }) = &event_phase.outcome {
                                                        let data = EventLocationData::get(event_phase.location);
                                                        let action = &data.scenarios[event_phase.scenario].actions[*choice];
                                                        action.outcomes[*effect].dialog[0]
                                                    } else {
                                                        continue
                                                    }
                                                }
                                                _ => continue,
                                            };
                                            state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                        }
                                    }
                                    EventPhaseAction::KeepGoingStart => {
                                        let event = PhaseActionSectionEvent::Event(EventPhaseAction::KeepGoingEnd);
                                        let cmd = Command::PhaseActionSection(event);
                                        let msg = KEEP_GOING_DIALOG[rand() as usize % KEEP_GOING_DIALOG.len()];
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    EventPhaseAction::KeepGoingEnd => {
                                        action = Some(Action::KeepGoing);
                                    }
                                    EventPhaseAction::TakeABreakStart => {
                                        let event = PhaseActionSectionEvent::Event(EventPhaseAction::TakeABreakEnd);
                                        let cmd = Command::PhaseActionSection(event);
                                        let msg = TAKE_A_BREAK_DIALOG[rand() as usize % TAKE_A_BREAK_DIALOG.len()];
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    EventPhaseAction::TakeABreakEnd => {
                                        action = Some(Action::TakeABreak);
                                    }
                                }
                                PhaseActionSectionEvent::Camp(e) => match e {
                                    CampPhaseAction::RummageStart => {
                                        let event = PhaseActionSectionEvent::Camp(CampPhaseAction::RummageEnd);
                                        let cmd = Command::PhaseActionSection(event);
                                        let msg = LOOT_RUMMAGE_DIALOG[rand() as usize % LOOT_RUMMAGE_DIALOG.len()];
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    CampPhaseAction::RummageEnd => {
                                        action = Some(Action::CampRummageForLoot);
                                        state.gui.phase_actions_section.camp = CampActionMenu::RummageResult;
                                    }
                                    CampPhaseAction::RummageConfirmFailure => {
                                        let msg = match phase {
                                            AdventurePhase::Camp(camp_phase) => match &camp_phase.rummage_result {
                                                Some(RummageResult::Fail) => LOOT_RUMMAGE_FAIL_ACCEPT_DIALOG[rand() as usize % LOOT_RUMMAGE_FAIL_ACCEPT_DIALOG.len()],
                                                _ => UNREACHABLE_DIALOG
                                            }
                                            _ => UNREACHABLE_DIALOG
                                        };
                                        let event = PhaseActionSectionEvent::Camp(CampPhaseAction::BackToDefaultMenu);
                                        let cmd = Command::PhaseActionSection(event);
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    CampPhaseAction::RummageConfirmSuccess(did_take_loot) => {
                                        let msg = match phase {
                                            AdventurePhase::Camp(camp_phase) => match &camp_phase.rummage_result {
                                                Some(RummageResult::Success { .. })=> {
                                                    if did_take_loot {
                                                        action = Some(Action::CampRummageTakeLoot);
                                                        &LOOT_RUMMAGE_TAKE_LOOT_DIALOG[rand() as usize % LOOT_RUMMAGE_TAKE_LOOT_DIALOG.len()]
                                                    } else {
                                                        action = Some(Action::CampRummageLeaveLoot);
                                                        &LOOT_RUMMAGE_LEAVE_LOOT_DIALOG[rand() as usize % LOOT_RUMMAGE_LEAVE_LOOT_DIALOG.len()]
                                                    }
                                                },
                                                _ => UNREACHABLE_DIALOG
                                            }
                                            _ => UNREACHABLE_DIALOG
                                        };
                                        let event = PhaseActionSectionEvent::Camp(CampPhaseAction::BackToDefaultMenu);
                                        let cmd = Command::PhaseActionSection(event);
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    CampPhaseAction::Bribe => {
                                        // TODO
                                        let msg = UNIMPLEMENTED_DIALOG[rand() as usize % UNIMPLEMENTED_DIALOG.len()];
                                        state.gui.open_goblin_dialog(turn.player, msg, None);
                                    }
                                    CampPhaseAction::ContinueStart => {
                                        let event = PhaseActionSectionEvent::Camp(CampPhaseAction::ContinueEnd);
                                        let cmd = Command::PhaseActionSection(event);
                                        let msg = KEEP_GOING_DIALOG[rand() as usize % KEEP_GOING_DIALOG.len()];
                                        state.gui.open_goblin_dialog(turn.player, msg, Some(cmd));
                                    }
                                    CampPhaseAction::ContinueEnd => {
                                        action = Some(Action::EventStart);
                                    }
                                    CampPhaseAction::BackToDefaultMenu => {
                                        state.gui.phase_actions_section.camp = CampActionMenu::Default;
                                    }
                                }
                            }
                            Some(Command::GoblinList(e)) => match e {
                                GoblinListEvent::OpenGoblinDialog(player) => {
                                    let msg = match phase {
                                        AdventurePhase::Event(event_phase) => {
                                            let data = EventLocationData::get(event_phase.location);
                                            data.dialog[0]
                                        }
                                        AdventurePhase::Camp(_camp_phase) => {
                                            CAMP_LOCATION_DATA.dialog[rand() as usize % CAMP_LOCATION_DATA.dialog.len()]
                                        }
                                    };
                                    state.gui.open_goblin_dialog(player, msg, None);
                                }
                                GoblinListEvent::OpenGoblinLootInspector(player) => {
                                    state.gui.open_goblin_loot_inspector(player);
                                }
                            }
                            Some(Command::GoblinDialog(e)) => match e {
                                GoblinDialogEvent::Close => {
                                    cmd = state.gui.close_goblin_dialog();
                                }
                                GoblinDialogEvent::FastForward => {
                                    state.gui.fast_forward_goblin_dialog();
                                }
                                GoblinDialogEvent::Next => {
                                    // TODO
                                }
                            }
                            Some(Command::GoblinLootInspector(e)) => match e {
                                GoblinLootInspectorEvent::Close => {
                                    state.gui.close_goblin_loot_inspector();
                                }
                                GoblinLootInspectorEvent::SelectLoot(_i) => {
                                    // TODO
                                }
                            }
                            _ => {
                                //
                            }
                        }
                    }
                    state.gui.commands.clear();
                    match action {
                        Some(Action::CampRummageForLoot) => {
                            if adventure.rummage_for_loot().is_err() {
                                turbo::println!("Couldn't rummage");
                            }
                        }
                        Some(Action::CampRummageTakeLoot) => {
                            if adventure.rummage_take_loot().is_err() {
                                turbo::println!("Couldn't take loot");
                            }
                        }
                        Some(Action::CampRummageLeaveLoot) => {
                            if adventure.rummage_leave_loot().is_err() {
                                turbo::println!("Couldn't leave loot");
                            }
                        }
                        Some(Action::EventStart) => {
                            if adventure.event_start().is_err() {
                                turbo::println!("Couldn't start event");
                            }
                        }
                        Some(Action::EventTakeRisk) => {
                            if adventure.event_make_choice(0).is_err() {
                                turbo::println!("Couldn't take risk");
                            }
                        }
                        Some(Action::EventPlayItSafe) => {
                            if adventure.event_make_choice(1).is_err() {
                                turbo::println!("Couldn't play it safe");
                            }
                        }
                        Some(Action::EventHandleOutcome) => {
                            if adventure.event_handle_outcome().is_err() {
                                turbo::println!("Couldn't handle outcome");
                            }
                        }
                        Some(Action::KeepGoing) => {
                            if adventure.keep_going().is_err() {
                                turbo::println!("Couldn't keep going");
                            }
                        }
                        Some(Action::TakeABreak) => {
                            if adventure.take_a_break().is_err() {
                                turbo::println!("Couldn't take a break");
                            }
                        }
                        None => {
                            // noop
                        }
                    }
                }
                AdventureState::Complete(_goblins, _settings) => {
                    //
                }
            }


            // Debug
            set_camera(0, 128);
            if gamepad(0).start.just_pressed() {
                turbo::println!("{:#?}", adventure);
            }
        }
        if go_to_title {
            state.adventure = None;
        }
        draw_cursor();
        state.save();
        return;
    }

    state.save();
}
