#![allow(dead_code)]

mod grid_builder;

mod prelude {
    pub use crate::grid_builder::*;
    pub use macroquad::{
        prelude::*,
        ui::{hash, root_ui, widgets},
    };
}

use crate::prelude::*;

#[macroquad::main("NetWar")]
async fn main() {
    let mut grid = Grid::new(12., 6., 45.);
    grid.fill_grid();
    grid.create_mines(4);

    let mut command = String::new();
    let mut cmd_history: Vec<String> = vec![];

    loop {
        clear_background(BLACK);
        grid.draw_grid();

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
            ui.label(None, "Bits:");
            ui.label(None, "Units:");
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

        next_frame().await
    }
}

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

        _ => return Err("No such command!".to_owned()),
    }
}
