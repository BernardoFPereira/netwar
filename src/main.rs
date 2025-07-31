#![allow(dead_code)]

mod grid_builder;
mod netagent;

mod prelude {
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
}
impl Player {
    fn new() -> Self {
        Self {
            units: Units::new(),
            bits: 100,
            gen_time: 1.,
        }
    }
}

#[macroquad::main("NetWar")]
async fn main() {
    let mut grid = Grid::new(12., 6., 45.);
    grid.fill_grid();
    grid.create_mines(4);
    grid.create_strongholds();

    let mut player = Player::new();
    let mut unit_gen_timer = 1.;

    let mut command = String::new();
    let mut cmd_history: Vec<String> = vec![];

    loop {
        clear_background(BLACK);

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
            ui.label(None, &format!("Bits: {}", player.bits));
            ui.label(None, &format!("Units: {}", player.units.agents.len()));
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
                for cmd in &mut cmd_history {
                    ui.label(None, &format!("{cmd}"));
                }
                if is_key_down(KeyCode::Enter) {
                    ui.scroll_here();
                }
            });
            ui.input_text(hash!(), "", &mut command);
        });

        if is_key_pressed(KeyCode::Enter) {
            if command != "" {
                cmd_history.push(command.clone());
                let parse_result = parse_commands(command.clone());
                match parse_result {
                    Ok(mut output) => {
                        cmd_history.append(&mut output);
                    }
                    Err(e) => cmd_history.push(e),
                }
            }
            command = "".to_owned();
        }

        grid.draw_grid();
        player.units.draw_units();
        unit_generation(&mut player, &mut unit_gen_timer);

        next_frame().await
    }
}

fn unit_generation(player: &mut Player, unit_gen_timer: &mut f32) {
    *unit_gen_timer -= get_frame_time();

    let new_gen_time = if player.bits as f32 >= 200. {
        player.bits as f32 / 100.
    } else {
        2.
    };

    // TODO: Maybe move unit_gen_timer to actual Stronghold Tiles
    if *unit_gen_timer <= 0. {
        player.units.agents.push(Agent::new(vec2(5., 5.)));
        *unit_gen_timer = new_gen_time;
    }
}

// TODO: Make commands actually affect the game world
fn parse_commands(input: String) -> Result<Vec<String>, String> {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let command = split_input[0];

    let mut args = vec![];
    let mut output = vec![];

    if split_input.len() > 1 {
        args = split_input[1..].to_vec();
    }

    match command {
        "route" => {
            output.push(">> Routing".to_owned());

            if args.len() <= 1 {
                output.push("Not enought arguments!".to_owned());
            } else {
                output.push(format!("{args:?}"));
            }

            return Ok(output);
        }

        "bits" => {
            output.push(">> Moar bits!".to_owned());
            if !args.is_empty() {
                match args[0].parse::<i32>() {
                    Ok(n) => {
                        output.push(format!("Moar bits! (+ {})", n))
                        // player.bits += n;
                    }
                    Err(e) => output.push(e.to_string()),
                }
                return Ok(output);
            } else {
                return Err("What?!".to_owned());
            }
        }

        _ => return Err("No such command!".to_owned()),
    }
}
