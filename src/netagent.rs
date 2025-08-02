use super::MatchState;
use crate::prelude::*;

// Idea for the future: Add different agent types with different behaviour.
// i.e. a type that doesn't flee from more than 1 enemy and instead seeks
// aggressively

pub struct Agent {
    position: Vec2,
    velocity: Vec2,
    target: Option<Vec2>,
}
impl Agent {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: vec2(0., 0.),
            target: None,
        }
    }
}

pub struct Units {
    pub agents: Vec<Agent>,
}
impl Units {
    pub fn new() -> Self {
        Self { agents: vec![] }
    }

    fn spawn_unit(&mut self, position: Vec2) {
        self.agents.push(Agent::new(position));
    }

    pub fn draw_units(&self) {
        for agent in &self.agents {
            let tri_size = 20.;
            let offset = vec2(tri_size, tri_size);

            draw_triangle(
                agent.position - offset,
                agent.position + vec2(offset.x, -offset.y),
                agent.position + vec2(0., offset.y),
                GREEN,
            );
        }
    }
}

pub fn unit_generation(ms: &mut MatchState) {
    ms.player.gen_time -= get_frame_time();

    let new_gen_time = if ms.player.bits as f32 >= 200. {
        ms.player.bits as f32 / 100.
    } else {
        2.
    };

    // TODO: Maybe move unit_gen_timer to actual Stronghold Tiles
    if ms.player.gen_time <= 0. {
        let mut spawn_pos = vec2(0., 0.);
        // Find Stronghold tile position
        ms.grid
            .tiles
            .iter()
            .filter(|t| t.kind == TileKind::Stronghold)
            .for_each(|s| {
                spawn_pos = s.center;
            });
        ms.player.units.agents.push(Agent::new(spawn_pos));
        ms.player.gen_time = new_gen_time;
    }
}
