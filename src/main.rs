use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};

#[derive(Debug)]
struct Tile {
    coord: Vec2,
    color: Color,
    is_mouse_over: bool,
}

struct Grid {
    width: u32,
    height: u32,
    hex_size: f32,
    tiles: Vec<Tile>,
}
impl Grid {
    fn new(width: u32, height: u32, hex_size: f32) -> Self {
        Self {
            width,
            height,
            hex_size,
            tiles: Self::fill_grid(width, height),
        }
    }

    fn fill_grid(width: u32, height: u32) -> Vec<Tile> {
        let mut result: Vec<Tile> = vec![];

        for j in 0..width {
            for i in 0..height {
                result.push(Tile {
                    coord: vec2(i as f32, j as f32),
                    color: BLACK,
                    is_mouse_over: false,
                });
            }
        }

        result
    }

    fn draw_grid(&self) {
        let start_pos = Vec2::new(screen_width() / 5., screen_height() / 6.);

        // Horizontal distance between centers
        let hex_width = self.hex_size * 1.5;
        // Vertical distance between centers
        let hex_height = self.hex_size * 3_f32.sqrt() / 2.;

        for j in 0..self.height {
            for i in 0..self.width {
                let x = start_pos.x + i as f32 * hex_width;
                let y = start_pos.y
                    + j as f32 * hex_height * 2.
                    + if i % 2 != 0 { hex_height } else { 0. };

                draw_hexagon(x, y, self.hex_size, 1., false, GREEN, BLACK);
                draw_text(&format!("{},{}", i, j), x - 10., y + 5., 20., WHITE);
            }
        }
    }
}

#[macroquad::main("HexaGrid")]
async fn main() {
    let grid = Grid::new(12, 6, 30.);
    let mut command = String::new();
    let mut cmd_history: Vec<String> = vec![];

    loop {
        clear_background(BLACK);
        grid.draw_grid();

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
