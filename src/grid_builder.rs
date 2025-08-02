use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    pub position: Vec2,
    pub center: Vec2,
    pub coord: Vec2,
    pub kind: TileKind,
    pub designation: String,
    pub is_mouse_over: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Blank,
    Mine,
    Stronghold,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub dimension: Vec2,
    pub tile_size: f32,
    pub tiles: Vec<Tile>,
}
impl Grid {
    pub fn new(width: f32, height: f32, tile_size: f32) -> Self {
        Self {
            dimension: vec2(width, height),
            tile_size,
            tiles: vec![],
        }
    }

    pub fn fill_grid(&mut self) {
        let start_pos = Vec2::new(screen_width() / 5., screen_height() / 6.);

        for j in 0..self.dimension.y as i32 {
            for i in 0..self.dimension.x as i32 {
                let coord = vec2(i as f32, j as f32);

                let x = start_pos.x + coord.x * self.tile_size;
                let y = start_pos.y + coord.y * self.tile_size;

                self.tiles.push(Tile {
                    position: vec2(x, y),
                    center: vec2(x + self.tile_size / 2., y + self.tile_size / 2.),
                    coord,
                    kind: TileKind::Blank,
                    designation: "".to_owned(),
                    is_mouse_over: false,
                });
            }
        }
    }

    pub fn idx_to_coord(&self, idx: i32) -> Vec2 {
        let x = idx % self.dimension.x as i32;
        let y = idx * self.dimension.y as i32;

        vec2(x as f32, y as f32)
    }

    pub fn coord_to_idx(&self, x: f32, y: f32) -> i32 {
        ((y * self.dimension.x) + x) as i32
    }

    pub fn create_strongholds(&mut self) {
        // TODO
        rand::srand(miniquad::date::now() as u64);
        // for _ in 0..amount {
        let rng_idx = (rand::gen_range(0, self.tiles.len())) as usize;
        println!("{}", rng_idx);
        self.tiles[rng_idx].kind = TileKind::Stronghold;
        self.tiles[rng_idx].designation = "S1".to_owned();
        // }
    }

    pub fn create_mines(&mut self, amount: i32) {
        // TODO: continue this
        rand::srand(miniquad::date::now() as u64);
        for i in 0..amount {
            let rng_idx = (rand::gen_range(0, self.tiles.len())) as usize;
            println!("{}", rng_idx);
            self.tiles[rng_idx].kind = TileKind::Mine;
            self.tiles[rng_idx].designation = format!("M{}", i);
        }
    }

    pub fn draw_grid(&self) {
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
                TileKind::Stronghold => {
                    draw_rectangle(
                        tile.position.x,
                        tile.position.y,
                        self.tile_size,
                        self.tile_size,
                        BLUE,
                    );
                }
            }

            draw_rectangle(tile.center.x - 2., tile.center.y - 2., 4., 4., DARKPURPLE);

            // Draw Coordinates and Position vectos
            draw_text(
                &format!("{},{}", tile.coord.x, tile.coord.y),
                tile.position.x + self.tile_size / 4.,
                tile.position.y + self.tile_size / 3.,
                15.,
                WHITE,
            );
            draw_text(
                &format!("{}", tile.designation),
                tile.position.x + self.tile_size / 4.,
                tile.position.y + self.tile_size - self.tile_size / 6.,
                30.,
                LIGHTGRAY,
            );
        }
    }
}
