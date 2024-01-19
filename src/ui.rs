use std::collections::HashMap;

use super::*;

////////////////////////////////////////////////////////////////////////////////
// Graphical User Interface
////////////////////////////////////////////////////////////////////////////////

impl GUI {
    pub fn dispatch(&mut self, cmd: Command) {
        self.commands.push_front(cmd);
    }
    pub fn is_overlay_open(&self) -> bool {
        self.goblin_dialog != None || self.loot_inspector != None
    }
    pub fn open_goblin_dialog(&mut self, player: Player, message: &str, on_close: Option<Command>) {
        let message = insert_line_breaks(message, 36);
        self.goblin_dialog = Some(GoblinDialog {
            player,
            message,
            max_len: 0,
            on_close,
        });
    }
    pub fn fast_forward_goblin_dialog(&mut self) {
        if let Some(goblin_dialog) = &mut self.goblin_dialog {
            goblin_dialog.max_len = goblin_dialog.message.len();
        }
    }
    pub fn close_goblin_dialog(&mut self) -> Option<Command> {
        if let Some(goblin_dialog) = self.goblin_dialog.take() {
            return goblin_dialog.on_close;
        }
        return None;
    }
    pub fn open_goblin_loot_inspector(&mut self, player: Player) {
        self.loot_inspector = Some(GoblinLootInspector {
            player,
            selected: None,
        });
    }
    pub fn close_goblin_loot_inspector(&mut self) {
        self.loot_inspector = None;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Phase Actions Section
////////////////////////////////////////////////////////////////////////////////

impl PhaseActionsSection {
    pub const MAX_LINE_LEN: usize = 23;
    pub const MAX_FONT_L_LINE_LEN: usize = 14;
    pub const DESC_CAMP_DEFAULT: &'static str = "The flickering campfire casts a warm glow, offering a brief respite from the adventurers' relentless journey...";
    pub const DESC_CAMP_RUMMAGE_FAIL: &'static str = "Your attempt to rummage the party's loot was noticed by the others. You play it off with a clumsy chuckle and whistle a tune as they make a hasty retreat.";
    pub const DESC_CAMP_RUMMAGE_SUCCESS: &'static str = "With deft fingers and a sly grin, you rummage through the loot sack, uncovering hidden treasures. Your eyes sparkle with glee as you decide whether to pocket your newfound riches, unnoticed by all.";
    pub fn draw_event_actions(&mut self, event_phase: &EventPhase) -> Option<EventPhaseAction> {
        let mut event = None;

        set_camera(0, 0);
        let mut x = 128;
        let mut y = 0;

        // Background
        let [_w, h] = resolution();
        rect!(w = 128, h = h, x = x, y = y, fill = 0x000303ff);
        x += 4;
        y += 8;

        let data = event_phase.location.data();

        // Title
        let msg = &insert_line_breaks(data.name, Self::MAX_FONT_L_LINE_LEN);
        text!(msg, font = Font::L, x = x, y = y);
        y += 8 * msg.lines().count() as i32;
        y += 8;

        // Description
        let msg = data.description;
        let msg = insert_line_breaks(msg, PhaseActionsSection::MAX_LINE_LEN);
        text!(&msg, x = x, y = y, color = WHITE);
        y += 8 * msg.lines().count() as i32;
        y += 8;

        // Scenario
        let scenario = &data.scenarios[event_phase.scenario];
        let msg = scenario.description;
        let msg = insert_line_breaks(msg, PhaseActionsSection::MAX_LINE_LEN);
        text!(&msg, x = x, y = y, color = WHITE);
        y += 8 * msg.lines().count() as i32;
        y += 8;

        // Actions - Action taken
        if let Some((action_index, result_index, accepted)) = &event_phase.result {
            let action =
                &event_phase.location.data().scenarios[event_phase.scenario].actions[*action_index];
            let outcome = &action.outcomes[*result_index];
            let is_good_outcome = outcome.effect.is_good();

            let msg = &outcome.effect.desc().to_ascii_uppercase();
            let msg = insert_line_breaks(msg, PhaseActionsSection::MAX_LINE_LEN);
            #[rustfmt::skip]
            text!(&msg, x = x, y = y, color = if is_good_outcome { GREEN } else { RED });
            y += 8 * msg.lines().count() as i32;
            y += 8;

            let msg = outcome.description;
            let msg = insert_line_breaks(msg, PhaseActionsSection::MAX_LINE_LEN);
            text!(&msg, x = x, y = y, color = WHITE);
            y += 8 * msg.lines().count() as i32;
            y += 8;

            if *accepted {
                let msg = "WHAT WILL YOU DO NEXT?";
                text!(&msg, x = x, y = y, color = WHITE);
                y += 8 * msg.lines().count() as i32;
                y += 8;
            }

            #[rustfmt::skip]
            let mut actions = vec![];
            if *accepted {
                if is_good_outcome {
                    actions.push((EventPhaseAction::KeepGoingStart, "> Keep Going"));
                }
                actions.push((EventPhaseAction::TakeABreakStart, "> Take a Break"));
            } else {
                actions.push((EventPhaseAction::ConfirmOutcome(false), "> Next..."));
            }
            for (action, msg) in actions {
                if cbutton(Font::S, x, y, Some(128 - 16), BLACK, WHITE, WHITE, msg) {
                    event = Some(action);
                }
                y += 16;
            }

            return event;
        }

        // Actions - No action taken
        let [risky, safe] = &scenario.actions;
        #[rustfmt::skip]
        let actions = [
            (EventPhaseAction::TakeRisk, risky),
            (EventPhaseAction::PlayItSafe, safe),
        ];
        for (action, a) in actions {
            if cbutton(Font::S, x, y, Some(128 - 16), BLACK, WHITE, WHITE, a.label) {
                event = Some(action);
            }
            y += 16;
        }

        return event;
    }
    pub fn draw_camp_actions(&mut self, phase: &CampPhase) -> Option<CampPhaseAction> {
        let mut event = None;

        set_camera(0, 0);
        let mut x = 128;
        let mut y = 0;

        // Background
        let [_w, h] = resolution();
        rect!(w = 128, h = h, x = x, y = y, fill = 0x000303ff);
        x += 4;
        y += 8;

        // Title
        let msg = &insert_line_breaks("ADVENTURERS' CAMP", Self::MAX_FONT_L_LINE_LEN);
        text!(msg, font = Font::L, x = x, y = y);
        y += 8 * msg.lines().count() as i32;
        y += 8;

        // Description
        let msg = PhaseActionsSection::DESC_CAMP_DEFAULT;
        let msg = insert_line_breaks(&msg, PhaseActionsSection::MAX_LINE_LEN);
        text!(&msg, x = x, y = y, color = WHITE);
        y += 8 * msg.lines().count() as i32;
        y += 8;

        match self.camp {
            CampActionMenu::Default => {
                // Description
                text!("WHAT WILL YOU DO NEXT?", x = x, y = y, color = WHITE);
                y += 8;
                y += 8;
                // Actions
                let mut actions = vec![];
                if phase.rummage_result == None {
                    actions.push((CampPhaseAction::RummageStart, "> Rummage party loot"));
                }
                if phase.bribe_result == None {
                    actions.push((CampPhaseAction::Bribe, "> Bribe a hero"));
                }
                actions.push((CampPhaseAction::ContinueStart, "> Continue journey"));
                for (action, msg) in actions {
                    if cbutton(Font::S, x, y, Some(128 - 16), BLACK, WHITE, WHITE, msg) {
                        event = Some(action);
                    }
                    y += 16;
                }
            }
            CampActionMenu::RummageResult => match &phase.rummage_result {
                Some(RummageResult::Fail) => {
                    // Description
                    text!("YOU GOT CAUGHT", x = x, y = y, color = RED);
                    y += 8;
                    y += 8;
                    let msg = Self::DESC_CAMP_RUMMAGE_FAIL;
                    let msg = insert_line_breaks(&msg, Self::MAX_LINE_LEN);
                    text!(&msg, x = x, y = y, color = WHITE);
                    y += 8 * msg.lines().count() as i32;
                    y += 8;
                    // Actions
                    let mut actions = vec![];
                    actions.push((CampPhaseAction::RummageConfirmFailure, "> Make an excuse"));
                    for (action, msg) in actions {
                        if cbutton(Font::S, x, y, Some(128 - 16), BLACK, WHITE, WHITE, msg) {
                            event = Some(action);
                        }
                        y += 16;
                    }
                }
                Some(RummageResult::Success { .. }) => {
                    // Description
                    text!("YOU FOUND SOME LOOT", x = x, y = y, color = GREEN);
                    y += 8;
                    y += 8;
                    let msg =
                        insert_line_breaks(Self::DESC_CAMP_RUMMAGE_SUCCESS, Self::MAX_LINE_LEN);
                    text!(&msg, x = x, y = y, color = WHITE);
                    y += 8 * msg.lines().count() as i32;
                    y += 8;
                    // Actions
                    #[rustfmt::skip]
                    let actions = [
                        (CampPhaseAction::RummageConfirmSuccess(true), "> Take loot"),
                        (CampPhaseAction::RummageConfirmSuccess(false), "> Leave it be"),
                    ];
                    for (action, msg) in actions {
                        if cbutton(Font::S, x, y, Some(128 - 16), BLACK, WHITE, WHITE, msg) {
                            event = Some(action);
                        }
                        y += 16;
                    }
                }
                _ => {
                    //
                }
            },
            CampActionMenu::BribeResult => {
                //
            }
        }

        return event;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Goblin List
////////////////////////////////////////////////////////////////////////////////

impl GoblinList {
    pub fn draw(
        &mut self,
        goblins: &GoblinMap,
        goblin_order: &GoblinOrder,
        turn: &Turn,
    ) -> Option<GoblinListEvent> {
        // Event
        let mut event = None;

        set_camera(0, 0);
        let [w, h] = resolution();
        let mut x = 0;
        let mut y = 128;
        x += 7;
        for i in 0..4 {
            let player = goblin_order.get(&i);
            if player.is_none() {
                break;
            }
            let player = player.unwrap();
            #[rustfmt::skip]
            let goblin_key = &format!("goblin_{}", match player {
                Player::P1 => 1,
                Player::P2 => 2,
                Player::P3 => 3,
                Player::P4 => 4,
            });
            sprite!(goblin_key, x = x, y = y);
            if cdiv(32, 32, x, y, TRANSPARENT, TRANSPARENT) {
                event = Some(GoblinListEvent::OpenGoblinDialog(*player));
            }
            let left = x;
            let top = y;
            x += 36;
            y += 6;
            let goblin = &goblins[&player];
            let attributes = [
                ("player", &format!("{:?}", player)),
                ("health", &goblin.health.to_string()),
                ("luck  ", &goblin.luck.to_string()),
                ("greed ", &goblin.greed.to_string()),
            ];
            for (key, val) in attributes {
                let key = key.to_ascii_uppercase();
                text!(&format!("{key}: {:0>2}", val), font = Font::S, x = x, y = y);
                y += 6;
            }
            let prev_y = y;
            x += 54;
            y = top + 5;
            rect!(w = 1, h = 24, x = x, y = y, fill = 0xffffff33);
            x += 5;
            y += 1;
            text!("$000", x = x, y = y, font = Font::M, color = WHITE);
            y += 10;
            if cbutton(Font::S, x - 1, y, None, BLACK, WHITE, BLACK, "BAG") {
                event = Some(GoblinListEvent::OpenGoblinLootInspector(*player));
            }
            x = left;
            y = prev_y + 2;
            if turn.player == *player {
                cdiv(120, 31, left, top + 1, TRANSPARENT, WHITE);
                rect!(w = 5, h = 31, x = x - 6, y = top + 1, fill = WHITE);
                text!(
                    "A\nC\nT\nI\nV\nE",
                    font = Font::S,
                    x = x - 5,
                    y = top + 2,
                    color = BLACK
                );
            } else {
                rect!(w = 120, h = 32, x = left, y = top, fill = BACKDROP);
            }
        }
        return event;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Goblin Dialog
////////////////////////////////////////////////////////////////////////////////

impl GoblinDialog {
    pub fn draw(&mut self) -> Option<GoblinDialogEvent> {
        let mut event = None;
        let is_entire_message = self.max_len >= self.message.len();

        set_camera(0, 0);
        let [w, h] = resolution();

        // Drop-shadow
        if cdiv(w, h - 32, 0, 0, BACKDROP, BACKDROP) && self.max_len > 0 {
            if is_entire_message {
                let _ = event.insert(GoblinDialogEvent::Close);
            }
        }
        let x = 0;
        let y = h as i32 - 32;

        // Panel
        if cdiv(w, 32, x, y, BLACK, WHITE) {
            if is_entire_message {
                let _ = event.insert(GoblinDialogEvent::Close);
            } else {
                let _ = event.insert(GoblinDialogEvent::FastForward);
                // TODO: skip to next page of dialog
                // let _ = event.insert(GoblinDialogEvent::Next);
            }
        }

        // Goblin portrait
        #[rustfmt::skip]
        let goblin_key = &format!("goblin_portrait_{}", match self.player {
            Player::P1 => 1,
            Player::P2 => 2,
            Player::P3 => 3,
            Player::P4 => 4,
        });
        sprite!(goblin_key, x = x, y = y - 32);
        let fg = WHITE;
        let bg = BLACK;
        circ!(d = 26, x = -10, y = y + 32 - 14, fill = bg);
        circ!(d = 26, x = -12, y = y + 32 - 12, fill = fg);
        text!(
            &format!("{:?}", self.player),
            x = 1,
            y = y + 25,
            font = Font::S,
            color = bg
        );

        // Message
        let x = x + 66;
        let y = y + 5;
        let msg = &self.message[0..self.max_len.min(self.message.len())];
        text!(msg, x = x, y = y, color = WHITE);

        // Indicator
        if is_entire_message && (self.max_len / 16) % 2 == 0 {
            circ!(d = 2, x = x + 3 + (36 * 5), y = y + 20, fill = WHITE);
        }

        // Increment max_len
        self.max_len += 1;

        // Return event
        return event;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Goblin Loot Inspector
////////////////////////////////////////////////////////////////////////////////

impl GoblinLootInspector {
    pub fn draw(&mut self, goblins: &HashMap<Player, Goblin>) -> Option<GoblinLootInspectorEvent> {
        let mut event = None;

        set_camera(0, 0);
        let [w, h] = resolution();
        // rect!(w = w, h = h, fill = 0x000000dd);
        if cdiv(w, h - 48, 0, 0, BACKDROP, BACKDROP) {
            let _ = event.insert(GoblinLootInspectorEvent::Close);
        }
        let mut x = 0;
        let mut y = h as i32 - 48;
        let top = y;
        cdiv(w, 48, x, y, BLACK, WHITE);
        y = h as i32;

        #[rustfmt::skip]
        let goblin_key = &format!("goblin_{}", match self.player {
            Player::P1 => 1,
            Player::P2 => 2,
            Player::P3 => 3,
            Player::P4 => 4,
        });
        // sprite!(goblin_key, x = x, y = y);
        y -= 56;
        sprite!("sack", x = x, y = y);
        y = h as i32;

        let fg = WHITE;
        let bg = BLACK;
        x -= 10;
        y -= 14;
        circ!(d = 26, x = x, y = y, fill = bg);
        x -= 2;
        y += 2;
        circ!(d = 26, x = x, y = y, fill = fg);
        x += 3;
        text!(
            &format!("{:?}", self.player),
            x = 1,
            y = h as i32 - 7,
            font = Font::S,
            color = bg
        );

        y = top + 5;
        x = 66;
        text!("LOOT BAG", x = x, y = y, color = WHITE);
        y += 10;
        for i in 0..26 {
            let cols = 13;
            let w = 14;
            let h = 14;
            let x = x + (i % cols) * w as i32;
            let y = y + (i / cols) * h as i32;
            // rect!(w = w - 1, h = h - 1, x = x, y = y, fill = 0xffffff66);
            if cdiv(w - 1, h - 1, x, y, 0xffffff33, TRANSPARENT) {
                let _ = event.insert(GoblinLootInspectorEvent::SelectLoot(i as usize));
            }
        }

        return event;
    }
}

////////////////////////////////////////////////////////////////////////////////
// UI Primitives
////////////////////////////////////////////////////////////////////////////////

pub fn cbutton(
    font: Font,
    x: i32,
    y: i32,
    w: Option<u32>,
    color: u32,
    fill: u32,
    border: u32,
    msg: &str,
) -> bool {
    let (fw, fh, px, py) = match font {
        Font::S => (5, 5, 8, 8),
        Font::M => (5, 8, 12, 8),
        Font::L => (8, 8, 16, 16),
    };
    let tw = fw * msg.chars().count() as u32;
    let w = w.unwrap_or(tw);
    let h = fh;
    let w = w + px;
    let h = h + py;
    let m = mouse(0);
    let [mx, my] = m.position;
    let did_intersect = mx >= x && mx < (x + w as i32) && my >= y && my < (y + h as i32);
    let is_just_released = did_intersect && m.left.just_released();
    let is_pressed = did_intersect && m.left.pressed();
    #[rustfmt::skip]
    let y = if is_just_released || is_pressed { y + 1 } else { y };
    #[rustfmt::skip]
    rectv!(w = w, h = h, x = x, y = y, fill = fill, border = Border {
        size: 1,
        radius: 4,
        color: border.swap_bytes(),
    });
    #[rustfmt::skip]
    text!(msg, x = x + (px / 2) as i32, y = y + 1 + (py / 2) as i32, font = font, color = color);
    is_just_released
}

pub fn button(font: Font, x: i32, y: i32, msg: &str) -> bool {
    let (fw, fh, px, py) = match font {
        Font::S => (5, 5, 8, 8),
        Font::M => (5, 8, 12, 8),
        Font::L => (8, 8, 16, 16),
    };
    let w = fw * msg.chars().count() as u32;
    let h = fh;
    let w = w + px;
    let h = h + py;
    let m = mouse(0);
    let [mx, my] = m.position;
    let did_intersect = mx >= x && mx < (x + w as i32) && my >= y && my < (y + h as i32);
    let is_just_released = did_intersect && m.left.just_released();
    let is_pressed = did_intersect && m.left.pressed();
    #[rustfmt::skip]
    let y = if is_just_released || is_pressed { y + 1 } else { y };
    #[rustfmt::skip]
    rectv!(w = w, h = h, x = x, y = y, fill = BG, border = Border {
        size: 1,
        radius: 4,
        color: FG.swap_bytes(),
    });
    #[rustfmt::skip]
    text!(msg, x = x + (px / 2) as i32, y = y + 1 + (py / 2) as i32, font = font, color = FG);
    is_just_released
}

pub fn ibutton(font: Font, x: i32, y: i32, msg: &str) -> bool {
    let (fw, fh, px, py) = match font {
        Font::S => (5, 5, 8, 8),
        Font::M => (5, 8, 12, 8),
        Font::L => (8, 8, 16, 16),
    };
    let w = fw * msg.chars().count() as u32;
    let h = fh;
    let w = w + px;
    let h = h + py;
    let m = mouse(0);
    let [mx, my] = m.position;
    let did_intersect = mx >= x && mx < (x + w as i32) && my >= y && my < (y + h as i32);
    let is_just_released = did_intersect && m.left.just_released();
    let is_pressed = did_intersect && m.left.pressed();
    #[rustfmt::skip]
    let y = if is_just_released || is_pressed { y + 1 } else { y };
    #[rustfmt::skip]
    rectv!(w = w, h = h, x= x, y = y, fill = FG, border = Border {
        size: 1,
        radius: 4,
        color: FG.swap_bytes(),
    });
    #[rustfmt::skip]
    text!(msg, x = x + (px / 2) as i32, y = y + 1 + (py / 2) as i32, font = font, color = BG);
    is_just_released
}

pub fn cdiv(w: u32, h: u32, x: i32, y: i32, fill: u32, border: u32) -> bool {
    #[rustfmt::skip]
    rectv!(w = w, h = h, x = x, y = y, fill = fill, border = Border {
        size: 1,
        radius: 4,
        color: border.swap_bytes(),
    });
    let m = mouse(0);
    let [mx, my] = m.position;
    let did_intersect = mx >= x && mx < (x + w as i32) && my >= y && my < (y + h as i32);
    did_intersect && m.left.just_released()
}

pub fn div(w: u32, h: u32, x: i32, y: i32) -> bool {
    #[rustfmt::skip]
    rectv!(w = w, h = h, x = x, y = y, fill = BG, border = Border {
        size: 1,
        radius: 4,
        color: FG.swap_bytes(),
    });
    let m = mouse(0);
    let [mx, my] = m.position;
    let did_intersect = mx >= x && mx < (x + w as i32) && my >= y && my < (y + h as i32);
    did_intersect && m.left.just_pressed()
}

pub fn idiv(w: u32, h: u32, x: i32, y: i32) -> bool {
    #[rustfmt::skip]
    rectv!(w = w, h = h, x = x, y = y, fill = FG, border = Border {
        size: 1,
        radius: 4,
        color: BG.swap_bytes(),
    });
    let m = mouse(0);
    let [mx, my] = m.position;
    let did_intersect = mx >= x && mx < (x + w as i32) && my >= y && my < (y + h as i32);
    did_intersect && m.left.just_pressed()
}
