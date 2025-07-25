use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self, Group},
    },
};

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
            tiles: vec![],
        }
    }

    fn draw_grid(&self) {
        let start_pos = Vec2::new(screen_width() / 5., screen_height() / 6.);

        let hex_width = self.hex_size * 1.5; // Horizontal distance between centers
        let hex_height = self.hex_size * 3_f32.sqrt() / 2.; // Vertical distance between centers

        for j in 0..self.width {
            for i in 0..self.height {
                let x = start_pos.x
                    + i as f32 * hex_width * 2.
                    + if j % 2 == 1 { hex_width } else { 0. };
                let y = start_pos.y + j as f32 * hex_height;

                draw_hexagon(x, y, self.hex_size, 1., false, GREEN, BLACK);
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
            for cmd in &mut cmd_history {
                ui.label(None, &format!("{cmd}"));
            }
            ui.input_text(hash!(), "", &mut command);
        });

        if is_key_pressed(KeyCode::Enter) {
            if command != "" {
                parse_commands(command.clone());
                cmd_history.push(command.clone());
            }
            command = "".to_owned();
        }

        next_frame().await
    }
}

fn parse_commands(input: String) {
    let command: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", command);
}
