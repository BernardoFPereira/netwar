#![allow(dead_code)]

mod commands;
mod grid_builder;
mod netagent;

mod prelude {
    pub use crate::commands::*;
    pub use crate::grid_builder::*;
    pub use crate::netagent::*;
    pub use macroquad::{
        prelude::*,
        ui::{hash, root_ui, widgets},
    };
}

use crate::prelude::*;

struct Player {
    units: Units,
    bits: i32,
    gen_time: f32,
    input: String,
}
impl Player {
    fn new() -> Self {
        Self {
            units: Units::new(),
            bits: 100,
            gen_time: 1.,
            input: String::new(),
        }
    }
}

pub struct MatchState {
    player: Player,
    grid: Grid,
    cmd_history: Vec<String>,
}
impl MatchState {
    fn new() -> Self {
        let mut grid = Grid::new(12., 6., 45.);
        grid.fill_grid();
        grid.create_mines(4);
        grid.create_strongholds();

        let player = Player::new();

        Self {
            player,
            grid,
            cmd_history: vec![],
        }
    }
}

#[macroquad::main("NetWars")]
async fn main() {
    let mut ms = MatchState::new();

    // let mut input = String::new();
    // let mut cmd_history: Vec<String> = vec![];

    loop {
        // Loop Start-up
        clear_background(BLACK);
        draw_windows(&mut ms);
        unit_generation(&mut ms);

        // Input Handling
        if is_key_pressed(KeyCode::Enter) {
            let input = ms.player.input.clone();
            if input != "" {
                ms.cmd_history.push(input.clone());

                let (cmd, args) = parse_commands(input.clone());
                let output = issue_command(&mut ms, cmd, args);

                ms.cmd_history.push(output);
            }
            ms.player.input = "".to_owned();
        }

        // Draw stuff
        ms.grid.draw_grid();
        ms.player.units.draw_units();

        next_frame().await
    }
}

fn draw_windows(ms: &mut MatchState) {
    // Create UI windows
    widgets::Window::new(
        hash!(),
        vec2(
            screen_width() - screen_width() / 2.,
            screen_height() - screen_height() / 3.,
        ),
        vec2(300., 100.),
    )
    .label("STATS")
    .movable(true)
    .ui(&mut *root_ui(), |ui| {
        ui.label(None, &format!("Bits: {}", ms.player.bits));
        ui.label(None, &format!("Units: {}", ms.player.units.agents.len()));
        ui.separator();
        ui.label(None, "Mines controlled:");
    });

    widgets::Window::new(
        hash!(),
        vec2(10., screen_height() - screen_height() / 3.),
        vec2(300., 200.),
    )
    .label("TERMINAL")
    .movable(true)
    .ui(&mut *root_ui(), |ui| {
        ui.group(hash!(), vec2(286., 150.), |ui| {
            for cmd in &mut ms.cmd_history {
                ui.label(None, &format!("{cmd}"));
            }
            if is_key_down(KeyCode::Enter) {
                ui.scroll_here();
            }
        });
        ui.input_text(hash!(), "", &mut ms.player.input);
    });
}

// fn unit_generation(player: &mut Player, unit_gen_timer: &mut f32) {
//     *unit_gen_timer -= get_frame_time();

//     let new_gen_time = if player.bits as f32 >= 200. {
//         player.bits as f32 / 100.
//     } else {
//         2.
//     };

//     // TODO: Maybe move unit_gen_timer to actual Stronghold Tiles
//     if *unit_gen_timer <= 0. {
//         player.units.agents.push(Agent::new(vec2(5., 5.)));
//         *unit_gen_timer = new_gen_time;
//     }
// }
