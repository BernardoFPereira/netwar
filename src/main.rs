#![allow(dead_code)]
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};

#[derive(Debug, Clone, Copy)]
struct Tile {
    position: Vec2,
    center: Vec2,
    coord: Vec2,
    kind: TileKind,
    is_mouse_over: bool,
}

#[derive(Debug, Clone, Copy)]
enum TileKind {
    Blank,
    Mine,
    Spawn,
}

struct Grid {
    dimension: Vec2,
    tile_size: f32,
    tiles: Vec<Tile>,
}
impl Grid {
    fn new(width: f32, height: f32, tile_size: f32) -> Self {
        Self {
            dimension: vec2(width, height),
            tile_size,
            tiles: vec![],
        }
    }

    fn fill_grid(&mut self) {
        let start_pos = Vec2::new(screen_width() / 5., screen_height() / 6.);

        for i in 0..self.dimension.x as i32 {
            for j in 0..self.dimension.y as i32 {
                let coord = vec2(i as f32, j as f32);

                let x = start_pos.x + coord.x * self.tile_size;
                let y = start_pos.y + coord.y * self.tile_size;

                self.tiles.push(Tile {
                    position: vec2(x, y),
                    center: vec2(x + self.tile_size / 2., y + self.tile_size / 2.),
                    coord,
                    kind: TileKind::Blank,
                    is_mouse_over: false,
                });
            }
        }
    }

    fn create_spawns() {
        // TODO
    }

    fn create_mines() {
        // TODO
    }

    fn draw_grid(&self) {
        for tile in self.tiles.clone() {
            match tile.kind {
                TileKind::Blank => {
                    draw_rectangle_lines(
                        tile.position.x,
                        tile.position.y,
                        self.tile_size,
                        self.tile_size,
                        1.,
                        GREEN,
                    );
                }
                TileKind::Mine => {
                    draw_rectangle(
                        tile.position.x,
                        tile.position.y,
                        self.tile_size,
                        self.tile_size,
                        YELLOW,
                    );
                }
                TileKind::Spawn => {
                    draw_rectangle(
                        tile.position.x,
                        tile.position.y,
                        self.tile_size,
                        self.tile_size,
                        BLUE,
                    );
                }
            }

            draw_rectangle(tile.center.x, tile.center.y, 5., 5., RED);

            // Draw Coordinates and Position vectos
            draw_text(
                &format!("{},{}", tile.coord.x, tile.coord.y),
                tile.position.x + self.tile_size / 4.,
                tile.position.y + self.tile_size / 3.,
                15.,
                WHITE,
            );
            draw_text(
                &format!(
                    "{},{}",
                    (tile.position.x + self.tile_size / 2.) as i32,
                    (tile.position.y + self.tile_size / 2.) as i32
                ),
                tile.position.x + self.tile_size / 4.,
                tile.position.y + self.tile_size - self.tile_size / 6.,
                10.,
                LIGHTGRAY,
            );
        }
    }
}

#[macroquad::main("NetWar")]
async fn main() {
    let mut grid = Grid::new(12., 6., 45.);
    grid.fill_grid();

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
