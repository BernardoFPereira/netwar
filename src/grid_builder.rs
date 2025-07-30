use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub position: Vec2,
    pub center: Vec2,
    pub coord: Vec2,
    pub kind: TileKind,
    pub is_mouse_over: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TileKind {
    Blank,
    Mine,
    Stronghold,
}

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

    pub fn create_mines(&mut self, amount: i32) {
        // TODO
        for _ in 0..amount {
            // vec2(x, y) of the tile to become a Mine
            let mine_pos = vec2(
                rand::gen_range(2., self.dimension.x - 2.),
                rand::gen_range(1., self.dimension.y - 1.),
            );

            self.tiles.iter_mut().for_each(|t| {
                if t.coord == mine_pos {
                    t.kind = TileKind::Mine;
                }
            });
        }
        // self.tiles.iter_mut().for_each(|t| {
        //     if (t.coord.x > 1. && t.coord.x < 10.) && (t.coord.y > 0. && t.coord.y < 5.) {
        //         t.kind = TileKind::Mine
        //     } else {
        //         t.kind = TileKind::Blank
        //     }
        // });
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
            // draw_text(
            //     &format!(
            //         "{},{}",
            //         (tile.position.x + self.tile_size / 2.) as i32,
            //         (tile.position.y + self.tile_size / 2.) as i32
            //     ),
            //     tile.position.x + self.tile_size / 4.,
            //     tile.position.y + self.tile_size - self.tile_size / 6.,
            //     10.,
            //     LIGHTGRAY,
            // );
        }
    }
}
